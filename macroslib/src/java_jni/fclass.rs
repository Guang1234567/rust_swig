use log::{debug, trace};
use petgraph::Direction;
use proc_macro2::TokenStream;
use quote::quote;
use rustc_hash::{FxHashMap, FxHashSet};
use smol_str::SmolStr;
use std::io::Write;
use syn::{spanned::Spanned, Type};

use super::{
    calc_this_type_for_method, java_class_full_name, java_class_name_to_jni, java_code,
    map_type::map_type, map_write_err, method_name, rust_code, JavaContext, JavaConverter,
    JavaForeignTypeInfo, JniForeignMethodSignature, INTERNAL_PTR_MARKER, JAVA_RUST_SELF_NAME,
};
use crate::{
    error::{panic_on_syn_error, DiagnosticError, Result},
    file_cache::FileWriteCache,
    namegen::new_unique_name,
    typemap::{
        ast::{if_result_return_ok_err_types, list_lifetimes, normalize_ty_lifetimes},
        ty::{normalized_type, RustType},
        utils::{
            convert_to_heap_pointer, create_suitable_types_for_constructor_and_self,
            foreign_from_rust_convert_method_output, foreign_to_rust_convert_method_inputs,
            unpack_from_heap_pointer,
        },
        ForeignTypeInfo, FROM_VAR_TEMPLATE, TO_VAR_TEMPLATE, TO_VAR_TYPE_TEMPLATE,
    },
    types::{ForeignerClassInfo, ForeignerMethod, MethodAccess, MethodVariant, SelfTypeVariant},
    WRITE_TO_MEM_FAILED_MSG,
};

pub(in crate::java_jni) fn generate(
    ctx: &mut JavaContext,
    class: &ForeignerClassInfo,
) -> Result<()> {
    debug!(
        "generate: begin for {}, this_type_for_method {:?}",
        class.name, class.self_desc
    );

    let f_methods_sign = find_suitable_foreign_types_for_methods(ctx, class)?;
    generate_java_code(
        ctx,
        class,
        &f_methods_sign,
        ctx.cfg.null_annotation_package.as_ref().map(String::as_str),
    )
    .map_err(|err| DiagnosticError::new(class.src_id, class.span(), err))?;
    debug!("generate: java code done");
    generate_rust_code(ctx, class, &f_methods_sign)?;

    Ok(())
}

struct MethodContext<'a> {
    class: &'a ForeignerClassInfo,
    method: &'a ForeignerMethod,
    f_method: &'a JniForeignMethodSignature,
    jni_func_name: &'a str,
    decl_func_args: &'a str,
    real_output_typename: &'a str,
    ret_name: &'a str,
}

fn generate_java_code(
    ctx: &mut JavaContext,
    class: &ForeignerClassInfo,
    methods_sign: &[JniForeignMethodSignature],
    null_annotation_package: Option<&str>,
) -> std::result::Result<(), String> {
    let path = ctx.cfg.output_dir.join(format!("{}.java", class.name));
    let mut file = FileWriteCache::new(&path, ctx.generated_foreign_files);

    let imports = java_code::get_null_annotation_imports(null_annotation_package, methods_sign);

    let class_doc_comments = java_code::doc_comments_to_java_comments(&class.doc_comments, true);
    writeln!(
        file,
        r#"// Automatically generated by rust_swig
package {package_name};
{imports}
{doc_comments}
public final class {class_name} {{"#,
        package_name = ctx.cfg.package_name,
        imports = imports,
        class_name = class.name,
        doc_comments = class_doc_comments,
    )
    .expect(WRITE_TO_MEM_FAILED_MSG);

    let mut have_methods = false;
    let mut have_constructor = false;

    for (method, f_method) in class.methods.iter().zip(methods_sign) {
        write!(
            &mut file,
            "{doc_comments}",
            doc_comments = java_code::doc_comments_to_java_comments(&method.doc_comments, false)
        )
        .expect(WRITE_TO_MEM_FAILED_MSG);

        let may_return_error = match method.fn_decl.output {
            syn::ReturnType::Default => false,
            syn::ReturnType::Type(_, ref ptype) => {
                let ret_rust_ty = ctx.conv_map.find_or_alloc_rust_type(ptype, class.src_id);
                if_result_return_ok_err_types(&ret_rust_ty).is_some()
            }
        };

        let exception_spec = if may_return_error {
            " throws Exception"
        } else {
            ""
        };

        let method_access = match method.access {
            MethodAccess::Private => "private",
            MethodAccess::Public => "public",
            MethodAccess::Protected => unreachable!(),
        };
        let conv_code_flags = match method.variant {
            MethodVariant::StaticMethod => java_code::ArgsFormatFlags::INTERNAL,
            MethodVariant::Method(_) => {
                java_code::ArgsFormatFlags::COMMA_BEFORE | java_code::ArgsFormatFlags::INTERNAL
            }
            MethodVariant::Constructor => java_code::ArgsFormatFlags::INTERNAL,
        };
        let mut known_names: FxHashSet<SmolStr> =
            method.arg_names_without_self().map(|x| x.into()).collect();
        if let MethodVariant::Method(_) = method.variant {
            if known_names.contains(JAVA_RUST_SELF_NAME) {
                return Err(format!("In method {} there is argument with name {}, this name reserved for generated code",
                                   method.short_name(), JAVA_RUST_SELF_NAME));
            }
            known_names.insert(JAVA_RUST_SELF_NAME.into());
        }
        let ret_name = new_unique_name(&known_names, "ret");
        known_names.insert(ret_name.clone());
        let conv_ret = new_unique_name(&known_names, "convRet");
        known_names.insert(conv_ret.clone());

        let (convert_code, args_for_call_internal) = convert_code_for_method(
            f_method,
            method.arg_names_without_self(),
            known_names,
            conv_code_flags,
        );
        let func_name = method_name(method, f_method);

        let external_args_except_self = java_code::args_with_java_types(
            f_method,
            method.arg_names_without_self(),
            java_code::ArgsFormatFlags::EXTERNAL,
            null_annotation_package.is_some(),
        );
        fn calc_output_conv<'a>(
            output: &'a JavaForeignTypeInfo,
            conv: &'a JavaConverter,
            ret_name: &str,
            conv_ret: &str,
        ) -> (&'a str, &'a str, String) {
            let ret_type = output.base.name.as_str();
            let intermidiate_ret_type = conv.java_transition_type.as_str();
            let conv_code = conv
                .converter
                .replace(FROM_VAR_TEMPLATE, ret_name)
                .replace(TO_VAR_TYPE_TEMPLATE, &format!("{} {}", ret_type, conv_ret))
                .replace(TO_VAR_TEMPLATE, &conv_ret);
            let conv_code = java_code::filter_null_annotation(&conv_code).trim().into();
            (ret_type, intermidiate_ret_type, conv_code)
        }

        let (ret_type, intermidiate_ret_type, ret_conv_code) = match method.variant {
            MethodVariant::StaticMethod => {
                if let Some(conv) = f_method.output.java_converter.as_ref() {
                    calc_output_conv(&f_method.output, conv, &ret_name, &conv_ret)
                } else {
                    let ret_type = f_method.output.base.name.as_str();
                    (ret_type, ret_type, String::new())
                }
            }
            MethodVariant::Method(_) => {
                if let Some(conv) = f_method.output.java_converter.as_ref() {
                    calc_output_conv(&f_method.output, conv, &ret_name, &conv_ret)
                } else {
                    let ret_type = f_method.output.base.name.as_str();
                    (ret_type, ret_type, String::new())
                }
            }
            MethodVariant::Constructor => ("long", "long", String::new()),
        };

        let need_conversation = !convert_code.is_empty() || !ret_conv_code.is_empty();

        match method.variant {
            MethodVariant::StaticMethod => {
                let (native, end) = if !need_conversation {
                    ("native ", ";\n")
                } else {
                    ("", " {\n")
                };
                write!(
                    file,
                    r#"
    {method_access} static {native}{ret_type} {func_name}({args_with_types}){exception_spec}{end}"#,
                    method_access = method_access,
                    ret_type = ret_type,
                    func_name = method.short_name(),
                    args_with_types = external_args_except_self,
                    exception_spec = exception_spec,
                    native = native,
                    end = end,
                )
                .expect(WRITE_TO_MEM_FAILED_MSG);

                if need_conversation {
                    if !convert_code.is_empty() {
                        let mut code = convert_code.as_bytes();
                        if code[0] == b'\n' {
                            code = &code[1..];
                        }
                        file.write_all(code).expect(WRITE_TO_MEM_FAILED_MSG);
                        file.write_all(b"\n").expect(WRITE_TO_MEM_FAILED_MSG);
                    }
                    if ret_conv_code.is_empty() {
                        write!(
                            file,
                            r#"        {return_code}{func_name}({args});
    }}"#,
                            func_name = func_name,
                            return_code = if ret_type != "void" { "return " } else { "" },
                            args = args_for_call_internal,
                        )
                        .expect(WRITE_TO_MEM_FAILED_MSG);
                    } else {
                        writeln!(
                            file,
                            r#"        {intermidiate_ret_type} {ret_name} = {func_name}({args});
        {ret_conv_code}"#,
                            ret_conv_code = ret_conv_code,
                            ret_name = ret_name,
                            intermidiate_ret_type =
                                java_code::filter_null_annotation(intermidiate_ret_type).trim(),
                            func_name = func_name,
                            args = args_for_call_internal,
                        )
                        .expect(WRITE_TO_MEM_FAILED_MSG);
                        if ret_type != "void" {
                            writeln!(
                                file,
                                r#"
        return {conv_ret};"#,
                                conv_ret = conv_ret
                            )
                            .expect(WRITE_TO_MEM_FAILED_MSG);
                        }
                        file.write_all(b"    }").expect(WRITE_TO_MEM_FAILED_MSG);
                    }

                    write!(
                        file,
                        r#"
    private static native {intermidiate_ret_type} {func_name}({args_with_types}){exception_spec};
"#,
                        func_name = func_name,
                        intermidiate_ret_type = intermidiate_ret_type,
                        exception_spec = exception_spec,
                        args_with_types = java_code::args_with_java_types(
                            f_method,
                            method.arg_names_without_self(),
                            java_code::ArgsFormatFlags::INTERNAL,
                            null_annotation_package.is_some()
                        ),
                    )
                    .expect(WRITE_TO_MEM_FAILED_MSG);
                }
            }
            MethodVariant::Method(_) => {
                have_methods = true;
                write!(
                    file,
                    r#"
    {method_access} final {ret_type} {method_name}({single_args_with_types}){exception_spec} {{"#,
                    method_access = method_access,
                    ret_type = ret_type,
                    method_name = method.short_name(),
                    exception_spec = exception_spec,
                    single_args_with_types = external_args_except_self,
                )
                .expect(WRITE_TO_MEM_FAILED_MSG);
                if !convert_code.is_empty() {
                    if convert_code.as_bytes()[0] != b'\n' {
                        file.write_all(b"\n").expect(WRITE_TO_MEM_FAILED_MSG);
                    }
                    file.write_all(convert_code.as_bytes())
                        .expect(WRITE_TO_MEM_FAILED_MSG);
                }
                if ret_conv_code.is_empty() {
                    writeln!(
                        file,
                        r#"
        {return_code}{func_name}({rust_self_name}{args});"#,
                        rust_self_name = JAVA_RUST_SELF_NAME,
                        return_code = if ret_type != "void" { "return " } else { "" },
                        args = args_for_call_internal,
                        func_name = func_name,
                    )
                    .expect(WRITE_TO_MEM_FAILED_MSG);
                } else {
                    writeln!(
                        file,
                        r#"
        {intermidiate_ret_type} {ret_name} = {func_name}({rust_self_name}{args});
        {ret_conv_code}"#,
                        rust_self_name = JAVA_RUST_SELF_NAME,
                        ret_conv_code = ret_conv_code,
                        ret_name = ret_name,
                        intermidiate_ret_type =
                            java_code::filter_null_annotation(intermidiate_ret_type).trim(),
                        func_name = func_name,
                        args = args_for_call_internal,
                    )
                    .expect(WRITE_TO_MEM_FAILED_MSG);
                    if ret_type != "void" {
                        writeln!(
                            file,
                            r#"
        return {conv_ret};"#,
                            conv_ret = conv_ret
                        )
                        .expect(WRITE_TO_MEM_FAILED_MSG);
                    }
                }

                file.write_all(b"    }").expect(WRITE_TO_MEM_FAILED_MSG);

                writeln!(
                    file,
                    r#"
    private static native {intermidiate_ret_type} {func_name}(long self{args_with_types}){exception_spec};"#,
                    intermidiate_ret_type = intermidiate_ret_type,
                    exception_spec = exception_spec,
                    func_name = func_name,
                    args_with_types = java_code::args_with_java_types(
                        f_method,
                        method.arg_names_without_self(),
                        java_code::ArgsFormatFlags::USE_COMMA_IF_NEED | java_code::ArgsFormatFlags::INTERNAL,
                        null_annotation_package.is_some()
                    ),
                )
                .expect(WRITE_TO_MEM_FAILED_MSG);
            }
            MethodVariant::Constructor => {
                have_constructor = true;

                if method.is_dummy_constructor() {
                    writeln!(
                        file,
                        r#"
    {method_access} {class_name}() {{}}"#,
                        method_access = method_access,
                        class_name = class.name,
                    )
                    .expect(WRITE_TO_MEM_FAILED_MSG);
                } else {
                    writeln!(
                        file,
                        r#"
    {method_access} {class_name}({ext_args_with_types}){exception_spec} {{"#,
                        method_access = method_access,
                        class_name = class.name,
                        exception_spec = exception_spec,
                        ext_args_with_types = external_args_except_self,
                    )
                    .expect(WRITE_TO_MEM_FAILED_MSG);
                    if !convert_code.is_empty() {
                        let mut code = convert_code.as_bytes();
                        if code[0] == b'\n' {
                            code = &code[1..];
                        }
                        file.write_all(code).expect(WRITE_TO_MEM_FAILED_MSG);
                        file.write_all(b"\n").expect(WRITE_TO_MEM_FAILED_MSG);
                    }
                    writeln!(
                        file,
                        r#"        {rust_self_name} = init({args});
    }}
    private static native long {func_name}({args_with_types}){exception_spec};"#,
                        rust_self_name = JAVA_RUST_SELF_NAME,
                        exception_spec = exception_spec,
                        func_name = func_name,
                        args_with_types = java_code::args_with_java_types(
                            f_method,
                            method.arg_names_without_self(),
                            java_code::ArgsFormatFlags::INTERNAL,
                            null_annotation_package.is_some()
                        ),
                        args = args_for_call_internal,
                    )
                    .expect(WRITE_TO_MEM_FAILED_MSG);
                }
            }
        }
    }

    if have_methods && !have_constructor {
        return Err(format!(
            "package {}, class {}: has methods, but no constructor\n
May be you need to use `private constructor = empty;` syntax?",
            ctx.cfg.package_name, class.name
        ));
    }
    if have_constructor {
        writeln!(
            file,
            r#"
    public synchronized void delete() {{
        if ({rust_self_name} != 0) {{
            do_delete({rust_self_name});
            {rust_self_name} = 0;
       }}
    }}
    @Override
    protected void finalize() throws Throwable {{
        try {{
            delete();
        }}
        finally {{
             super.finalize();
        }}
    }}
    private static native void do_delete(long me);
    /*package*/ {class_name}({internal_ptr_marker} marker, long ptr) {{
        assert marker == {internal_ptr_marker}.RAW_PTR;
        this.{rust_self_name} = ptr;
    }}
    /*package*/ long {rust_self_name};"#,
            rust_self_name = JAVA_RUST_SELF_NAME,
            class_name = class.name,
            internal_ptr_marker = INTERNAL_PTR_MARKER,
        )
        .expect(WRITE_TO_MEM_FAILED_MSG);
    }

    //utility class, so add private constructor
    //to prevent object creation
    if !have_constructor && !have_methods {
        writeln!(
            file,
            r#"
    private {class_name}() {{}}"#,
            class_name = class.name
        )
        .map_err(&map_write_err)?;
    }

    file.write_all(class.foreigner_code.as_bytes())
        .expect(WRITE_TO_MEM_FAILED_MSG);
    write!(file, "}}").expect(WRITE_TO_MEM_FAILED_MSG);

    file.update_file_if_necessary().map_err(&map_write_err)?;
    Ok(())
}

fn generate_rust_code(
    ctx: &mut JavaContext,
    class: &ForeignerClassInfo,
    f_methods_sign: &[JniForeignMethodSignature],
) -> Result<()> {
    //to handle java method overload
    let mut gen_fnames = FxHashMap::<String, usize>::default();
    for (method, f_method) in class.methods.iter().zip(f_methods_sign.iter()) {
        let val_ref = gen_fnames.entry(method_name(method, f_method));
        *val_ref.or_insert(0) += 1;
    }

    let dummy_ty = parse_type! { () };
    let dummy_rust_ty = ctx.conv_map.find_or_alloc_rust_type_no_src_id(&dummy_ty);

    let (this_type_for_method, code_box_this) =
        if let Some(this_type) = calc_this_type_for_method(ctx.conv_map, class) {
            let this_type = ctx.conv_map.ty_to_rust_type(&this_type);
            debug!(
                "generate_rust_code: add implements SwigForeignClass for {}",
                this_type.normalized_name
            );

            let (this_type_for_method, code_box_this) =
                convert_to_heap_pointer(ctx.conv_map, &this_type, "this");
            let class_name_for_user =
                java_class_full_name(&ctx.cfg.package_name, &class.name.to_string());
            let class_name_for_jni = java_class_name_to_jni(&class_name_for_user);
            let lifetimes = list_lifetimes(&this_type.ty);
            let lifetimes = &lifetimes;

            let unpack_code = unpack_from_heap_pointer(&this_type, TO_VAR_TEMPLATE, true)
                .replace(TO_VAR_TEMPLATE, "x");
            let unpack_code: TokenStream = syn::parse_str(&unpack_code).unwrap_or_else(|err| {
                panic_on_syn_error("internal/java foreign class unpack code", unpack_code, err)
            });
            let this_type_for_method_ty = normalized_type(&this_type_for_method.normalized_name);
            let this_type_for_method_ty_as_is = &this_type_for_method.ty;
            let class_name = &this_type.ty;
            let fclass_impl_code = quote! {
                impl<#(#lifetimes),*> SwigForeignClass for #class_name {
                    type PointedType = #this_type_for_method_ty_as_is;

                    fn jni_class_name() -> *const ::std::os::raw::c_char {
                        swig_c_str!(#class_name_for_jni)
                    }
                    fn box_object(this: Self) -> jlong {
                        #code_box_this
                        this as jlong
                    }
                    fn unbox_object(x: jlong) -> Self {
                        let x: *mut #this_type_for_method_ty = unsafe {
                            jlong_to_pointer::<#this_type_for_method_ty>(x).as_mut().unwrap()
                        };
                        #unpack_code
                        x
                    }
                    fn to_pointer(x: jlong) -> ::std::ptr::NonNull<Self::PointedType> {
                        let x: *mut #this_type_for_method_ty = unsafe {
                            jlong_to_pointer::<#this_type_for_method_ty>(x).as_mut().unwrap()
                        };
                        ::std::ptr::NonNull::<Self::PointedType>::new(x).unwrap()
                    }
                }
            };
            ctx.rust_code.push(fclass_impl_code);
            (this_type_for_method, code_box_this)
        } else {
            (dummy_rust_ty.clone(), TokenStream::new())
        };

    let no_this_info = || {
        DiagnosticError::new(
            class.src_id,
            class.span(),
            format!(
                "Class {} has methods, but there is no constructor\n
May be you need to use `private constructor = empty;` syntax?",
                class.name,
            ),
        )
    };

    let mut have_constructor = false;

    for (method, f_method) in class.methods.iter().zip(f_methods_sign.iter()) {
        let java_method_name = method_name(method, f_method);
        let method_overloading = gen_fnames[&java_method_name] > 1;
        let jni_func_name = rust_code::generate_jni_func_name(
            &ctx.cfg.package_name,
            class,
            &java_method_name,
            method.variant,
            f_method,
            method_overloading,
        )?;
        trace!("generate_rust_code jni name: {}", jni_func_name);

        let mut known_names: FxHashSet<SmolStr> =
            method.arg_names_without_self().map(|x| x.into()).collect();
        if let MethodVariant::Method(_) = method.variant {
            if known_names.contains("this") {
                return Err(DiagnosticError::new(
                    class.src_id,
                    method.rust_id.span(),
                    "Invalid argument name 'this' reserved for generate code purposes",
                ));
            }
            known_names.insert("this".into());
        }
        let ret_name = new_unique_name(&known_names, "ret");
        known_names.insert(ret_name.clone());

        let decl_func_args = {
            use std::fmt::Write;
            let mut buf = String::new();
            for (f_type_info, arg_name) in
                f_method.input.iter().zip(method.arg_names_without_self())
            {
                write!(
                    &mut buf,
                    "{}: {}, ",
                    arg_name,
                    f_type_info.as_ref().correspoding_rust_type.typename()
                )
                .expect(WRITE_TO_MEM_FAILED_MSG);
            }
            buf
        };

        let real_output_typename = match method.fn_decl.output {
            syn::ReturnType::Default => "()",
            syn::ReturnType::Type(_, ref ty) => normalize_ty_lifetimes(&*ty),
        };

        let method_ctx = MethodContext {
            class,
            method,
            f_method,
            jni_func_name: &jni_func_name,
            decl_func_args: &decl_func_args,
            real_output_typename: &real_output_typename,
            ret_name: &ret_name,
        };

        match method.variant {
            MethodVariant::StaticMethod => {
                generate_static_method(ctx, &method_ctx)?;
            }
            MethodVariant::Method(ref self_variant) => {
                generate_method(ctx, &method_ctx, *self_variant, &this_type_for_method)?;
            }
            MethodVariant::Constructor => {
                have_constructor = true;
                if !method.is_dummy_constructor() {
                    let constructor_ret_type = class
                        .self_desc
                        .as_ref()
                        .map(|x| &x.constructor_ret_type)
                        .ok_or_else(&no_this_info)?
                        .clone();
                    let this_type =
                        calc_this_type_for_method(ctx.conv_map, class).ok_or_else(&no_this_info)?;
                    generate_constructor(
                        ctx,
                        &method_ctx,
                        constructor_ret_type,
                        this_type,
                        &code_box_this,
                    )?;
                }
            }
        }
    }

    if have_constructor {
        let this_type: RustType = ctx.conv_map.find_or_alloc_rust_type(
            &calc_this_type_for_method(ctx.conv_map, class).ok_or_else(&no_this_info)?,
            class.src_id,
        );
        let jlong_type = ctx.conv_map.ty_to_rust_type(&parse_type! { jlong });

        let unpack_code = unpack_from_heap_pointer(&this_type, "this", false);

        let jni_destructor_name = rust_code::generate_jni_func_name(
            &ctx.cfg.package_name,
            class,
            "do_delete",
            MethodVariant::StaticMethod,
            &JniForeignMethodSignature {
                output: ForeignTypeInfo {
                    name: "".into(),
                    correspoding_rust_type: dummy_rust_ty.clone(),
                }
                .into(),
                input: vec![JavaForeignTypeInfo {
                    base: ForeignTypeInfo {
                        name: "long".into(),
                        correspoding_rust_type: jlong_type,
                    },
                    java_converter: None,
                    annotation: None,
                }],
            },
            false,
        )?;
        let code = format!(
            r#"
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn {jni_destructor_name}(env: *mut JNIEnv, _: jclass, this: jlong) {{
    let this: *mut {this_type} = unsafe {{
        jlong_to_pointer::<{this_type}>(this).as_mut().unwrap()
    }};
{unpack_code}
    drop(this);
}}
"#,
            jni_destructor_name = jni_destructor_name,
            unpack_code = unpack_code,
            this_type = this_type_for_method.normalized_name,
        );
        debug!("we generate and parse code: {}", code);
        ctx.rust_code.push(
            syn::parse_str(&code).unwrap_or_else(|err| {
                panic_on_syn_error("java/jni internal desctructor", code, err)
            }),
        );
    }

    Ok(())
}

fn find_suitable_foreign_types_for_methods(
    ctx: &mut JavaContext,
    class: &ForeignerClassInfo,
) -> Result<Vec<JniForeignMethodSignature>> {
    let mut ret = Vec::<JniForeignMethodSignature>::with_capacity(class.methods.len());
    let empty_symbol = "";
    let dummy_ty = parse_type! { () };
    let dummy_rust_ty = ctx.conv_map.find_or_alloc_rust_type_no_src_id(&dummy_ty);

    for method in &class.methods {
        debug!(
            "find_suitable_foreign_types_for_methods: class {}, method {}",
            class.name,
            method.short_name()
        );
        //skip self argument
        let skip_n = match method.variant {
            MethodVariant::Method(_) => 1,
            _ => 0,
        };
        assert!(method.fn_decl.inputs.len() >= skip_n);
        let mut input =
            Vec::<JavaForeignTypeInfo>::with_capacity(method.fn_decl.inputs.len() - skip_n);
        for arg in method.fn_decl.inputs.iter().skip(skip_n) {
            let named_arg = arg
                .as_named_arg()
                .map_err(|err| DiagnosticError::from_syn_err(class.src_id, err))?;
            let arg_rust_ty = ctx
                .conv_map
                .find_or_alloc_rust_type(&named_arg.ty, class.src_id);

            let fti = map_type(
                ctx,
                &arg_rust_ty,
                Direction::Incoming,
                (class.src_id, named_arg.ty.span()),
            )?;
            input.push(fti);
        }
        let output = match method.variant {
            MethodVariant::Constructor => ForeignTypeInfo {
                name: empty_symbol.into(),
                correspoding_rust_type: dummy_rust_ty.clone(),
            }
            .into(),
            _ => match method.fn_decl.output {
                syn::ReturnType::Default => ForeignTypeInfo {
                    name: "void".into(),
                    correspoding_rust_type: dummy_rust_ty.clone(),
                }
                .into(),
                syn::ReturnType::Type(_, ref rt) => {
                    let ret_rust_ty = ctx.conv_map.find_or_alloc_rust_type(rt, class.src_id);
                    map_type(
                        ctx,
                        &ret_rust_ty,
                        Direction::Outgoing,
                        (class.src_id, rt.span()),
                    )?
                }
            },
        };
        ret.push(JniForeignMethodSignature { output, input });
    }
    Ok(ret)
}

fn generate_static_method(ctx: &mut JavaContext, mc: &MethodContext) -> Result<()> {
    let jni_ret_type = mc.f_method.output.base.correspoding_rust_type.typename();
    let (mut deps_code_out, convert_output_code) = foreign_from_rust_convert_method_output(
        ctx.conv_map,
        mc.class.src_id,
        &mc.method.fn_decl.output,
        &mc.f_method.output,
        mc.ret_name,
        &jni_ret_type,
    )?;
    ctx.rust_code.append(&mut deps_code_out);
    let (mut deps_code_in, convert_input_code) = foreign_to_rust_convert_method_inputs(
        ctx.conv_map,
        mc.class.src_id,
        mc.method,
        mc.f_method,
        mc.method.arg_names_without_self(),
        &jni_ret_type,
    )?;
    ctx.rust_code.append(&mut deps_code_in);

    let code = format!(
        r#"
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn {func_name}(env: *mut JNIEnv, _: jclass, {decl_func_args}) -> {jni_ret_type} {{
{convert_input_code}
    let mut {ret_name}: {real_output_typename} = {call};
{convert_output_code}
    {ret_name}
}}
"#,
        func_name = mc.jni_func_name,
        decl_func_args = mc.decl_func_args,
        jni_ret_type = jni_ret_type,
        convert_input_code = convert_input_code,
        convert_output_code = convert_output_code,
        real_output_typename = mc.real_output_typename,
        call = mc.method.generate_code_to_call_rust_func(),
        ret_name = mc.ret_name,
    );

    ctx.rust_code
        .push(syn::parse_str(&code).unwrap_or_else(|err| {
            panic_on_syn_error("java/jni internal static method", code, err)
        }));

    Ok(())
}

fn generate_constructor(
    ctx: &mut JavaContext,
    mc: &MethodContext,
    construct_ret_type: Type,
    this_type: Type,
    code_box_this: &TokenStream,
) -> Result<()> {
    let (mut deps_code_in, convert_input_code) = foreign_to_rust_convert_method_inputs(
        ctx.conv_map,
        mc.class.src_id,
        mc.method,
        mc.f_method,
        mc.method.arg_names_without_self(),
        "jlong",
    )?;
    ctx.rust_code.append(&mut deps_code_in);
    let this_type = ctx.conv_map.ty_to_rust_type(&this_type);
    let construct_ret_type = ctx.conv_map.ty_to_rust_type(&construct_ret_type);

    let (mut deps_this, convert_this) = ctx.conv_map.convert_rust_types(
        construct_ret_type.to_idx(),
        this_type.to_idx(),
        "this",
        "this",
        "jlong",
        (mc.class.src_id, mc.method.span()),
    )?;
    ctx.rust_code.append(&mut deps_this);

    let code = format!(
        r#"
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn {func_name}(env: *mut JNIEnv, _: jclass, {decl_func_args}) -> jlong {{
{convert_input_code}
    let this: {real_output_typename} = {call};
{convert_this}
{box_this}
    this as jlong
}}
"#,
        func_name = mc.jni_func_name,
        convert_this = convert_this,
        decl_func_args = mc.decl_func_args,
        convert_input_code = convert_input_code,
        box_this = code_box_this,
        real_output_typename = mc.real_output_typename,
        call = mc.method.generate_code_to_call_rust_func(),
    );

    ctx.rust_code.push(
        syn::parse_str(&code)
            .unwrap_or_else(|err| panic_on_syn_error("java/jni internal constructor", code, err)),
    );

    Ok(())
}

fn generate_method(
    ctx: &mut JavaContext,
    mc: &MethodContext,
    self_variant: SelfTypeVariant,
    this_type_for_method: &RustType,
) -> Result<()> {
    let jni_ret_type = mc.f_method.output.base.correspoding_rust_type.typename();
    let (mut deps_code_in, convert_input_code) = foreign_to_rust_convert_method_inputs(
        ctx.conv_map,
        mc.class.src_id,
        mc.method,
        mc.f_method,
        mc.method.arg_names_without_self(),
        &jni_ret_type,
    )?;
    ctx.rust_code.append(&mut deps_code_in);
    let (mut deps_code_out, convert_output_code) = foreign_from_rust_convert_method_output(
        ctx.conv_map,
        mc.class.src_id,
        &mc.method.fn_decl.output,
        &mc.f_method.output,
        mc.ret_name,
        &jni_ret_type,
    )?;
    ctx.rust_code.append(&mut deps_code_out);

    //&mut constructor_real_type -> &mut class.self_type

    let (from_ty, to_ty): (Type, Type) = create_suitable_types_for_constructor_and_self(
        self_variant,
        mc.class,
        &this_type_for_method.ty,
    );
    let from_ty = ctx
        .conv_map
        .find_or_alloc_rust_type(&from_ty, mc.class.src_id);
    let this_type_ref = from_ty.normalized_name.as_str();
    let to_ty = ctx
        .conv_map
        .find_or_alloc_rust_type(&to_ty, mc.class.src_id);

    let (mut deps_this, convert_this) = ctx.conv_map.convert_rust_types(
        from_ty.to_idx(),
        to_ty.to_idx(),
        "this",
        "this",
        jni_ret_type,
        (mc.class.src_id, mc.method.span()),
    )?;
    ctx.rust_code.append(&mut deps_this);

    let code = format!(
        r#"
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C"
 fn {func_name}(env: *mut JNIEnv, _: jclass, this: jlong, {decl_func_args}) -> {jni_ret_type} {{
{convert_input_code}
    let this: {this_type_ref} = unsafe {{
        jlong_to_pointer::<{this_type}>(this).as_mut().unwrap()
    }};
{convert_this}
    let mut {ret_name}: {real_output_typename} = {call};
{convert_output_code}
    {ret_name}
}}
"#,
        func_name = mc.jni_func_name,
        decl_func_args = mc.decl_func_args,
        convert_input_code = convert_input_code,
        jni_ret_type = jni_ret_type,
        this_type_ref = this_type_ref,
        this_type = this_type_for_method.normalized_name,
        convert_this = convert_this,
        convert_output_code = convert_output_code,
        real_output_typename = mc.real_output_typename,
        call = mc.method.generate_code_to_call_rust_func(),
        ret_name = mc.ret_name,
    );

    ctx.rust_code.push(
        syn::parse_str(&code)
            .unwrap_or_else(|err| panic_on_syn_error("java/jni internal method", code, err)),
    );
    Ok(())
}

fn convert_code_for_method<'a, NI: Iterator<Item = &'a str>>(
    f_method: &JniForeignMethodSignature,
    arg_name_iter: NI,
    mut known_names: FxHashSet<SmolStr>,
    flags: java_code::ArgsFormatFlags,
) -> (String, String) {
    let mut conv_code = String::new();
    let mut args_for_call_internal = String::new();

    if flags.contains(java_code::ArgsFormatFlags::COMMA_BEFORE) && !f_method.input.is_empty() {
        args_for_call_internal.push_str(", ");
    }

    for (i, (arg, arg_name)) in f_method.input.iter().zip(arg_name_iter).enumerate() {
        let after_conv_arg_name = if let Some(java_conv) = arg.java_converter.as_ref() {
            let templ = format!("a{}", i);
            let after_conv_arg_name = new_unique_name(&known_names, &templ);
            known_names.insert(after_conv_arg_name.clone());
            let java_code: String = java_conv
                .converter
                .replace(
                    TO_VAR_TYPE_TEMPLATE,
                    &format!("{} {}", java_conv.java_transition_type, after_conv_arg_name),
                )
                .replace(TO_VAR_TEMPLATE, &after_conv_arg_name)
                .replace(FROM_VAR_TEMPLATE, arg_name);
            let java_code = java_code::filter_null_annotation(&java_code);
            conv_code.push_str(&java_code);
            Some(after_conv_arg_name)
        } else {
            None
        };
        if let Some(after_conv_arg_name) = after_conv_arg_name {
            args_for_call_internal.push_str(&after_conv_arg_name);
        } else {
            args_for_call_internal.push_str(arg_name);
        }
        if i != (f_method.input.len() - 1) {
            args_for_call_internal.push_str(", ");
        }
    }
    (conv_code, args_for_call_internal)
}
