#[allow(dead_code)]
mod internal_aliases {
    use super::*;
    pub type JStringOptStr = jstring;
    pub type JOptionalInt = jobject;
    pub type JInteger = jobject;
    pub type JByte = jobject;
    pub type JShort = jobject;
    pub type JFloat = jobject;
    pub type JDouble = jobject;
    pub type JOptionalDouble = jobject;
    pub type JLong = jobject;
    pub type JOptionalLong = jobject;
    #[repr(transparent)]
    pub struct JForeignObjectsArray<T: SwigForeignClass> {
        pub(crate) inner: jobjectArray,
        pub(crate) _marker: ::std::marker::PhantomData<T>,
    }
    pub type JStringPath = jstring;
}
#[doc = " Default JNI_VERSION"]
const SWIG_JNI_VERSION: jint = JNI_VERSION_1_6 as jint;
#[doc = " Marker for what to cache in JNI_OnLoad"]
#[allow(unused_macros)]
macro_rules! swig_jni_find_class {
    ( $ id : ident , $ path : expr ) => {
        unsafe { $id }
    };
    ( $ id : ident , $ path : expr , ) => {
        unsafe { $id }
    };
}
#[allow(unused_macros)]
macro_rules! swig_jni_get_method_id {
    ( $ global_id : ident , $ class_id : ident , $ name : expr , $ sig : expr ) => {
        unsafe { $global_id }
    };
    ( $ global_id : ident , $ class_id : ident , $ name : expr , $ sig : expr , ) => {
        unsafe { $global_id }
    };
}
#[allow(unused_macros)]
macro_rules! swig_jni_get_static_method_id {
    ( $ global_id : ident , $ class_id : ident , $ name : expr , $ sig : expr ) => {
        unsafe { $global_id }
    };
    ( $ global_id : ident , $ class_id : ident , $ name : expr , $ sig : expr , ) => {
        unsafe { $global_id }
    };
}
#[allow(unused_macros)]
macro_rules! swig_jni_get_field_id {
    ( $ global_id : ident , $ class_id : ident , $ name : expr , $ sig : expr ) => {
        unsafe { $global_id }
    };
    ( $ global_id : ident , $ class_id : ident , $ name : expr , $ sig : expr , ) => {
        unsafe { $global_id }
    };
}
#[allow(unused_macros)]
macro_rules! swig_jni_get_static_field_id {
    ( $ global_id : ident , $ class_id : ident , $ name : expr , $ sig : expr ) => {
        unsafe { $global_id }
    };
    ( $ global_id : ident , $ class_id : ident , $ name : expr , $ sig : expr , ) => {
        unsafe { $global_id }
    };
}
#[allow(dead_code)]
#[doc = ""]
trait SwigInto<T> {
    fn swig_into(self, env: *mut JNIEnv) -> T;
}
#[allow(dead_code)]
#[doc = ""]
trait SwigFrom<T> {
    fn swig_from(_: T, env: *mut JNIEnv) -> Self;
}
#[allow(dead_code)]
#[doc = ""]
trait SwigDeref {
    type Target: ?Sized;
    fn swig_deref(&self) -> &Self::Target;
}
#[allow(dead_code)]
#[doc = ""]
trait SwigDerefMut {
    type Target: ?Sized;
    fn swig_deref_mut(&mut self) -> &mut Self::Target;
}
#[allow(unused_macros)]
macro_rules! swig_c_str {
    ( $ lit : expr ) => {
        concat!($lit, "\0").as_ptr() as *const ::std::os::raw::c_char
    };
}
#[allow(unused_macros)]
macro_rules ! swig_assert_eq_size { ( $ x : ty , $ ( $ xs : ty ) ,+ $ ( , ) * ) => { $ ( let _ = :: std :: mem :: transmute ::<$ x , $ xs >; ) + } ; }
#[cfg(target_pointer_width = "32")]
pub unsafe fn jlong_to_pointer<T>(val: jlong) -> *mut T {
    (val as u32) as *mut T
}
#[cfg(target_pointer_width = "64")]
pub unsafe fn jlong_to_pointer<T>(val: jlong) -> *mut T {
    val as *mut T
}
#[allow(dead_code)]
pub trait SwigForeignClass {
    type PointedType;
    fn jni_class() -> jclass;
    fn jni_class_pointer_field() -> jfieldID;
    fn box_object(x: Self) -> jlong;
    fn unbox_object(x: jlong) -> Self;
    fn to_pointer(x: jlong) -> ::std::ptr::NonNull<Self::PointedType>;
}
#[allow(dead_code)]
pub trait SwigForeignCLikeEnum {
    fn as_jint(&self) -> jint;
    #[doc = " # Panics"]
    #[doc = " Panics on error"]
    fn from_jint(_: jint) -> Self;
}
#[allow(dead_code)]
pub struct JavaString {
    string: jstring,
    chars: *const ::std::os::raw::c_char,
    env: *mut JNIEnv,
}
#[allow(dead_code)]
impl JavaString {
    pub fn new(env: *mut JNIEnv, js: jstring) -> JavaString {
        let chars = if !js.is_null() {
            unsafe { (**env).GetStringUTFChars.unwrap()(env, js, ::std::ptr::null_mut()) }
        } else {
            ::std::ptr::null_mut()
        };
        JavaString {
            string: js,
            chars: chars,
            env: env,
        }
    }
    pub fn to_str(&self) -> &str {
        if !self.chars.is_null() {
            let s = unsafe { ::std::ffi::CStr::from_ptr(self.chars) };
            s.to_str().unwrap()
        } else {
            ""
        }
    }
}
#[allow(dead_code)]
impl Drop for JavaString {
    fn drop(&mut self) {
        assert!(!self.env.is_null());
        if !self.string.is_null() {
            assert!(!self.chars.is_null());
            unsafe {
                (**self.env).ReleaseStringUTFChars.unwrap()(self.env, self.string, self.chars)
            };
            self.env = ::std::ptr::null_mut();
            self.chars = ::std::ptr::null_mut();
        }
    }
}
#[allow(dead_code)]
struct JavaCallback {
    java_vm: *mut JavaVM,
    this: jobject,
    methods: Vec<jmethodID>,
}
#[doc = " According to JNI spec it should be safe to"]
#[doc = " pass pointer to JavaVm and jobject (global) across threads"]
unsafe impl Send for JavaCallback {}
#[allow(dead_code)]
struct JniEnvHolder<'a> {
    env: Option<*mut JNIEnv>,
    callback: &'a JavaCallback,
    need_detach: bool,
}
#[allow(dead_code)]
impl<'a> Drop for JniEnvHolder<'a> {
    fn drop(&mut self) {
        if self.need_detach {
            let res = unsafe {
                (**self.callback.java_vm).DetachCurrentThread.unwrap()(self.callback.java_vm)
            };
            if res != 0 {
                log::error!("JniEnvHolder: DetachCurrentThread failed: {}", res);
            }
        }
    }
}
#[allow(dead_code)]
impl JavaCallback {
    fn new(obj: jobject, env: *mut JNIEnv) -> JavaCallback {
        let mut java_vm: *mut JavaVM = ::std::ptr::null_mut();
        let ret = unsafe { (**env).GetJavaVM.unwrap()(env, &mut java_vm) };
        assert_eq!(0, ret, "GetJavaVm failed");
        let global_obj = unsafe { (**env).NewGlobalRef.unwrap()(env, obj) };
        assert!(!global_obj.is_null());
        JavaCallback {
            java_vm,
            this: global_obj,
            methods: Vec::new(),
        }
    }
    fn get_jni_env(&self) -> JniEnvHolder {
        assert!(!self.java_vm.is_null());
        let mut env: *mut JNIEnv = ::std::ptr::null_mut();
        let res = unsafe {
            (**self.java_vm).GetEnv.unwrap()(
                self.java_vm,
                (&mut env) as *mut *mut JNIEnv as *mut *mut ::std::os::raw::c_void,
                SWIG_JNI_VERSION,
            )
        };
        if res == (JNI_OK as jint) {
            return JniEnvHolder {
                env: Some(env),
                callback: self,
                need_detach: false,
            };
        }
        if res != (JNI_EDETACHED as jint) {
            panic!("get_jni_env: GetEnv return error `{}`", res);
        }
        trait ConvertPtr<T> {
            fn convert_ptr(self) -> T;
        }
        impl ConvertPtr<*mut *mut ::std::os::raw::c_void> for *mut *mut JNIEnv {
            fn convert_ptr(self) -> *mut *mut ::std::os::raw::c_void {
                self as *mut *mut ::std::os::raw::c_void
            }
        }
        impl ConvertPtr<*mut *mut JNIEnv> for *mut *mut JNIEnv {
            fn convert_ptr(self) -> *mut *mut JNIEnv {
                self
            }
        }
        let res = unsafe {
            (**self.java_vm).AttachCurrentThread.unwrap()(
                self.java_vm,
                (&mut env as *mut *mut JNIEnv).convert_ptr(),
                ::std::ptr::null_mut(),
            )
        };
        if res != 0 {
            log::error!(
                "JavaCallback::get_jnienv: AttachCurrentThread failed: {}",
                res
            );
            JniEnvHolder {
                env: None,
                callback: self,
                need_detach: false,
            }
        } else {
            assert!(!env.is_null());
            JniEnvHolder {
                env: Some(env),
                callback: self,
                need_detach: true,
            }
        }
    }
}
#[allow(dead_code)]
impl Drop for JavaCallback {
    fn drop(&mut self) {
        let env = self.get_jni_env();
        if let Some(env) = env.env {
            assert!(!env.is_null());
            unsafe { (**env).DeleteGlobalRef.unwrap()(env, self.this) };
        } else {
            log::error!("JavaCallback::drop failed, can not get JNIEnv");
        }
    }
}
#[allow(dead_code)]
fn jni_throw(env: *mut JNIEnv, ex_class: jclass, message: &str) {
    let c_message = ::std::ffi::CString::new(message).unwrap();
    let res = unsafe { (**env).ThrowNew.unwrap()(env, ex_class, c_message.as_ptr()) };
    if res != 0 {
        log::error!(
            "JNI ThrowNew({}) failed for class {:?} failed",
            message,
            ex_class
        );
    }
}
#[allow(dead_code)]
fn jni_throw_exception(env: *mut JNIEnv, message: &str) {
    let exception_class = swig_jni_find_class!(JAVA_LANG_EXCEPTION, "java/lang/Exception");
    jni_throw(env, exception_class, message)
}
#[allow(dead_code)]
fn object_to_jobject<T: SwigForeignClass>(env: *mut JNIEnv, obj: T) -> jobject {
    let jcls = <T>::jni_class();
    assert!(!jcls.is_null());
    let field_id = <T>::jni_class_pointer_field();
    assert!(!field_id.is_null());
    let jobj: jobject = unsafe { (**env).AllocObject.unwrap()(env, jcls) };
    assert!(!jobj.is_null(), "object_to_jobject: AllocObject failed");
    let ret: jlong = <T>::box_object(obj);
    unsafe {
        (**env).SetLongField.unwrap()(env, jobj, field_id, ret);
        if (**env).ExceptionCheck.unwrap()(env) != 0 {
            panic!("object_to_jobject: Can not set mNativeObj field: catch exception");
        }
    }
    jobj
}
#[allow(dead_code)]
fn jobject_array_to_vec_of_objects<T: SwigForeignClass + Clone>(
    env: *mut JNIEnv,
    arr: internal_aliases::JForeignObjectsArray<T>,
) -> Vec<T> {
    let field_id = <T>::jni_class_pointer_field();
    assert!(!field_id.is_null());
    let length = unsafe { (**env).GetArrayLength.unwrap()(env, arr.inner) };
    let len = <usize as ::std::convert::TryFrom<jsize>>::try_from(length)
        .expect("invalid jsize, in jsize => usize conversation");
    let mut result = Vec::with_capacity(len);
    for i in 0..length {
        let native: &mut T = unsafe {
            let obj = (**env).GetObjectArrayElement.unwrap()(env, arr.inner, i);
            if (**env).ExceptionCheck.unwrap()(env) != 0 {
                panic!("Failed to retrieve element {} from this `jobjectArray'", i);
            }
            let ptr = (**env).GetLongField.unwrap()(env, obj, field_id);
            let native = (jlong_to_pointer(ptr) as *mut T).as_mut().unwrap();
            (**env).DeleteLocalRef.unwrap()(env, obj);
            native
        };
        result.push(native.clone());
    }
    result
}
#[allow(dead_code)]
fn vec_of_objects_to_jobject_array<T: SwigForeignClass>(
    env: *mut JNIEnv,
    mut arr: Vec<T>,
) -> internal_aliases::JForeignObjectsArray<T> {
    let jcls: jclass = <T>::jni_class();
    assert!(!jcls.is_null());
    let arr_len = <jsize as ::std::convert::TryFrom<usize>>::try_from(arr.len())
        .expect("invalid usize, in usize => to jsize conversation");
    let obj_arr: jobjectArray =
        unsafe { (**env).NewObjectArray.unwrap()(env, arr_len, jcls, ::std::ptr::null_mut()) };
    assert!(!obj_arr.is_null());
    let field_id = <T>::jni_class_pointer_field();
    assert!(!field_id.is_null());
    for (i, r_obj) in arr.drain(..).enumerate() {
        let jobj: jobject = unsafe { (**env).AllocObject.unwrap()(env, jcls) };
        assert!(!jobj.is_null());
        let r_obj: jlong = <T>::box_object(r_obj);
        unsafe {
            (**env).SetLongField.unwrap()(env, jobj, field_id, r_obj);
            if (**env).ExceptionCheck.unwrap()(env) != 0 {
                panic!("Can not mNativeObj field: catch exception");
            }
            (**env).SetObjectArrayElement.unwrap()(env, obj_arr, i as jsize, jobj);
            if (**env).ExceptionCheck.unwrap()(env) != 0 {
                panic!("SetObjectArrayElement({}) failed", i);
            }
            (**env).DeleteLocalRef.unwrap()(env, jobj);
        }
    }
    internal_aliases::JForeignObjectsArray {
        inner: obj_arr,
        _marker: ::std::marker::PhantomData,
    }
}
#[allow(dead_code)]
trait JniInvalidValue {
    fn jni_invalid_value() -> Self;
}
impl<T> JniInvalidValue for *const T {
    fn jni_invalid_value() -> Self {
        ::std::ptr::null()
    }
}
impl<T> JniInvalidValue for *mut T {
    fn jni_invalid_value() -> Self {
        ::std::ptr::null_mut()
    }
}
impl JniInvalidValue for () {
    fn jni_invalid_value() {}
}
impl<T: SwigForeignClass> JniInvalidValue for internal_aliases::JForeignObjectsArray<T> {
    fn jni_invalid_value() -> Self {
        Self {
            inner: ::std::ptr::null_mut(),
            _marker: ::std::marker::PhantomData,
        }
    }
}
macro_rules ! impl_jni_jni_invalid_value { ( $ ( $ type : ty ) * ) => ( $ ( impl JniInvalidValue for $ type { fn jni_invalid_value ( ) -> Self { <$ type >:: default ( ) } } ) * ) }
impl_jni_jni_invalid_value! { jbyte jshort jint jlong jfloat jdouble }
#[allow(dead_code)]
pub fn u64_to_jlong_checked(x: u64) -> jlong {
    <jlong as ::std::convert::TryFrom<u64>>::try_from(x)
        .expect("invalid u64, in u64 => jlong conversation")
}
#[allow(dead_code)]
fn from_std_string_jstring(x: String, env: *mut JNIEnv) -> jstring {
    let x = x.into_bytes();
    unsafe {
        let x = ::std::ffi::CString::from_vec_unchecked(x);
        (**env).NewStringUTF.unwrap()(env, x.as_ptr())
    }
}
macro_rules ! define_array_handling_code { ( $ ( [ jni_arr_type = $ jni_arr_type : ident , rust_arr_wrapper = $ rust_arr_wrapper : ident , jni_get_array_elements = $ jni_get_array_elements : ident , jni_elem_type = $ jni_elem_type : ident , rust_elem_type = $ rust_elem_type : ident , jni_release_array_elements = $ jni_release_array_elements : ident , jni_new_array = $ jni_new_array : ident , jni_set_array_region = $ jni_set_array_region : ident ] ) ,* ) => { $ ( # [ allow ( dead_code ) ] struct $ rust_arr_wrapper { array : $ jni_arr_type , data : * mut $ jni_elem_type , env : * mut JNIEnv , } # [ allow ( dead_code ) ] impl $ rust_arr_wrapper { fn new ( env : * mut JNIEnv , array : $ jni_arr_type ) -> $ rust_arr_wrapper { assert ! ( ! array . is_null ( ) ) ; let data = unsafe { ( ** env ) .$ jni_get_array_elements . unwrap ( ) ( env , array , :: std :: ptr :: null_mut ( ) ) } ; $ rust_arr_wrapper { array , data , env } } fn to_slice ( & self ) -> & [ $ rust_elem_type ] { unsafe { let len : jsize = ( ** self . env ) . GetArrayLength . unwrap ( ) ( self . env , self . array ) ; assert ! ( ( len as u64 ) <= ( usize :: max_value ( ) as u64 ) ) ; :: std :: slice :: from_raw_parts ( self . data , len as usize ) } } fn from_slice_to_raw ( arr : & [ $ rust_elem_type ] , env : * mut JNIEnv ) -> $ jni_arr_type { assert ! ( ( arr . len ( ) as u64 ) <= ( jsize :: max_value ( ) as u64 ) ) ; let jarr : $ jni_arr_type = unsafe { ( ** env ) .$ jni_new_array . unwrap ( ) ( env , arr . len ( ) as jsize ) } ; assert ! ( ! jarr . is_null ( ) ) ; unsafe { ( ** env ) .$ jni_set_array_region . unwrap ( ) ( env , jarr , 0 , arr . len ( ) as jsize , arr . as_ptr ( ) ) ; if ( ** env ) . ExceptionCheck . unwrap ( ) ( env ) != 0 { panic ! ( "{}:{} {} failed" , file ! ( ) , line ! ( ) , stringify ! ( $ jni_set_array_region ) ) ; } } jarr } } # [ allow ( dead_code ) ] impl Drop for $ rust_arr_wrapper { fn drop ( & mut self ) { assert ! ( ! self . env . is_null ( ) ) ; assert ! ( ! self . array . is_null ( ) ) ; unsafe { ( ** self . env ) .$ jni_release_array_elements . unwrap ( ) ( self . env , self . array , self . data , JNI_ABORT as jint , ) } ; } } ) * } }
define_array_handling_code!(
    [
        jni_arr_type = jbyteArray,
        rust_arr_wrapper = JavaByteArray,
        jni_get_array_elements = GetByteArrayElements,
        jni_elem_type = jbyte,
        rust_elem_type = i8,
        jni_release_array_elements = ReleaseByteArrayElements,
        jni_new_array = NewByteArray,
        jni_set_array_region = SetByteArrayRegion
    ],
    [
        jni_arr_type = jshortArray,
        rust_arr_wrapper = JavaShortArray,
        jni_get_array_elements = GetShortArrayElements,
        jni_elem_type = jshort,
        rust_elem_type = i16,
        jni_release_array_elements = ReleaseShortArrayElements,
        jni_new_array = NewShortArray,
        jni_set_array_region = SetShortArrayRegion
    ],
    [
        jni_arr_type = jintArray,
        rust_arr_wrapper = JavaIntArray,
        jni_get_array_elements = GetIntArrayElements,
        jni_elem_type = jint,
        rust_elem_type = i32,
        jni_release_array_elements = ReleaseIntArrayElements,
        jni_new_array = NewIntArray,
        jni_set_array_region = SetIntArrayRegion
    ],
    [
        jni_arr_type = jlongArray,
        rust_arr_wrapper = JavaLongArray,
        jni_get_array_elements = GetLongArrayElements,
        jni_elem_type = jlong,
        rust_elem_type = i64,
        jni_release_array_elements = ReleaseLongArrayElements,
        jni_new_array = NewLongArray,
        jni_set_array_region = SetLongArrayRegion
    ],
    [
        jni_arr_type = jfloatArray,
        rust_arr_wrapper = JavaFloatArray,
        jni_get_array_elements = GetFloatArrayElements,
        jni_elem_type = jfloat,
        rust_elem_type = f32,
        jni_release_array_elements = ReleaseFloatArrayElements,
        jni_new_array = NewFloatArray,
        jni_set_array_region = SetFloatArrayRegion
    ],
    [
        jni_arr_type = jdoubleArray,
        rust_arr_wrapper = JavaDoubleArray,
        jni_get_array_elements = GetDoubleArrayElements,
        jni_elem_type = jdouble,
        rust_elem_type = f64,
        jni_release_array_elements = ReleaseDoubleArrayElements,
        jni_new_array = NewDoubleArray,
        jni_set_array_region = SetDoubleArrayRegion
    ]
);
#[allow(dead_code)]
fn to_java_util_optional_double(
    env: *mut JNIEnv,
    x: Option<f64>,
) -> internal_aliases::JOptionalDouble {
    let class: jclass = swig_jni_find_class!(JAVA_UTIL_OPTIONAL_DOUBLE, "java/util/OptionalDouble");
    assert!(!class.is_null(),);
    match x {
        Some(val) => {
            let of_m: jmethodID = swig_jni_get_static_method_id!(
                JAVA_UTIL_OPTIONAL_DOUBLE_OF,
                JAVA_UTIL_OPTIONAL_DOUBLE,
                "of",
                "(D)Ljava/util/OptionalDouble;"
            );
            assert!(!of_m.is_null());
            let ret = unsafe {
                let ret = (**env).CallStaticObjectMethod.unwrap()(env, class, of_m, val);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("OptionalDouble.of failed: catch exception");
                }
                ret
            };
            assert!(!ret.is_null());
            ret
        }
        None => {
            let empty_m: jmethodID = swig_jni_get_static_method_id!(
                JAVA_UTIL_OPTIONAL_DOUBLE_EMPTY,
                JAVA_UTIL_OPTIONAL_DOUBLE,
                "empty",
                "()Ljava/util/OptionalDouble;"
            );
            assert!(!empty_m.is_null());
            let ret = unsafe {
                let ret = (**env).CallStaticObjectMethod.unwrap()(env, class, empty_m);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("OptionalDouble.empty failed: catch exception");
                }
                ret
            };
            assert!(!ret.is_null());
            ret
        }
    }
}
#[allow(dead_code)]
fn from_java_lang_double_to_rust(env: *mut JNIEnv, x: internal_aliases::JDouble) -> Option<f64> {
    if x.is_null() {
        None
    } else {
        let x = unsafe { (**env).NewLocalRef.unwrap()(env, x) };
        if x.is_null() {
            None
        } else {
            let class: jclass = swig_jni_find_class!(JAVA_LANG_DOUBLE, "java/lang/Double");
            assert!(!class.is_null());
            let double_value_m: jmethodID = swig_jni_get_method_id!(
                JAVA_LANG_DOUBLE_DOUBLE_VALUE_METHOD,
                JAVA_LANG_DOUBLE,
                "doubleValue",
                "()D",
            );
            assert!(!double_value_m.is_null(),);
            let ret: f64 = unsafe {
                let ret = (**env).CallDoubleMethod.unwrap()(env, x, double_value_m);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("Double.doubleValue failed: catch exception");
                }
                (**env).DeleteLocalRef.unwrap()(env, x);
                ret
            };
            Some(ret)
        }
    }
}
#[allow(dead_code)]
fn from_java_lang_float_to_rust(env: *mut JNIEnv, x: internal_aliases::JFloat) -> Option<f32> {
    if x.is_null() {
        None
    } else {
        let x = unsafe { (**env).NewLocalRef.unwrap()(env, x) };
        if x.is_null() {
            None
        } else {
            let class: jclass = swig_jni_find_class!(JAVA_LANG_FLOAT, "java/lang/Float");
            assert!(!class.is_null());
            let float_value_m: jmethodID = swig_jni_get_method_id!(
                JAVA_LANG_FLOAT_FLOAT_VALUE,
                JAVA_LANG_FLOAT,
                "floatValue",
                "()F"
            );
            assert!(!float_value_m.is_null());
            let ret: f32 = unsafe {
                let ret = (**env).CallFloatMethod.unwrap()(env, x, float_value_m);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("Float.floatValue failed: catch exception");
                }
                (**env).DeleteLocalRef.unwrap()(env, x);
                ret
            };
            Some(ret)
        }
    }
}
#[allow(dead_code)]
fn to_java_util_optional_long(env: *mut JNIEnv, x: Option<i64>) -> internal_aliases::JOptionalLong {
    let class: jclass = swig_jni_find_class!(JAVA_UTIL_OPTIONAL_LONG, "java/util/OptionalLong");
    assert!(!class.is_null(),);
    match x {
        Some(val) => {
            let of_m: jmethodID = swig_jni_get_static_method_id!(
                JAVA_UTIL_OPTIONAL_LONG_OF,
                JAVA_UTIL_OPTIONAL_LONG,
                "of",
                "(J)Ljava/util/OptionalLong;"
            );
            assert!(!of_m.is_null());
            let ret = unsafe {
                let ret = (**env).CallStaticObjectMethod.unwrap()(env, class, of_m, val);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("OptionalLong.of failed: catch exception");
                }
                ret
            };
            assert!(!ret.is_null());
            ret
        }
        None => {
            let empty_m: jmethodID = swig_jni_get_static_method_id!(
                JAVA_UTIL_OPTIONAL_LONG_EMPTY,
                JAVA_UTIL_OPTIONAL_LONG,
                "empty",
                "()Ljava/util/OptionalLong;",
            );
            assert!(!empty_m.is_null());
            let ret = unsafe {
                let ret = (**env).CallStaticObjectMethod.unwrap()(env, class, empty_m);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("OptionalLong.empty failed: catch exception");
                }
                ret
            };
            assert!(!ret.is_null());
            ret
        }
    }
}
#[allow(dead_code)]
fn from_java_lang_long_to_rust(env: *mut JNIEnv, x: internal_aliases::JLong) -> Option<i64> {
    if x.is_null() {
        None
    } else {
        let x = unsafe { (**env).NewLocalRef.unwrap()(env, x) };
        if x.is_null() {
            None
        } else {
            let class: jclass = swig_jni_find_class!(JAVA_LANG_LONG, "java/lang/Long");
            assert!(!class.is_null());
            let long_value_m: jmethodID = swig_jni_get_method_id!(
                JAVA_LANG_LONG_LONG_VALUE,
                JAVA_LANG_LONG,
                "longValue",
                "()J"
            );
            assert!(!long_value_m.is_null());
            let ret: i64 = unsafe {
                let ret = (**env).CallLongMethod.unwrap()(env, x, long_value_m);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("Long.longValue failed: catch exception");
                }
                (**env).DeleteLocalRef.unwrap()(env, x);
                ret
            };
            Some(ret)
        }
    }
}
#[allow(dead_code)]
fn from_java_lang_int_to_rust(env: *mut JNIEnv, x: internal_aliases::JInteger) -> Option<i32> {
    if x.is_null() {
        None
    } else {
        let x = unsafe { (**env).NewLocalRef.unwrap()(env, x) };
        if x.is_null() {
            None
        } else {
            let class: jclass = swig_jni_find_class!(JAVA_LANG_INTEGER, "java/lang/Integer");
            assert!(!class.is_null());
            let int_value_m: jmethodID = swig_jni_get_method_id!(
                JAVA_LANG_INTEGER_INT_VALUE,
                JAVA_LANG_INTEGER,
                "intValue",
                "()I"
            );
            assert!(!int_value_m.is_null(),);
            let ret: i32 = unsafe {
                let ret = (**env).CallIntMethod.unwrap()(env, x, int_value_m);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("Integer.intValue failed: catch exception");
                }
                (**env).DeleteLocalRef.unwrap()(env, x);
                ret
            };
            Some(ret)
        }
    }
}
#[allow(dead_code)]
fn from_java_lang_byte_to_rust(env: *mut JNIEnv, x: internal_aliases::JByte) -> Option<i8> {
    if x.is_null() {
        None
    } else {
        let x = unsafe { (**env).NewLocalRef.unwrap()(env, x) };
        if x.is_null() {
            None
        } else {
            let class: jclass = swig_jni_find_class!(JAVA_LANG_BYTE, "java/lang/Byte");
            assert!(!class.is_null());
            let byte_value_m: jmethodID = swig_jni_get_method_id!(
                JAVA_LANG_BYTE_BYTE_VALUE,
                JAVA_LANG_BYTE,
                "byteValue",
                "()B"
            );
            assert!(!byte_value_m.is_null(),);
            let ret: i8 = unsafe {
                let ret = (**env).CallByteMethod.unwrap()(env, x, byte_value_m);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("Byte.byteValue failed: catch exception");
                }
                (**env).DeleteLocalRef.unwrap()(env, x);
                ret
            };
            Some(ret)
        }
    }
}
#[allow(dead_code)]
fn from_java_lang_short_to_rust(env: *mut JNIEnv, x: internal_aliases::JByte) -> Option<i16> {
    if x.is_null() {
        None
    } else {
        let x = unsafe { (**env).NewLocalRef.unwrap()(env, x) };
        if x.is_null() {
            None
        } else {
            let class: jclass = swig_jni_find_class!(JAVA_LANG_SHORT, "java/lang/Short");
            assert!(!class.is_null());
            let short_value_m: jmethodID = swig_jni_get_method_id!(
                JAVA_LANG_SHORT_SHORT_VALUE,
                JAVA_LANG_SHORT,
                "shortValue",
                "()S"
            );
            assert!(!short_value_m.is_null());
            let ret: i16 = unsafe {
                let ret = (**env).CallShortMethod.unwrap()(env, x, short_value_m);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("Short.shortValue failed: catch exception");
                }
                (**env).DeleteLocalRef.unwrap()(env, x);
                ret
            };
            Some(ret)
        }
    }
}
#[allow(dead_code)]
fn to_java_util_optional_int(env: *mut JNIEnv, x: Option<i32>) -> jobject {
    let class: jclass = swig_jni_find_class!(JAVA_UTIL_OPTIONAL_INT, "java/util/OptionalInt");
    assert!(!class.is_null(),);
    match x {
        Some(val) => {
            let of_m: jmethodID = swig_jni_get_static_method_id!(
                JAVA_UTIL_OPTIONAL_INT_OF,
                JAVA_UTIL_OPTIONAL_INT,
                "of",
                "(I)Ljava/util/OptionalInt;"
            );
            assert!(!of_m.is_null());
            let ret = unsafe {
                let ret = (**env).CallStaticObjectMethod.unwrap()(env, class, of_m, val);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("OptionalInt.of failed: catch exception");
                }
                ret
            };
            assert!(!ret.is_null());
            ret
        }
        None => {
            let empty_m: jmethodID = swig_jni_get_static_method_id!(
                JAVA_UTIL_OPTIONAL_INT_EMPTY,
                JAVA_UTIL_OPTIONAL_INT,
                "empty",
                "()Ljava/util/OptionalInt;"
            );
            assert!(!empty_m.is_null());
            let ret = unsafe {
                let ret = (**env).CallStaticObjectMethod.unwrap()(env, class, empty_m);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("OptionalInt.empty failed: catch exception");
                }
                ret
            };
            assert!(!ret.is_null());
            ret
        }
    }
}
use crate::android_c_headers::*;
use crate::create_boo;
use crate::create_interface;
use crate::method_not_method;
use crate::Boo;
use crate::BooleanHolder;
use crate::BuildList;
use crate::Config;
use crate::EnumObserver;
use crate::Foo;
use crate::Interface;
use crate::Moo;
use crate::MyEnum;
use crate::Observable;
use crate::OnEvent;
use crate::Session;
use crate::StringHolder;
use crate::TestMethodNotMethod;
use crate::TestPathAndResult;
use chrono::{DateTime, Utc};
use std::{
    cell::{Ref, RefCell, RefMut},
    path::{Path, PathBuf},
    rc::Rc,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex, MutexGuard,
    },
    time::{Duration, SystemTime},
};
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_JNIReachabilityFence_reachabilityFence1(
    _env: *mut JNIEnv,
    _: jclass,
    _: jobject,
) {
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_JNIReachabilityFence_reachabilityFence2(
    _env: *mut JNIEnv,
    _: jclass,
    _: jobject,
    _: jobject,
) {
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_JNIReachabilityFence_reachabilityFence3(
    _env: *mut JNIEnv,
    _: jclass,
    _: jobject,
    _: jobject,
    _: jobject,
) {
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_JNIReachabilityFence_reachabilityFence4(
    _env: *mut JNIEnv,
    _: jclass,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
) {
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_JNIReachabilityFence_reachabilityFence5(
    _env: *mut JNIEnv,
    _: jclass,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
) {
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_JNIReachabilityFence_reachabilityFence6(
    _env: *mut JNIEnv,
    _: jclass,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
) {
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_JNIReachabilityFence_reachabilityFence7(
    _env: *mut JNIEnv,
    _: jclass,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
) {
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_JNIReachabilityFence_reachabilityFence8(
    _env: *mut JNIEnv,
    _: jclass,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
) {
}
impl SwigForeignClass for BooleanHolder {
    type PointedType = BooleanHolder;
    fn jni_class() -> jclass {
        swig_jni_find_class!(
            FOREIGN_CLASS_BOOLEANHOLDER,
            "net/akaame/myapplication/generated/rust_jni_interface/BooleanHolder"
        )
    }
    fn jni_class_pointer_field() -> jfieldID {
        swig_jni_get_field_id!(
            FOREIGN_CLASS_BOOLEANHOLDER_MNATIVEOBJ_FIELD,
            FOREIGN_CLASS_BOOLEANHOLDER,
            "mNativeObj",
            "J"
        )
    }
    fn box_object(this: Self) -> jlong {
        let this: Box<BooleanHolder> = Box::new(this);
        let this: *mut BooleanHolder = Box::into_raw(this);
        this as jlong
    }
    fn unbox_object(x: jlong) -> Self {
        let x: *mut BooleanHolder =
            unsafe { jlong_to_pointer::<BooleanHolder>(x).as_mut().unwrap() };
        let x: Box<BooleanHolder> = unsafe { Box::from_raw(x) };
        let x: BooleanHolder = *x;
        x
    }
    fn to_pointer(x: jlong) -> ::std::ptr::NonNull<Self::PointedType> {
        let x: *mut BooleanHolder =
            unsafe { jlong_to_pointer::<BooleanHolder>(x).as_mut().unwrap() };
        ::std::ptr::NonNull::<Self::PointedType>::new(x).unwrap()
    }
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_BooleanHolder_init(
    env: *mut JNIEnv,
    _: jclass,
) -> jlong {
    let this: BooleanHolder = BooleanHolder::new();
    let this: Box<BooleanHolder> = Box::new(this);
    let this: *mut BooleanHolder = Box::into_raw(this);
    this as jlong
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_BooleanHolder_do_1set(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
    v: jboolean,
) -> () {
    let v: bool = v != 0;
    let this: &mut BooleanHolder =
        unsafe { jlong_to_pointer::<BooleanHolder>(this).as_mut().unwrap() };
    let mut ret: () = BooleanHolder::set(this, v);
    ret
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_BooleanHolder_do_1delete(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) {
    let this: *mut BooleanHolder =
        unsafe { jlong_to_pointer::<BooleanHolder>(this).as_mut().unwrap() };
    let this: Box<BooleanHolder> = unsafe { Box::from_raw(this) };
    drop(this);
}
impl SwigForeignClass for StringHolder {
    type PointedType = StringHolder;
    fn jni_class() -> jclass {
        swig_jni_find_class!(
            FOREIGN_CLASS_STRINGHOLDER,
            "net/akaame/myapplication/generated/rust_jni_interface/StringHolder"
        )
    }
    fn jni_class_pointer_field() -> jfieldID {
        swig_jni_get_field_id!(
            FOREIGN_CLASS_STRINGHOLDER_MNATIVEOBJ_FIELD,
            FOREIGN_CLASS_STRINGHOLDER,
            "mNativeObj",
            "J"
        )
    }
    fn box_object(this: Self) -> jlong {
        let this: Box<StringHolder> = Box::new(this);
        let this: *mut StringHolder = Box::into_raw(this);
        this as jlong
    }
    fn unbox_object(x: jlong) -> Self {
        let x: *mut StringHolder = unsafe { jlong_to_pointer::<StringHolder>(x).as_mut().unwrap() };
        let x: Box<StringHolder> = unsafe { Box::from_raw(x) };
        let x: StringHolder = *x;
        x
    }
    fn to_pointer(x: jlong) -> ::std::ptr::NonNull<Self::PointedType> {
        let x: *mut StringHolder = unsafe { jlong_to_pointer::<StringHolder>(x).as_mut().unwrap() };
        ::std::ptr::NonNull::<Self::PointedType>::new(x).unwrap()
    }
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_StringHolder_init(
    env: *mut JNIEnv,
    _: jclass,
) -> jlong {
    let this: StringHolder = StringHolder::new();
    let this: Box<StringHolder> = Box::new(this);
    let this: *mut StringHolder = Box::into_raw(this);
    this as jlong
}
impl SwigInto<JavaString> for jstring {
    fn swig_into(self, env: *mut JNIEnv) -> JavaString {
        JavaString::new(env, self)
    }
}
impl SwigDeref for JavaString {
    type Target = str;
    fn swig_deref(&self) -> &Self::Target {
        self.to_str()
    }
}
impl<'a> SwigInto<String> for &'a str {
    fn swig_into(self, _: *mut JNIEnv) -> String {
        self.into()
    }
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_StringHolder_do_1set(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
    v: jstring,
) -> () {
    let mut v: JavaString = v.swig_into(env);
    let mut v: &str = v.swig_deref();
    let mut v: String = v.swig_into(env);
    let this: &mut StringHolder =
        unsafe { jlong_to_pointer::<StringHolder>(this).as_mut().unwrap() };
    let mut ret: () = StringHolder::set(this, v);
    ret
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_StringHolder_do_1delete(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) {
    let this: *mut StringHolder =
        unsafe { jlong_to_pointer::<StringHolder>(this).as_mut().unwrap() };
    let this: Box<StringHolder> = unsafe { Box::from_raw(this) };
    drop(this);
}
impl SwigForeignClass for Config {
    type PointedType = Config;
    fn jni_class() -> jclass {
        swig_jni_find_class!(
            FOREIGN_CLASS_CONFIG,
            "net/akaame/myapplication/generated/rust_jni_interface/Config"
        )
    }
    fn jni_class_pointer_field() -> jfieldID {
        swig_jni_get_field_id!(
            FOREIGN_CLASS_CONFIG_MNATIVEOBJ_FIELD,
            FOREIGN_CLASS_CONFIG,
            "mNativeObj",
            "J"
        )
    }
    fn box_object(this: Self) -> jlong {
        let this: Box<Config> = Box::new(this);
        let this: *mut Config = Box::into_raw(this);
        this as jlong
    }
    fn unbox_object(x: jlong) -> Self {
        let x: *mut Config = unsafe { jlong_to_pointer::<Config>(x).as_mut().unwrap() };
        let x: Box<Config> = unsafe { Box::from_raw(x) };
        let x: Config = *x;
        x
    }
    fn to_pointer(x: jlong) -> ::std::ptr::NonNull<Self::PointedType> {
        let x: *mut Config = unsafe { jlong_to_pointer::<Config>(x).as_mut().unwrap() };
        ::std::ptr::NonNull::<Self::PointedType>::new(x).unwrap()
    }
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Config_init(
    env: *mut JNIEnv,
    _: jclass,
    name: jstring,
) -> jlong {
    let mut name: JavaString = name.swig_into(env);
    let mut name: &str = name.swig_deref();
    let mut name: String = name.swig_into(env);
    let this: Config = Config::new(name);
    let this: Box<Config> = Box::new(this);
    let this: *mut Config = Box::into_raw(this);
    this as jlong
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Config_do_1delete(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) {
    let this: *mut Config = unsafe { jlong_to_pointer::<Config>(this).as_mut().unwrap() };
    let this: Box<Config> = unsafe { Box::from_raw(this) };
    drop(this);
}
impl SwigForeignClass for Session {
    type PointedType = Session;
    fn jni_class() -> jclass {
        swig_jni_find_class!(
            FOREIGN_CLASS_SESSION,
            "net/akaame/myapplication/generated/rust_jni_interface/Session"
        )
    }
    fn jni_class_pointer_field() -> jfieldID {
        swig_jni_get_field_id!(
            FOREIGN_CLASS_SESSION_MNATIVEOBJ_FIELD,
            FOREIGN_CLASS_SESSION,
            "mNativeObj",
            "J"
        )
    }
    fn box_object(this: Self) -> jlong {
        let this: Box<Session> = Box::new(this);
        let this: *mut Session = Box::into_raw(this);
        this as jlong
    }
    fn unbox_object(x: jlong) -> Self {
        let x: *mut Session = unsafe { jlong_to_pointer::<Session>(x).as_mut().unwrap() };
        let x: Box<Session> = unsafe { Box::from_raw(x) };
        let x: Session = *x;
        x
    }
    fn to_pointer(x: jlong) -> ::std::ptr::NonNull<Self::PointedType> {
        let x: *mut Session = unsafe { jlong_to_pointer::<Session>(x).as_mut().unwrap() };
        ::std::ptr::NonNull::<Self::PointedType>::new(x).unwrap()
    }
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Session_init(
    env: *mut JNIEnv,
    _: jclass,
    cfg: jlong,
) -> jlong {
    let cfg: *mut Config = unsafe { jlong_to_pointer::<Config>(cfg).as_mut().unwrap() };
    let cfg: Box<Config> = unsafe { Box::from_raw(cfg) };
    let cfg: Config = *cfg;
    let this: Session = Session::new(cfg);
    let this: Box<Session> = Box::new(this);
    let this: *mut Session = Box::into_raw(this);
    this as jlong
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Session_do_1add_1and1(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
    val: jint,
) -> jint {
    let val: i32 = val;
    let this: &Session = unsafe { jlong_to_pointer::<Session>(this).as_mut().unwrap() };
    let mut ret: i32 = Session::add_and1(this, val);
    let ret: jint = ret;
    ret
}
impl SwigFrom<String> for jstring {
    fn swig_from(x: String, env: *mut JNIEnv) -> Self {
        from_std_string_jstring(x, env)
    }
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Session_greet(
    env: *mut JNIEnv,
    _: jclass,
    to: jstring,
) -> jstring {
    let mut to: JavaString = to.swig_into(env);
    let mut to: &str = to.swig_deref();
    let mut ret: String = Session::greet(to);
    let mut ret: jstring = <jstring>::swig_from(ret, env);
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Session_do_1calcF(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
    a0: jint,
    a1: jint,
) -> jint {
    let a0: i32 = a0;
    let a1: i32 = a1;
    let this: &Session = unsafe { jlong_to_pointer::<Session>(this).as_mut().unwrap() };
    let mut ret: i32 = Session::f(this, a0, a1);
    let ret: jint = ret;
    ret
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Session_do_1delete(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) {
    let this: *mut Session = unsafe { jlong_to_pointer::<Session>(this).as_mut().unwrap() };
    let this: Box<Session> = unsafe { Box::from_raw(this) };
    drop(this);
}
impl SwigForeignCLikeEnum for MyEnum {
    fn as_jint(&self) -> jint {
        match *self {
            MyEnum::Item1 => 0i32,
            MyEnum::Item2 => 1i32,
            MyEnum::Item3 => 2i32,
        }
    }
    fn from_jint(x: jint) -> Self {
        match x {
            0i32 => MyEnum::Item1,
            1i32 => MyEnum::Item2,
            2i32 => MyEnum::Item3,
            _ => panic!(concat!("{} not expected for ", stringify!(MyEnum)), x),
        }
    }
}
#[allow(dead_code)]
impl SwigFrom<MyEnum> for jobject {
    fn swig_from(x: MyEnum, env: *mut JNIEnv) -> jobject {
        let cls: jclass = swig_jni_find_class!(
            FOREIGN_ENUM_MYENUM,
            "net/akaame/myapplication/generated/rust_jni_interface/MyEnum"
        );
        assert!(!cls.is_null());
        let static_field_id: jfieldID = match x {
            MyEnum::Item1 => {
                let field = swig_jni_get_static_field_id!(
                    FOREIGN_ENUM_MYENUM_ITEM1,
                    FOREIGN_ENUM_MYENUM,
                    "ITEM1",
                    "Lnet/akaame/myapplication/generated/rust_jni_interface/MyEnum;"
                );
                assert!(!field.is_null());
                field
            }
            MyEnum::Item2 => {
                let field = swig_jni_get_static_field_id!(
                    FOREIGN_ENUM_MYENUM_ITEM2,
                    FOREIGN_ENUM_MYENUM,
                    "ITEM2",
                    "Lnet/akaame/myapplication/generated/rust_jni_interface/MyEnum;"
                );
                assert!(!field.is_null());
                field
            }
            MyEnum::Item3 => {
                let field = swig_jni_get_static_field_id!(
                    FOREIGN_ENUM_MYENUM_ITEM3,
                    FOREIGN_ENUM_MYENUM,
                    "ITEM3",
                    "Lnet/akaame/myapplication/generated/rust_jni_interface/MyEnum;"
                );
                assert!(!field.is_null());
                field
            }
        };
        assert!(!static_field_id.is_null());
        let ret: jobject =
            unsafe { (**env).GetStaticObjectField.unwrap()(env, cls, static_field_id) };
        assert!(
            !ret.is_null(),
            concat!(
                "Can get value of item in ",
                "net/akaame/myapplication/generated/rust_jni_interface/MyEnum"
            )
        );
        ret
    }
}
impl SwigForeignClass for Box<Box<Interface>> {
    type PointedType = Box<Interface>;
    fn jni_class() -> jclass {
        swig_jni_find_class!(
            FOREIGN_CLASS_INTERFACE,
            "net/akaame/myapplication/generated/rust_jni_interface/Interface"
        )
    }
    fn jni_class_pointer_field() -> jfieldID {
        swig_jni_get_field_id!(
            FOREIGN_CLASS_INTERFACE_MNATIVEOBJ_FIELD,
            FOREIGN_CLASS_INTERFACE,
            "mNativeObj",
            "J"
        )
    }
    fn box_object(this: Self) -> jlong {
        let this: *const Box<Interface> = Box::into_raw(this);
        this as jlong
    }
    fn unbox_object(x: jlong) -> Self {
        let x: *mut Box<Interface> =
            unsafe { jlong_to_pointer::<Box<Interface>>(x).as_mut().unwrap() };
        let x: Box<Box<Interface>> = unsafe { Box::from_raw(x) };
        x
    }
    fn to_pointer(x: jlong) -> ::std::ptr::NonNull<Self::PointedType> {
        let x: *mut Box<Interface> =
            unsafe { jlong_to_pointer::<Box<Interface>>(x).as_mut().unwrap() };
        ::std::ptr::NonNull::<Self::PointedType>::new(x).unwrap()
    }
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Interface_init(
    env: *mut JNIEnv,
    _: jclass,
) -> jlong {
    let this: Box<Box<Interface>> = create_interface();
    let this: *const Box<Interface> = Box::into_raw(this);
    this as jlong
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Interface_do_1f(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
    a0: jint,
) -> jint {
    let a0: i32 = a0;
    let this: &Box<Interface> =
        unsafe { jlong_to_pointer::<Box<Interface>>(this).as_mut().unwrap() };
    let mut this: &Interface = this.as_ref();
    let mut ret: i32 = Interface::f(this, a0);
    let ret: jint = ret;
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Interface_do_1set(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
    x: jint,
) -> () {
    let x: i32 = x;
    let this: &mut Box<Interface> =
        unsafe { jlong_to_pointer::<Box<Interface>>(this).as_mut().unwrap() };
    let mut this: &mut Interface = this.as_mut();
    let mut ret: () = Interface::set(this, x);
    ret
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Interface_do_1delete(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) {
    let this: *mut Box<Interface> =
        unsafe { jlong_to_pointer::<Box<Interface>>(this).as_mut().unwrap() };
    let this: Box<Box<Interface>> = unsafe { Box::from_raw(this) };
    drop(this);
}
impl SwigForeignClass for TestMethodNotMethod {
    type PointedType = TestMethodNotMethod;
    fn jni_class() -> jclass {
        swig_jni_find_class!(
            FOREIGN_CLASS_TESTMETHODNOTMETHOD,
            "net/akaame/myapplication/generated/rust_jni_interface/TestMethodNotMethod"
        )
    }
    fn jni_class_pointer_field() -> jfieldID {
        swig_jni_get_field_id!(
            FOREIGN_CLASS_TESTMETHODNOTMETHOD_MNATIVEOBJ_FIELD,
            FOREIGN_CLASS_TESTMETHODNOTMETHOD,
            "mNativeObj",
            "J"
        )
    }
    fn box_object(this: Self) -> jlong {
        let this: Box<TestMethodNotMethod> = Box::new(this);
        let this: *mut TestMethodNotMethod = Box::into_raw(this);
        this as jlong
    }
    fn unbox_object(x: jlong) -> Self {
        let x: *mut TestMethodNotMethod =
            unsafe { jlong_to_pointer::<TestMethodNotMethod>(x).as_mut().unwrap() };
        let x: Box<TestMethodNotMethod> = unsafe { Box::from_raw(x) };
        let x: TestMethodNotMethod = *x;
        x
    }
    fn to_pointer(x: jlong) -> ::std::ptr::NonNull<Self::PointedType> {
        let x: *mut TestMethodNotMethod =
            unsafe { jlong_to_pointer::<TestMethodNotMethod>(x).as_mut().unwrap() };
        ::std::ptr::NonNull::<Self::PointedType>::new(x).unwrap()
    }
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_TestMethodNotMethod_init(
    env: *mut JNIEnv,
    _: jclass,
) -> jlong {
    let this: TestMethodNotMethod = TestMethodNotMethod::new();
    let this: Box<TestMethodNotMethod> = Box::new(this);
    let this: *mut TestMethodNotMethod = Box::into_raw(this);
    this as jlong
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_TestMethodNotMethod_do_1method_1not_1method(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) -> () {
    let this: &TestMethodNotMethod = unsafe {
        jlong_to_pointer::<TestMethodNotMethod>(this)
            .as_mut()
            .unwrap()
    };
    let mut ret: () = method_not_method(this);
    ret
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_TestMethodNotMethod_do_1delete(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) {
    let this: *mut TestMethodNotMethod = unsafe {
        jlong_to_pointer::<TestMethodNotMethod>(this)
            .as_mut()
            .unwrap()
    };
    let this: Box<TestMethodNotMethod> = unsafe { Box::from_raw(this) };
    drop(this);
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_TestFnInline_int_1to_1str(
    env: *mut JNIEnv,
    _: jclass,
    a: jint,
) -> jstring {
    let a: i32 = a;
    let mut ret: String = { format!("{}", a) };
    let mut ret: jstring = <jstring>::swig_from(ret, env);
    ret
}
impl SwigForeignClass for Foo {
    type PointedType = Foo;
    fn jni_class() -> jclass {
        swig_jni_find_class!(
            FOREIGN_CLASS_FOO,
            "net/akaame/myapplication/generated/rust_jni_interface/Foo"
        )
    }
    fn jni_class_pointer_field() -> jfieldID {
        swig_jni_get_field_id!(
            FOREIGN_CLASS_FOO_MNATIVEOBJ_FIELD,
            FOREIGN_CLASS_FOO,
            "mNativeObj",
            "J"
        )
    }
    fn box_object(this: Self) -> jlong {
        let this: Box<Foo> = Box::new(this);
        let this: *mut Foo = Box::into_raw(this);
        this as jlong
    }
    fn unbox_object(x: jlong) -> Self {
        let x: *mut Foo = unsafe { jlong_to_pointer::<Foo>(x).as_mut().unwrap() };
        let x: Box<Foo> = unsafe { Box::from_raw(x) };
        let x: Foo = *x;
        x
    }
    fn to_pointer(x: jlong) -> ::std::ptr::NonNull<Self::PointedType> {
        let x: *mut Foo = unsafe { jlong_to_pointer::<Foo>(x).as_mut().unwrap() };
        ::std::ptr::NonNull::<Self::PointedType>::new(x).unwrap()
    }
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Foo_init(
    env: *mut JNIEnv,
    _: jclass,
    val: jint,
    name: jstring,
) -> jlong {
    let val: i32 = val;
    let mut name: JavaString = name.swig_into(env);
    let mut name: &str = name.swig_deref();
    let this: Foo = Foo::new(val, name);
    let this: Box<Foo> = Box::new(this);
    let this: *mut Foo = Box::into_raw(this);
    this as jlong
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Foo_do_1calcF(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
    a0: jint,
    a1: jint,
) -> jint {
    let a0: i32 = a0;
    let a1: i32 = a1;
    let this: &Foo = unsafe { jlong_to_pointer::<Foo>(this).as_mut().unwrap() };
    let mut ret: i32 = Foo::f(this, a0, a1);
    let ret: jint = ret;
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Foo_do_1f_1double(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
    a0: jdouble,
    a1: jdouble,
) -> jdouble {
    let a0: f64 = a0;
    let a1: f64 = a1;
    let this: &Foo = unsafe { jlong_to_pointer::<Foo>(this).as_mut().unwrap() };
    let mut ret: f64 = Foo::f_double(this, a0, a1);
    let ret: jdouble = ret;
    ret
}
impl<'a> SwigFrom<&'a str> for jstring {
    fn swig_from(x: &'a str, env: *mut JNIEnv) -> Self {
        let x = ::std::ffi::CString::new(x).unwrap();
        unsafe { (**env).NewStringUTF.unwrap()(env, x.as_ptr()) }
    }
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Foo_do_1getName(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) -> jstring {
    let this: &Foo = unsafe { jlong_to_pointer::<Foo>(this).as_mut().unwrap() };
    let mut ret: &str = Foo::name(this);
    let mut ret: jstring = <jstring>::swig_from(ret, env);
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Foo_fHypot(
    env: *mut JNIEnv,
    _: jclass,
    a: jdouble,
    b: jdouble,
) -> jdouble {
    let a: f64 = a;
    let b: f64 = b;
    let mut ret: f64 = { a.hypot(b) };
    let ret: jdouble = ret;
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Foo_do_1now(
    env: *mut JNIEnv,
    _: jclass,
) -> jlong {
    let mut ret: SystemTime = { SystemTime::now() };
    let since_unix_epoch = ret
        .duration_since(::std::time::UNIX_EPOCH)
        .expect("SystemTime to Unix time conv. error");
    let ret: jlong = <i64 as ::std::convert::TryFrom<u64>>::try_from(
        since_unix_epoch.as_secs() * 1_000 + u64::from(since_unix_epoch.subsec_millis()),
    )
    .expect("SystemTime: milleseconds u64 to i64 convert error");
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Foo_do_1chrono_1now(
    env: *mut JNIEnv,
    _: jclass,
) -> jlong {
    let mut ret: DateTime<Utc> = { Utc::now() };
    let ret: jlong = ret.timestamp_millis();
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Foo_do_1chrono_1now_1opt(
    env: *mut JNIEnv,
    _: jclass,
    flag: jboolean,
) -> internal_aliases::JOptionalLong {
    let flag: bool = flag != 0;
    let mut ret: Option<DateTime<Utc>> = {
        if flag {
            Some(Utc::now())
        } else {
            None
        }
    };
    let tmp: Option<i64> = ret.map(|x| x.timestamp_millis());
    let ret: internal_aliases::JOptionalLong = to_java_util_optional_long(env, tmp);
    ret
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Foo_do_1delete(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) {
    let this: *mut Foo = unsafe { jlong_to_pointer::<Foo>(this).as_mut().unwrap() };
    let this: Box<Foo> = unsafe { Box::from_raw(this) };
    drop(this);
}
impl SwigForeignClass for Rc<RefCell<Boo>> {
    type PointedType = RefCell<Boo>;
    fn jni_class() -> jclass {
        swig_jni_find_class!(
            FOREIGN_CLASS_BOO,
            "net/akaame/myapplication/generated/rust_jni_interface/Boo"
        )
    }
    fn jni_class_pointer_field() -> jfieldID {
        swig_jni_get_field_id!(
            FOREIGN_CLASS_BOO_MNATIVEOBJ_FIELD,
            FOREIGN_CLASS_BOO,
            "mNativeObj",
            "J"
        )
    }
    fn box_object(this: Self) -> jlong {
        let this: *const RefCell<Boo> = Rc::into_raw(this);
        this as jlong
    }
    fn unbox_object(x: jlong) -> Self {
        let x: *mut RefCell<Boo> = unsafe { jlong_to_pointer::<RefCell<Boo>>(x).as_mut().unwrap() };
        let x: Rc<RefCell<Boo>> = unsafe { Rc::from_raw(x) };
        x
    }
    fn to_pointer(x: jlong) -> ::std::ptr::NonNull<Self::PointedType> {
        let x: *mut RefCell<Boo> = unsafe { jlong_to_pointer::<RefCell<Boo>>(x).as_mut().unwrap() };
        ::std::ptr::NonNull::<Self::PointedType>::new(x).unwrap()
    }
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Boo_init(
    env: *mut JNIEnv,
    _: jclass,
) -> jlong {
    let this: Rc<RefCell<Boo>> = create_boo();
    let this: *const RefCell<Boo> = Rc::into_raw(this);
    this as jlong
}
impl<'a, T> SwigFrom<&'a RefCell<T>> for Ref<'a, T> {
    fn swig_from(m: &'a RefCell<T>, _: *mut JNIEnv) -> Ref<'a, T> {
        m.borrow()
    }
}
impl<'a, T> SwigDeref for Ref<'a, T> {
    type Target = T;
    fn swig_deref(&self) -> &T {
        self
    }
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Boo_do_1test(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
    a0: jboolean,
) -> jfloat {
    let a0: bool = a0 != 0;
    let this: &RefCell<Boo> = unsafe { jlong_to_pointer::<RefCell<Boo>>(this).as_mut().unwrap() };
    let mut this: Ref<Boo> = <Ref<Boo>>::swig_from(this, env);
    let mut this: &Boo = this.swig_deref();
    let mut ret: f32 = Boo::test(this, a0);
    let ret: jfloat = ret;
    ret
}
impl<'a, T> SwigFrom<&'a RefCell<T>> for RefMut<'a, T> {
    fn swig_from(m: &'a RefCell<T>, _: *mut JNIEnv) -> RefMut<'a, T> {
        m.borrow_mut()
    }
}
impl<'a, T> SwigDerefMut for RefMut<'a, T> {
    type Target = T;
    fn swig_deref_mut(&mut self) -> &mut T {
        self
    }
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Boo_do_1setA(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
    a0: jint,
) -> () {
    let a0: i32 = a0;
    let this: &mut RefCell<Boo> =
        unsafe { jlong_to_pointer::<RefCell<Boo>>(this).as_mut().unwrap() };
    let mut this: &RefCell<Boo> = this;
    let mut this: RefMut<Boo> = <RefMut<Boo>>::swig_from(this, env);
    let mut this: &mut Boo = this.swig_deref_mut();
    let mut ret: () = Boo::set_a(this, a0);
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Boo_do_1getA(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) -> jint {
    let this: &RefCell<Boo> = unsafe { jlong_to_pointer::<RefCell<Boo>>(this).as_mut().unwrap() };
    let mut this: Ref<Boo> = <Ref<Boo>>::swig_from(this, env);
    let mut this: &Boo = this.swig_deref();
    let mut ret: i32 = Boo::get_a(this);
    let ret: jint = ret;
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Boo_test_1u8(
    env: *mut JNIEnv,
    _: jclass,
    v: jshort,
) -> jshort {
    let v: u8 = <u8 as ::std::convert::TryFrom<jshort>>::try_from(v)
        .expect("invalid jshort, in jshort => u8 conversation");
    let mut ret: u8 = { v + 1 };
    let ret: jshort = jshort::from(ret);
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Boo_test_1i8(
    env: *mut JNIEnv,
    _: jclass,
    v: jbyte,
) -> jbyte {
    let v: i8 = v;
    let mut ret: i8 = { v + 1 };
    let ret: jbyte = ret;
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Boo_test_1u16(
    env: *mut JNIEnv,
    _: jclass,
    v: jint,
) -> jint {
    let v: u16 = <u16 as ::std::convert::TryFrom<jint>>::try_from(v)
        .expect("invalid jint, in jint => u16 conversation");
    let mut ret: u16 = { v + 1 };
    let ret: jint = jint::from(ret);
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Boo_test_1i16(
    env: *mut JNIEnv,
    _: jclass,
    v: jshort,
) -> jshort {
    let v: i16 = v;
    let mut ret: i16 = { v + 1 };
    let ret: jshort = ret;
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Boo_test_1i32(
    env: *mut JNIEnv,
    _: jclass,
    v: jint,
) -> jint {
    let v: i32 = v;
    let mut ret: i32 = { v + 1 };
    let ret: jint = ret;
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Boo_test_1u32(
    env: *mut JNIEnv,
    _: jclass,
    v: jlong,
) -> jlong {
    let v: u32 = <u32 as ::std::convert::TryFrom<jlong>>::try_from(v)
        .expect("invalid jlong, in jlong => u32 conversation");
    let mut ret: u32 = { v + 1 };
    let ret: jlong = jlong::from(ret);
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Boo_test_1u64(
    env: *mut JNIEnv,
    _: jclass,
    v: jlong,
) -> jlong {
    let v: u64 = <u64 as ::std::convert::TryFrom<jlong>>::try_from(v)
        .expect("invalid jlong, in jlong => u64 conversation");
    let mut ret: u64 = { v + 1 };
    let ret: jlong = <jlong as ::std::convert::TryFrom<u64>>::try_from(ret)
        .expect("invalid u64, in u64 => jlong conversation");
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Boo_test_1i64(
    env: *mut JNIEnv,
    _: jclass,
    v: jlong,
) -> jlong {
    let v: i64 = v;
    let mut ret: i64 = { v + 1 };
    let ret: jlong = ret;
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Boo_test_1f32(
    env: *mut JNIEnv,
    _: jclass,
    v: jfloat,
) -> jfloat {
    let v: f32 = v;
    let mut ret: f32 = { v + 1. };
    let ret: jfloat = ret;
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Boo_test_1f64(
    env: *mut JNIEnv,
    _: jclass,
    v: jdouble,
) -> jdouble {
    let v: f64 = v;
    let mut ret: f64 = { v + 1. };
    let ret: jdouble = ret;
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Boo_r_1test_1u8(
    env: *mut JNIEnv,
    _: jclass,
    v: jshort,
) -> jshort {
    let v: u8 = <u8 as ::std::convert::TryFrom<jshort>>::try_from(v)
        .expect("invalid jshort, in jshort => u8 conversation");
    let mut ret: Result<u8, &str> = { Ok(v + 1) };
    let ret: jshort = match ret {
        Ok(x) => {
            let ret: jshort = jshort::from(x);
            ret
        }
        Err(msg) => {
            jni_throw_exception(env, msg);
            return <jshort>::jni_invalid_value();
        }
    };
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Boo_r_1test_1i8(
    env: *mut JNIEnv,
    _: jclass,
    v: jbyte,
) -> jbyte {
    let v: i8 = v;
    let mut ret: Result<i8, &str> = { Ok(v + 1) };
    let ret: jbyte = match ret {
        Ok(x) => {
            let ret: jbyte = x;
            ret
        }
        Err(msg) => {
            jni_throw_exception(env, msg);
            return <jbyte>::jni_invalid_value();
        }
    };
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Boo_r_1test_1u16(
    env: *mut JNIEnv,
    _: jclass,
    v: jint,
) -> jint {
    let v: u16 = <u16 as ::std::convert::TryFrom<jint>>::try_from(v)
        .expect("invalid jint, in jint => u16 conversation");
    let mut ret: Result<u16, &str> = { Ok(v + 1) };
    let ret: jint = match ret {
        Ok(x) => {
            let ret: jint = jint::from(x);
            ret
        }
        Err(msg) => {
            jni_throw_exception(env, msg);
            return <jint>::jni_invalid_value();
        }
    };
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Boo_r_1test_1i16(
    env: *mut JNIEnv,
    _: jclass,
    v: jshort,
) -> jshort {
    let v: i16 = v;
    let mut ret: Result<i16, &str> = { Ok(v + 1) };
    let ret: jshort = match ret {
        Ok(x) => {
            let ret: jshort = x;
            ret
        }
        Err(msg) => {
            jni_throw_exception(env, msg);
            return <jshort>::jni_invalid_value();
        }
    };
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Boo_r_1test_1i32(
    env: *mut JNIEnv,
    _: jclass,
    v: jint,
) -> jint {
    let v: i32 = v;
    let mut ret: Result<i32, &str> = { Ok(v + 1) };
    let ret: jint = match ret {
        Ok(x) => {
            let ret: jint = x;
            ret
        }
        Err(msg) => {
            jni_throw_exception(env, msg);
            return <jint>::jni_invalid_value();
        }
    };
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Boo_r_1test_1u32(
    env: *mut JNIEnv,
    _: jclass,
    v: jlong,
) -> jlong {
    let v: u32 = <u32 as ::std::convert::TryFrom<jlong>>::try_from(v)
        .expect("invalid jlong, in jlong => u32 conversation");
    let mut ret: Result<u32, &str> = { Ok(v + 1) };
    let ret: jlong = match ret {
        Ok(x) => {
            let ret: jlong = jlong::from(x);
            ret
        }
        Err(msg) => {
            jni_throw_exception(env, msg);
            return <jlong>::jni_invalid_value();
        }
    };
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Boo_r_1test_1u64(
    env: *mut JNIEnv,
    _: jclass,
    v: jlong,
) -> jlong {
    let v: u64 = <u64 as ::std::convert::TryFrom<jlong>>::try_from(v)
        .expect("invalid jlong, in jlong => u64 conversation");
    let mut ret: Result<u64, &str> = { Ok(v + 1) };
    let ret: jlong = match ret {
        Ok(x) => {
            let ret: jlong = <jlong as ::std::convert::TryFrom<u64>>::try_from(x)
                .expect("invalid u64, in u64 => jlong conversation");
            ret
        }
        Err(msg) => {
            jni_throw_exception(env, msg);
            return <jlong>::jni_invalid_value();
        }
    };
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Boo_r_1test_1i64(
    env: *mut JNIEnv,
    _: jclass,
    v: jlong,
) -> jlong {
    let v: i64 = v;
    let mut ret: Result<i64, &str> = { Ok(v + 1) };
    let ret: jlong = match ret {
        Ok(x) => {
            let ret: jlong = x;
            ret
        }
        Err(msg) => {
            jni_throw_exception(env, msg);
            return <jlong>::jni_invalid_value();
        }
    };
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Boo_r_1test_1f32(
    env: *mut JNIEnv,
    _: jclass,
    v: jfloat,
) -> jfloat {
    let v: f32 = v;
    let mut ret: Result<f32, &str> = { Ok(v + 1.) };
    let ret: jfloat = match ret {
        Ok(x) => {
            let ret: jfloat = x;
            ret
        }
        Err(msg) => {
            jni_throw_exception(env, msg);
            return <jfloat>::jni_invalid_value();
        }
    };
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Boo_r_1test_1f64(
    env: *mut JNIEnv,
    _: jclass,
    v: jdouble,
) -> jdouble {
    let v: f64 = v;
    let mut ret: Result<f64, &str> = { Ok(v + 1.) };
    let ret: jdouble = match ret {
        Ok(x) => {
            let ret: jdouble = x;
            ret
        }
        Err(msg) => {
            jni_throw_exception(env, msg);
            return <jdouble>::jni_invalid_value();
        }
    };
    ret
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Boo_do_1delete(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) {
    let this: *mut RefCell<Boo> =
        unsafe { jlong_to_pointer::<RefCell<Boo>>(this).as_mut().unwrap() };
    let this: Rc<RefCell<Boo>> = unsafe { Rc::from_raw(this) };
    drop(this);
}
impl SwigForeignClass for TestPathAndResult {
    type PointedType = TestPathAndResult;
    fn jni_class() -> jclass {
        swig_jni_find_class!(
            FOREIGN_CLASS_TESTPATHANDRESULT,
            "net/akaame/myapplication/generated/rust_jni_interface/TestPathAndResult"
        )
    }
    fn jni_class_pointer_field() -> jfieldID {
        swig_jni_get_field_id!(
            FOREIGN_CLASS_TESTPATHANDRESULT_MNATIVEOBJ_FIELD,
            FOREIGN_CLASS_TESTPATHANDRESULT,
            "mNativeObj",
            "J"
        )
    }
    fn box_object(this: Self) -> jlong {
        let this: Box<TestPathAndResult> = Box::new(this);
        let this: *mut TestPathAndResult = Box::into_raw(this);
        this as jlong
    }
    fn unbox_object(x: jlong) -> Self {
        let x: *mut TestPathAndResult =
            unsafe { jlong_to_pointer::<TestPathAndResult>(x).as_mut().unwrap() };
        let x: Box<TestPathAndResult> = unsafe { Box::from_raw(x) };
        let x: TestPathAndResult = *x;
        x
    }
    fn to_pointer(x: jlong) -> ::std::ptr::NonNull<Self::PointedType> {
        let x: *mut TestPathAndResult =
            unsafe { jlong_to_pointer::<TestPathAndResult>(x).as_mut().unwrap() };
        ::std::ptr::NonNull::<Self::PointedType>::new(x).unwrap()
    }
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_TestPathAndResult_init__(
    env: *mut JNIEnv,
    _: jclass,
) -> jlong {
    let this: Result<TestPathAndResult, String> = TestPathAndResult::empty();
    let this: jlong = match this {
        Ok(x) => {
            let ret: jlong = <TestPathAndResult>::box_object(x);
            ret
        }
        Err(msg) => {
            jni_throw_exception(env, &msg);
            return <jlong>::jni_invalid_value();
        }
    };
    this as jlong
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_TestPathAndResult_init__Ljava_lang_String_2(
    env: *mut JNIEnv,
    _: jclass,
    path: internal_aliases::JStringPath,
) -> jlong {
    let jstr = JavaString::new(env, path);
    let path: &Path = Path::new(jstr.to_str());
    let this: Result<TestPathAndResult, String> = TestPathAndResult::new(path);
    let this: jlong = match this {
        Ok(x) => {
            let ret: jlong = <TestPathAndResult>::box_object(x);
            ret
        }
        Err(msg) => {
            jni_throw_exception(env, &msg);
            return <jlong>::jni_invalid_value();
        }
    };
    this as jlong
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_TestPathAndResult_do_1getPath(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) -> jstring {
    let this: &TestPathAndResult = unsafe {
        jlong_to_pointer::<TestPathAndResult>(this)
            .as_mut()
            .unwrap()
    };
    let mut ret: String = TestPathAndResult::get_path(this);
    let mut ret: jstring = <jstring>::swig_from(ret, env);
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_TestPathAndResult_do_1getBoo(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) -> jlong {
    let this: &TestPathAndResult = unsafe {
        jlong_to_pointer::<TestPathAndResult>(this)
            .as_mut()
            .unwrap()
    };
    let mut ret: Rc<RefCell<Boo>> = TestPathAndResult::get_boo(this);
    let ret: jlong = <Rc<RefCell<Boo>>>::box_object(ret);
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_TestPathAndResult_do_1get_1foo_1list(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) -> internal_aliases::JForeignObjectsArray<Foo> {
    let this: &TestPathAndResult = unsafe {
        jlong_to_pointer::<TestPathAndResult>(this)
            .as_mut()
            .unwrap()
    };
    let mut ret: Vec<Foo> = TestPathAndResult::get_foo_list(this);
    let ret: internal_aliases::JForeignObjectsArray<Foo> =
        vec_of_objects_to_jobject_array(env, ret);
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_TestPathAndResult_get_1result_1foo_1list(
    env: *mut JNIEnv,
    _: jclass,
    generate_err: jboolean,
) -> internal_aliases::JForeignObjectsArray<Foo> {
    let generate_err: bool = generate_err != 0;
    let mut ret: Result<Vec<Foo>, String> = TestPathAndResult::get_result_foo_list(generate_err);
    let ret: internal_aliases::JForeignObjectsArray<Foo> = match ret {
        Ok(x) => {
            let ret: internal_aliases::JForeignObjectsArray<Foo> =
                vec_of_objects_to_jobject_array(env, x);
            ret
        }
        Err(msg) => {
            jni_throw_exception(env, &msg);
            return <internal_aliases::JForeignObjectsArray<Foo>>::jni_invalid_value();
        }
    };
    ret
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_TestPathAndResult_do_1delete(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) {
    let this: *mut TestPathAndResult = unsafe {
        jlong_to_pointer::<TestPathAndResult>(this)
            .as_mut()
            .unwrap()
    };
    let this: Box<TestPathAndResult> = unsafe { Box::from_raw(this) };
    drop(this);
}
impl EnumObserver for JavaCallback {
    #[allow(unused_mut)]
    fn on_state_changed(&self, a0: MyEnum, a1: bool) {
        swig_assert_eq_size!(::std::os::raw::c_uint, u32);
        swig_assert_eq_size!(::std::os::raw::c_int, i32);
        let env = self.get_jni_env();
        if let Some(env) = env.env {
            let mut a0: jobject = <jobject>::swig_from(a0, env);
            let a1: jboolean = if a1 { 1 as jboolean } else { 0 as jboolean };
            unsafe {
                (**env).CallVoidMethod.unwrap()(
                    env,
                    self.this,
                    self.methods[0usize],
                    a0,
                    a1 as ::std::os::raw::c_uint,
                );
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    log::error!(concat!(
                        stringify!(on_state_changed),
                        ": java throw exception"
                    ));
                    (**env).ExceptionDescribe.unwrap()(env);
                    (**env).ExceptionClear.unwrap()(env);
                }
            };
        }
    }
}
impl SwigForeignClass for Moo {
    type PointedType = Moo;
    fn jni_class() -> jclass {
        swig_jni_find_class!(
            FOREIGN_CLASS_TESTENUMCLASS,
            "net/akaame/myapplication/generated/rust_jni_interface/TestEnumClass"
        )
    }
    fn jni_class_pointer_field() -> jfieldID {
        swig_jni_get_field_id!(
            FOREIGN_CLASS_TESTENUMCLASS_MNATIVEOBJ_FIELD,
            FOREIGN_CLASS_TESTENUMCLASS,
            "mNativeObj",
            "J"
        )
    }
    fn box_object(this: Self) -> jlong {
        let this: Box<Moo> = Box::new(this);
        let this: *mut Moo = Box::into_raw(this);
        this as jlong
    }
    fn unbox_object(x: jlong) -> Self {
        let x: *mut Moo = unsafe { jlong_to_pointer::<Moo>(x).as_mut().unwrap() };
        let x: Box<Moo> = unsafe { Box::from_raw(x) };
        let x: Moo = *x;
        x
    }
    fn to_pointer(x: jlong) -> ::std::ptr::NonNull<Self::PointedType> {
        let x: *mut Moo = unsafe { jlong_to_pointer::<Moo>(x).as_mut().unwrap() };
        ::std::ptr::NonNull::<Self::PointedType>::new(x).unwrap()
    }
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_TestEnumClass_init(
    env: *mut JNIEnv,
    _: jclass,
) -> jlong {
    let this: Moo = Moo::default();
    let this: Box<Moo> = Box::new(this);
    let this: *mut Moo = Box::into_raw(this);
    this as jlong
}
impl<T: SwigForeignCLikeEnum> SwigFrom<jint> for T {
    fn swig_from(x: jint, _: *mut JNIEnv) -> T {
        T::from_jint(x)
    }
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_TestEnumClass_do_1f1(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
    v: jint,
) -> jint {
    let mut v: MyEnum = <MyEnum>::swig_from(v, env);
    let this: &mut Moo = unsafe { jlong_to_pointer::<Moo>(this).as_mut().unwrap() };
    let mut ret: i32 = Moo::f1(this, v);
    let ret: jint = ret;
    ret
}
impl<T: SwigForeignCLikeEnum> SwigFrom<T> for jint {
    fn swig_from(x: T, _: *mut JNIEnv) -> jint {
        x.as_jint()
    }
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_TestEnumClass_do_1next_1enum(
    env: *mut JNIEnv,
    _: jclass,
    v: jint,
) -> jint {
    let mut v: MyEnum = <MyEnum>::swig_from(v, env);
    let mut ret: MyEnum = Moo::next_enum(v);
    let mut ret: jint = <jint>::swig_from(ret, env);
    ret
}
#[doc = ""]
impl SwigFrom<jobject> for Box<EnumObserver> {
    fn swig_from(this: jobject, env: *mut JNIEnv) -> Self {
        let mut cb = JavaCallback::new(this, env);
        cb.methods.reserve(1);
        let class = unsafe { (**env).GetObjectClass.unwrap()(env, cb.this) };
        assert!(
            !class.is_null(),
            "GetObjectClass return null class for EnumObserver"
        );
        let method_id: jmethodID = unsafe {
            (**env).GetMethodID.unwrap()(
                env,
                class,
                swig_c_str!("onStateUpdate"),
                swig_c_str!("(Lnet/akaame/myapplication/generated/rust_jni_interface/MyEnum;Z)V"),
            )
        };
        assert!(!method_id.is_null(), "Can not find onStateUpdate id");
        cb.methods.push(method_id);
        Box::new(cb)
    }
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_TestEnumClass_call_1cb(
    env: *mut JNIEnv,
    _: jclass,
    cb: jobject,
) -> () {
    let mut cb: Box<EnumObserver> = <Box<EnumObserver>>::swig_from(cb, env);
    let mut ret: () = {
        let mut state = false;
        for e in &[MyEnum::Item1, MyEnum::Item2, MyEnum::Item3] {
            cb.on_state_changed(*e, state);
            state = !state;
        }
    };
    ret
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_TestEnumClass_do_1delete(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) {
    let this: *mut Moo = unsafe { jlong_to_pointer::<Moo>(this).as_mut().unwrap() };
    let this: Box<Moo> = unsafe { Box::from_raw(this) };
    drop(this);
}
impl OnEvent for JavaCallback {
    #[allow(unused_mut)]
    fn something_change(&self, a0: i32, a1: &str) {
        swig_assert_eq_size!(::std::os::raw::c_uint, u32);
        swig_assert_eq_size!(::std::os::raw::c_int, i32);
        let env = self.get_jni_env();
        if let Some(env) = env.env {
            let a0: jint = a0;
            let mut a1: jstring = <jstring>::swig_from(a1, env);
            unsafe {
                (**env).CallVoidMethod.unwrap()(env, self.this, self.methods[0usize], a0, a1);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    log::error!(concat!(
                        stringify!(something_change),
                        ": java throw exception"
                    ));
                    (**env).ExceptionDescribe.unwrap()(env);
                    (**env).ExceptionClear.unwrap()(env);
                }
            };
        }
    }
}
impl SwigForeignClass for Observable {
    type PointedType = Observable;
    fn jni_class() -> jclass {
        swig_jni_find_class!(
            FOREIGN_CLASS_OBSERVABLE,
            "net/akaame/myapplication/generated/rust_jni_interface/Observable"
        )
    }
    fn jni_class_pointer_field() -> jfieldID {
        swig_jni_get_field_id!(
            FOREIGN_CLASS_OBSERVABLE_MNATIVEOBJ_FIELD,
            FOREIGN_CLASS_OBSERVABLE,
            "mNativeObj",
            "J"
        )
    }
    fn box_object(this: Self) -> jlong {
        let this: Box<Observable> = Box::new(this);
        let this: *mut Observable = Box::into_raw(this);
        this as jlong
    }
    fn unbox_object(x: jlong) -> Self {
        let x: *mut Observable = unsafe { jlong_to_pointer::<Observable>(x).as_mut().unwrap() };
        let x: Box<Observable> = unsafe { Box::from_raw(x) };
        let x: Observable = *x;
        x
    }
    fn to_pointer(x: jlong) -> ::std::ptr::NonNull<Self::PointedType> {
        let x: *mut Observable = unsafe { jlong_to_pointer::<Observable>(x).as_mut().unwrap() };
        ::std::ptr::NonNull::<Self::PointedType>::new(x).unwrap()
    }
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Observable_init(
    env: *mut JNIEnv,
    _: jclass,
) -> jlong {
    let this: Observable = Observable::default();
    let this: Box<Observable> = Box::new(this);
    let this: *mut Observable = Box::into_raw(this);
    this as jlong
}
#[doc = ""]
impl SwigFrom<jobject> for Box<OnEvent> {
    fn swig_from(this: jobject, env: *mut JNIEnv) -> Self {
        let mut cb = JavaCallback::new(this, env);
        cb.methods.reserve(1);
        let class = unsafe { (**env).GetObjectClass.unwrap()(env, cb.this) };
        assert!(
            !class.is_null(),
            "GetObjectClass return null class for MyObserver"
        );
        let method_id: jmethodID = unsafe {
            (**env).GetMethodID.unwrap()(
                env,
                class,
                swig_c_str!("onStateChanged"),
                swig_c_str!("(ILjava/lang/String;)V"),
            )
        };
        assert!(!method_id.is_null(), "Can not find onStateChanged id");
        cb.methods.push(method_id);
        Box::new(cb)
    }
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Observable_do_1subscribe(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
    a0: jobject,
) -> () {
    let mut a0: Box<OnEvent> = <Box<OnEvent>>::swig_from(a0, env);
    let this: &mut Observable = unsafe { jlong_to_pointer::<Observable>(this).as_mut().unwrap() };
    let mut ret: () = Observable::subscribe(this, a0);
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Observable_do_1change(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
    a0: jint,
    a1: jstring,
) -> () {
    let a0: i32 = a0;
    let mut a1: JavaString = a1.swig_into(env);
    let mut a1: &str = a1.swig_deref();
    let this: &Observable = unsafe { jlong_to_pointer::<Observable>(this).as_mut().unwrap() };
    let mut ret: () = Observable::change(this, a0, a1);
    ret
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_net_akaame_myapplication_generated_rust_1jni_1interface_Observable_do_1delete(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) {
    let this: *mut Observable = unsafe { jlong_to_pointer::<Observable>(this).as_mut().unwrap() };
    let this: Box<Observable> = unsafe { Box::from_raw(this) };
    drop(this);
}
static mut JAVA_UTIL_OPTIONAL_LONG: jclass = ::std::ptr::null_mut();
static mut JAVA_UTIL_OPTIONAL_LONG_OF: jmethodID = ::std::ptr::null_mut();
static mut JAVA_UTIL_OPTIONAL_LONG_EMPTY: jmethodID = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_STRINGHOLDER: jclass = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_STRINGHOLDER_MNATIVEOBJ_FIELD: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_SESSION: jclass = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_SESSION_MNATIVEOBJ_FIELD: jfieldID = ::std::ptr::null_mut();
static mut JAVA_LANG_FLOAT: jclass = ::std::ptr::null_mut();
static mut JAVA_LANG_FLOAT_FLOAT_VALUE: jmethodID = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_BOO: jclass = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_BOO_MNATIVEOBJ_FIELD: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_TESTMETHODNOTMETHOD: jclass = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_TESTMETHODNOTMETHOD_MNATIVEOBJ_FIELD: jfieldID = ::std::ptr::null_mut();
static mut JAVA_LANG_SHORT: jclass = ::std::ptr::null_mut();
static mut JAVA_LANG_SHORT_SHORT_VALUE: jmethodID = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_TESTPATHANDRESULT: jclass = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_TESTPATHANDRESULT_MNATIVEOBJ_FIELD: jfieldID = ::std::ptr::null_mut();
static mut JAVA_UTIL_OPTIONAL_INT: jclass = ::std::ptr::null_mut();
static mut JAVA_UTIL_OPTIONAL_INT_OF: jmethodID = ::std::ptr::null_mut();
static mut JAVA_UTIL_OPTIONAL_INT_EMPTY: jmethodID = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_TESTENUMCLASS: jclass = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_TESTENUMCLASS_MNATIVEOBJ_FIELD: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_FOO: jclass = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_FOO_MNATIVEOBJ_FIELD: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_INTERFACE: jclass = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_INTERFACE_MNATIVEOBJ_FIELD: jfieldID = ::std::ptr::null_mut();
static mut JAVA_LANG_LONG: jclass = ::std::ptr::null_mut();
static mut JAVA_LANG_LONG_LONG_VALUE: jmethodID = ::std::ptr::null_mut();
static mut JAVA_LANG_EXCEPTION: jclass = ::std::ptr::null_mut();
static mut JAVA_LANG_BYTE: jclass = ::std::ptr::null_mut();
static mut JAVA_LANG_BYTE_BYTE_VALUE: jmethodID = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_OBSERVABLE: jclass = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_OBSERVABLE_MNATIVEOBJ_FIELD: jfieldID = ::std::ptr::null_mut();
static mut JAVA_UTIL_OPTIONAL_DOUBLE: jclass = ::std::ptr::null_mut();
static mut JAVA_UTIL_OPTIONAL_DOUBLE_OF: jmethodID = ::std::ptr::null_mut();
static mut JAVA_UTIL_OPTIONAL_DOUBLE_EMPTY: jmethodID = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_CONFIG: jclass = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_CONFIG_MNATIVEOBJ_FIELD: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_BOOLEANHOLDER: jclass = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_BOOLEANHOLDER_MNATIVEOBJ_FIELD: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_MYENUM: jclass = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_MYENUM_ITEM1: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_MYENUM_ITEM2: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_MYENUM_ITEM3: jfieldID = ::std::ptr::null_mut();
static mut JAVA_LANG_INTEGER: jclass = ::std::ptr::null_mut();
static mut JAVA_LANG_INTEGER_INT_VALUE: jmethodID = ::std::ptr::null_mut();
static mut JAVA_LANG_DOUBLE: jclass = ::std::ptr::null_mut();
static mut JAVA_LANG_DOUBLE_DOUBLE_VALUE_METHOD: jmethodID = ::std::ptr::null_mut();
#[no_mangle]
pub extern "system" fn JNI_OnLoad(
    java_vm: *mut JavaVM,
    _reserved: *mut ::std::os::raw::c_void,
) -> jint {
    println!("JNI_OnLoad begin");
    assert!(!java_vm.is_null());
    let mut env: *mut JNIEnv = ::std::ptr::null_mut();
    let res = unsafe {
        (**java_vm).GetEnv.unwrap()(
            java_vm,
            (&mut env) as *mut *mut JNIEnv as *mut *mut ::std::os::raw::c_void,
            SWIG_JNI_VERSION,
        )
    };
    if res != (JNI_OK as jint) {
        panic!("JNI GetEnv in JNI_OnLoad failed, return code {}", res);
    }
    assert!(!env.is_null());
    unsafe {
        let class_local_ref =
            (**env).FindClass.unwrap()(env, swig_c_str!("java/util/OptionalLong"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/util/OptionalLong")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/util/OptionalLong")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_UTIL_OPTIONAL_LONG = class;
        let method_id: jmethodID = (**env).GetStaticMethodID.unwrap()(
            env,
            class,
            swig_c_str!("of"),
            swig_c_str!("(J)Ljava/util/OptionalLong;"),
        );
        assert!(
            !method_id.is_null(),
            concat!(
                "GetStaticMethodID for class ",
                "java/util/OptionalLong",
                " method ",
                "of",
                " sig ",
                "(J)Ljava/util/OptionalLong;",
                " failed"
            )
        );
        JAVA_UTIL_OPTIONAL_LONG_OF = method_id;
        let method_id: jmethodID = (**env).GetStaticMethodID.unwrap()(
            env,
            class,
            swig_c_str!("empty"),
            swig_c_str!("()Ljava/util/OptionalLong;"),
        );
        assert!(
            !method_id.is_null(),
            concat!(
                "GetStaticMethodID for class ",
                "java/util/OptionalLong",
                " method ",
                "empty",
                " sig ",
                "()Ljava/util/OptionalLong;",
                " failed"
            )
        );
        JAVA_UTIL_OPTIONAL_LONG_EMPTY = method_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(
            env,
            swig_c_str!("net/akaame/myapplication/generated/rust_jni_interface/StringHolder"),
        );
        assert!(
            !class_local_ref.is_null(),
            concat!(
                "FindClass failed for ",
                "net/akaame/myapplication/generated/rust_jni_interface/StringHolder"
            )
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!(
                "FindClass failed for ",
                "net/akaame/myapplication/generated/rust_jni_interface/StringHolder"
            )
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        FOREIGN_CLASS_STRINGHOLDER = class;
        let field_id: jfieldID =
            (**env).GetFieldID.unwrap()(env, class, swig_c_str!("mNativeObj"), swig_c_str!("J"));
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "net/akaame/myapplication/generated/rust_jni_interface/StringHolder",
                " method ",
                "mNativeObj",
                " sig ",
                "J",
                " failed"
            )
        );
        FOREIGN_CLASS_STRINGHOLDER_MNATIVEOBJ_FIELD = field_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(
            env,
            swig_c_str!("net/akaame/myapplication/generated/rust_jni_interface/Session"),
        );
        assert!(
            !class_local_ref.is_null(),
            concat!(
                "FindClass failed for ",
                "net/akaame/myapplication/generated/rust_jni_interface/Session"
            )
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!(
                "FindClass failed for ",
                "net/akaame/myapplication/generated/rust_jni_interface/Session"
            )
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        FOREIGN_CLASS_SESSION = class;
        let field_id: jfieldID =
            (**env).GetFieldID.unwrap()(env, class, swig_c_str!("mNativeObj"), swig_c_str!("J"));
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "net/akaame/myapplication/generated/rust_jni_interface/Session",
                " method ",
                "mNativeObj",
                " sig ",
                "J",
                " failed"
            )
        );
        FOREIGN_CLASS_SESSION_MNATIVEOBJ_FIELD = field_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(env, swig_c_str!("java/lang/Float"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/lang/Float")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/lang/Float")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_LANG_FLOAT = class;
        let method_id: jmethodID =
            (**env).GetMethodID.unwrap()(env, class, swig_c_str!("floatValue"), swig_c_str!("()F"));
        assert!(
            !method_id.is_null(),
            concat!(
                "GetMethodID for class ",
                "java/lang/Float",
                " method ",
                "floatValue",
                " sig ",
                "()F",
                " failed"
            )
        );
        JAVA_LANG_FLOAT_FLOAT_VALUE = method_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(
            env,
            swig_c_str!("net/akaame/myapplication/generated/rust_jni_interface/Boo"),
        );
        assert!(
            !class_local_ref.is_null(),
            concat!(
                "FindClass failed for ",
                "net/akaame/myapplication/generated/rust_jni_interface/Boo"
            )
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!(
                "FindClass failed for ",
                "net/akaame/myapplication/generated/rust_jni_interface/Boo"
            )
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        FOREIGN_CLASS_BOO = class;
        let field_id: jfieldID =
            (**env).GetFieldID.unwrap()(env, class, swig_c_str!("mNativeObj"), swig_c_str!("J"));
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "net/akaame/myapplication/generated/rust_jni_interface/Boo",
                " method ",
                "mNativeObj",
                " sig ",
                "J",
                " failed"
            )
        );
        FOREIGN_CLASS_BOO_MNATIVEOBJ_FIELD = field_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(
            env,
            swig_c_str!(
                "net/akaame/myapplication/generated/rust_jni_interface/TestMethodNotMethod"
            ),
        );
        assert!(
            !class_local_ref.is_null(),
            concat!(
                "FindClass failed for ",
                "net/akaame/myapplication/generated/rust_jni_interface/TestMethodNotMethod"
            )
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!(
                "FindClass failed for ",
                "net/akaame/myapplication/generated/rust_jni_interface/TestMethodNotMethod"
            )
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        FOREIGN_CLASS_TESTMETHODNOTMETHOD = class;
        let field_id: jfieldID =
            (**env).GetFieldID.unwrap()(env, class, swig_c_str!("mNativeObj"), swig_c_str!("J"));
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "net/akaame/myapplication/generated/rust_jni_interface/TestMethodNotMethod",
                " method ",
                "mNativeObj",
                " sig ",
                "J",
                " failed"
            )
        );
        FOREIGN_CLASS_TESTMETHODNOTMETHOD_MNATIVEOBJ_FIELD = field_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(env, swig_c_str!("java/lang/Short"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/lang/Short")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/lang/Short")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_LANG_SHORT = class;
        let method_id: jmethodID =
            (**env).GetMethodID.unwrap()(env, class, swig_c_str!("shortValue"), swig_c_str!("()S"));
        assert!(
            !method_id.is_null(),
            concat!(
                "GetMethodID for class ",
                "java/lang/Short",
                " method ",
                "shortValue",
                " sig ",
                "()S",
                " failed"
            )
        );
        JAVA_LANG_SHORT_SHORT_VALUE = method_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(
            env,
            swig_c_str!("net/akaame/myapplication/generated/rust_jni_interface/TestPathAndResult"),
        );
        assert!(
            !class_local_ref.is_null(),
            concat!(
                "FindClass failed for ",
                "net/akaame/myapplication/generated/rust_jni_interface/TestPathAndResult"
            )
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!(
                "FindClass failed for ",
                "net/akaame/myapplication/generated/rust_jni_interface/TestPathAndResult"
            )
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        FOREIGN_CLASS_TESTPATHANDRESULT = class;
        let field_id: jfieldID =
            (**env).GetFieldID.unwrap()(env, class, swig_c_str!("mNativeObj"), swig_c_str!("J"));
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "net/akaame/myapplication/generated/rust_jni_interface/TestPathAndResult",
                " method ",
                "mNativeObj",
                " sig ",
                "J",
                " failed"
            )
        );
        FOREIGN_CLASS_TESTPATHANDRESULT_MNATIVEOBJ_FIELD = field_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(env, swig_c_str!("java/util/OptionalInt"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/util/OptionalInt")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/util/OptionalInt")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_UTIL_OPTIONAL_INT = class;
        let method_id: jmethodID = (**env).GetStaticMethodID.unwrap()(
            env,
            class,
            swig_c_str!("of"),
            swig_c_str!("(I)Ljava/util/OptionalInt;"),
        );
        assert!(
            !method_id.is_null(),
            concat!(
                "GetStaticMethodID for class ",
                "java/util/OptionalInt",
                " method ",
                "of",
                " sig ",
                "(I)Ljava/util/OptionalInt;",
                " failed"
            )
        );
        JAVA_UTIL_OPTIONAL_INT_OF = method_id;
        let method_id: jmethodID = (**env).GetStaticMethodID.unwrap()(
            env,
            class,
            swig_c_str!("empty"),
            swig_c_str!("()Ljava/util/OptionalInt;"),
        );
        assert!(
            !method_id.is_null(),
            concat!(
                "GetStaticMethodID for class ",
                "java/util/OptionalInt",
                " method ",
                "empty",
                " sig ",
                "()Ljava/util/OptionalInt;",
                " failed"
            )
        );
        JAVA_UTIL_OPTIONAL_INT_EMPTY = method_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(
            env,
            swig_c_str!("net/akaame/myapplication/generated/rust_jni_interface/TestEnumClass"),
        );
        assert!(
            !class_local_ref.is_null(),
            concat!(
                "FindClass failed for ",
                "net/akaame/myapplication/generated/rust_jni_interface/TestEnumClass"
            )
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!(
                "FindClass failed for ",
                "net/akaame/myapplication/generated/rust_jni_interface/TestEnumClass"
            )
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        FOREIGN_CLASS_TESTENUMCLASS = class;
        let field_id: jfieldID =
            (**env).GetFieldID.unwrap()(env, class, swig_c_str!("mNativeObj"), swig_c_str!("J"));
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "net/akaame/myapplication/generated/rust_jni_interface/TestEnumClass",
                " method ",
                "mNativeObj",
                " sig ",
                "J",
                " failed"
            )
        );
        FOREIGN_CLASS_TESTENUMCLASS_MNATIVEOBJ_FIELD = field_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(
            env,
            swig_c_str!("net/akaame/myapplication/generated/rust_jni_interface/Foo"),
        );
        assert!(
            !class_local_ref.is_null(),
            concat!(
                "FindClass failed for ",
                "net/akaame/myapplication/generated/rust_jni_interface/Foo"
            )
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!(
                "FindClass failed for ",
                "net/akaame/myapplication/generated/rust_jni_interface/Foo"
            )
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        FOREIGN_CLASS_FOO = class;
        let field_id: jfieldID =
            (**env).GetFieldID.unwrap()(env, class, swig_c_str!("mNativeObj"), swig_c_str!("J"));
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "net/akaame/myapplication/generated/rust_jni_interface/Foo",
                " method ",
                "mNativeObj",
                " sig ",
                "J",
                " failed"
            )
        );
        FOREIGN_CLASS_FOO_MNATIVEOBJ_FIELD = field_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(
            env,
            swig_c_str!("net/akaame/myapplication/generated/rust_jni_interface/Interface"),
        );
        assert!(
            !class_local_ref.is_null(),
            concat!(
                "FindClass failed for ",
                "net/akaame/myapplication/generated/rust_jni_interface/Interface"
            )
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!(
                "FindClass failed for ",
                "net/akaame/myapplication/generated/rust_jni_interface/Interface"
            )
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        FOREIGN_CLASS_INTERFACE = class;
        let field_id: jfieldID =
            (**env).GetFieldID.unwrap()(env, class, swig_c_str!("mNativeObj"), swig_c_str!("J"));
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "net/akaame/myapplication/generated/rust_jni_interface/Interface",
                " method ",
                "mNativeObj",
                " sig ",
                "J",
                " failed"
            )
        );
        FOREIGN_CLASS_INTERFACE_MNATIVEOBJ_FIELD = field_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(env, swig_c_str!("java/lang/Long"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/lang/Long")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/lang/Long")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_LANG_LONG = class;
        let method_id: jmethodID =
            (**env).GetMethodID.unwrap()(env, class, swig_c_str!("longValue"), swig_c_str!("()J"));
        assert!(
            !method_id.is_null(),
            concat!(
                "GetMethodID for class ",
                "java/lang/Long",
                " method ",
                "longValue",
                " sig ",
                "()J",
                " failed"
            )
        );
        JAVA_LANG_LONG_LONG_VALUE = method_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(env, swig_c_str!("java/lang/Exception"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/lang/Exception")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/lang/Exception")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_LANG_EXCEPTION = class;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(env, swig_c_str!("java/lang/Byte"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/lang/Byte")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/lang/Byte")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_LANG_BYTE = class;
        let method_id: jmethodID =
            (**env).GetMethodID.unwrap()(env, class, swig_c_str!("byteValue"), swig_c_str!("()B"));
        assert!(
            !method_id.is_null(),
            concat!(
                "GetMethodID for class ",
                "java/lang/Byte",
                " method ",
                "byteValue",
                " sig ",
                "()B",
                " failed"
            )
        );
        JAVA_LANG_BYTE_BYTE_VALUE = method_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(
            env,
            swig_c_str!("net/akaame/myapplication/generated/rust_jni_interface/Observable"),
        );
        assert!(
            !class_local_ref.is_null(),
            concat!(
                "FindClass failed for ",
                "net/akaame/myapplication/generated/rust_jni_interface/Observable"
            )
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!(
                "FindClass failed for ",
                "net/akaame/myapplication/generated/rust_jni_interface/Observable"
            )
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        FOREIGN_CLASS_OBSERVABLE = class;
        let field_id: jfieldID =
            (**env).GetFieldID.unwrap()(env, class, swig_c_str!("mNativeObj"), swig_c_str!("J"));
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "net/akaame/myapplication/generated/rust_jni_interface/Observable",
                " method ",
                "mNativeObj",
                " sig ",
                "J",
                " failed"
            )
        );
        FOREIGN_CLASS_OBSERVABLE_MNATIVEOBJ_FIELD = field_id;
    }
    unsafe {
        let class_local_ref =
            (**env).FindClass.unwrap()(env, swig_c_str!("java/util/OptionalDouble"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/util/OptionalDouble")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/util/OptionalDouble")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_UTIL_OPTIONAL_DOUBLE = class;
        let method_id: jmethodID = (**env).GetStaticMethodID.unwrap()(
            env,
            class,
            swig_c_str!("of"),
            swig_c_str!("(D)Ljava/util/OptionalDouble;"),
        );
        assert!(
            !method_id.is_null(),
            concat!(
                "GetStaticMethodID for class ",
                "java/util/OptionalDouble",
                " method ",
                "of",
                " sig ",
                "(D)Ljava/util/OptionalDouble;",
                " failed"
            )
        );
        JAVA_UTIL_OPTIONAL_DOUBLE_OF = method_id;
        let method_id: jmethodID = (**env).GetStaticMethodID.unwrap()(
            env,
            class,
            swig_c_str!("empty"),
            swig_c_str!("()Ljava/util/OptionalDouble;"),
        );
        assert!(
            !method_id.is_null(),
            concat!(
                "GetStaticMethodID for class ",
                "java/util/OptionalDouble",
                " method ",
                "empty",
                " sig ",
                "()Ljava/util/OptionalDouble;",
                " failed"
            )
        );
        JAVA_UTIL_OPTIONAL_DOUBLE_EMPTY = method_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(
            env,
            swig_c_str!("net/akaame/myapplication/generated/rust_jni_interface/Config"),
        );
        assert!(
            !class_local_ref.is_null(),
            concat!(
                "FindClass failed for ",
                "net/akaame/myapplication/generated/rust_jni_interface/Config"
            )
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!(
                "FindClass failed for ",
                "net/akaame/myapplication/generated/rust_jni_interface/Config"
            )
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        FOREIGN_CLASS_CONFIG = class;
        let field_id: jfieldID =
            (**env).GetFieldID.unwrap()(env, class, swig_c_str!("mNativeObj"), swig_c_str!("J"));
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "net/akaame/myapplication/generated/rust_jni_interface/Config",
                " method ",
                "mNativeObj",
                " sig ",
                "J",
                " failed"
            )
        );
        FOREIGN_CLASS_CONFIG_MNATIVEOBJ_FIELD = field_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(
            env,
            swig_c_str!("net/akaame/myapplication/generated/rust_jni_interface/BooleanHolder"),
        );
        assert!(
            !class_local_ref.is_null(),
            concat!(
                "FindClass failed for ",
                "net/akaame/myapplication/generated/rust_jni_interface/BooleanHolder"
            )
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!(
                "FindClass failed for ",
                "net/akaame/myapplication/generated/rust_jni_interface/BooleanHolder"
            )
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        FOREIGN_CLASS_BOOLEANHOLDER = class;
        let field_id: jfieldID =
            (**env).GetFieldID.unwrap()(env, class, swig_c_str!("mNativeObj"), swig_c_str!("J"));
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "net/akaame/myapplication/generated/rust_jni_interface/BooleanHolder",
                " method ",
                "mNativeObj",
                " sig ",
                "J",
                " failed"
            )
        );
        FOREIGN_CLASS_BOOLEANHOLDER_MNATIVEOBJ_FIELD = field_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(
            env,
            swig_c_str!("net/akaame/myapplication/generated/rust_jni_interface/MyEnum"),
        );
        assert!(
            !class_local_ref.is_null(),
            concat!(
                "FindClass failed for ",
                "net/akaame/myapplication/generated/rust_jni_interface/MyEnum"
            )
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!(
                "FindClass failed for ",
                "net/akaame/myapplication/generated/rust_jni_interface/MyEnum"
            )
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        FOREIGN_ENUM_MYENUM = class;
        let field_id: jfieldID = (**env).GetStaticFieldID.unwrap()(
            env,
            class,
            swig_c_str!("ITEM1"),
            swig_c_str!("Lnet/akaame/myapplication/generated/rust_jni_interface/MyEnum;"),
        );
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "net/akaame/myapplication/generated/rust_jni_interface/MyEnum",
                " method ",
                "ITEM1",
                " sig ",
                "Lnet/akaame/myapplication/generated/rust_jni_interface/MyEnum;",
                " failed"
            )
        );
        FOREIGN_ENUM_MYENUM_ITEM1 = field_id;
        let field_id: jfieldID = (**env).GetStaticFieldID.unwrap()(
            env,
            class,
            swig_c_str!("ITEM2"),
            swig_c_str!("Lnet/akaame/myapplication/generated/rust_jni_interface/MyEnum;"),
        );
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "net/akaame/myapplication/generated/rust_jni_interface/MyEnum",
                " method ",
                "ITEM2",
                " sig ",
                "Lnet/akaame/myapplication/generated/rust_jni_interface/MyEnum;",
                " failed"
            )
        );
        FOREIGN_ENUM_MYENUM_ITEM2 = field_id;
        let field_id: jfieldID = (**env).GetStaticFieldID.unwrap()(
            env,
            class,
            swig_c_str!("ITEM3"),
            swig_c_str!("Lnet/akaame/myapplication/generated/rust_jni_interface/MyEnum;"),
        );
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "net/akaame/myapplication/generated/rust_jni_interface/MyEnum",
                " method ",
                "ITEM3",
                " sig ",
                "Lnet/akaame/myapplication/generated/rust_jni_interface/MyEnum;",
                " failed"
            )
        );
        FOREIGN_ENUM_MYENUM_ITEM3 = field_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(env, swig_c_str!("java/lang/Integer"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/lang/Integer")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/lang/Integer")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_LANG_INTEGER = class;
        let method_id: jmethodID =
            (**env).GetMethodID.unwrap()(env, class, swig_c_str!("intValue"), swig_c_str!("()I"));
        assert!(
            !method_id.is_null(),
            concat!(
                "GetMethodID for class ",
                "java/lang/Integer",
                " method ",
                "intValue",
                " sig ",
                "()I",
                " failed"
            )
        );
        JAVA_LANG_INTEGER_INT_VALUE = method_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(env, swig_c_str!("java/lang/Double"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/lang/Double")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/lang/Double")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_LANG_DOUBLE = class;
        let method_id: jmethodID = (**env).GetMethodID.unwrap()(
            env,
            class,
            swig_c_str!("doubleValue"),
            swig_c_str!("()D"),
        );
        assert!(
            !method_id.is_null(),
            concat!(
                "GetMethodID for class ",
                "java/lang/Double",
                " method ",
                "doubleValue",
                " sig ",
                "()D",
                " failed"
            )
        );
        JAVA_LANG_DOUBLE_DOUBLE_VALUE_METHOD = method_id;
    }
    SWIG_JNI_VERSION
}
#[no_mangle]
pub extern "system" fn JNI_OnUnload(java_vm: *mut JavaVM, _reserved: *mut ::std::os::raw::c_void) {
    println!("JNI_OnUnLoad begin");
    assert!(!java_vm.is_null());
    let mut env: *mut JNIEnv = ::std::ptr::null_mut();
    let res = unsafe {
        (**java_vm).GetEnv.unwrap()(
            java_vm,
            (&mut env) as *mut *mut JNIEnv as *mut *mut ::std::os::raw::c_void,
            SWIG_JNI_VERSION,
        )
    };
    if res != (JNI_OK as jint) {
        panic!("JNI GetEnv in JNI_OnLoad failed, return code {}", res);
    }
    assert!(!env.is_null());
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_UTIL_OPTIONAL_LONG);
        JAVA_UTIL_OPTIONAL_LONG = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, FOREIGN_CLASS_STRINGHOLDER);
        FOREIGN_CLASS_STRINGHOLDER = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, FOREIGN_CLASS_SESSION);
        FOREIGN_CLASS_SESSION = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_LANG_FLOAT);
        JAVA_LANG_FLOAT = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, FOREIGN_CLASS_BOO);
        FOREIGN_CLASS_BOO = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, FOREIGN_CLASS_TESTMETHODNOTMETHOD);
        FOREIGN_CLASS_TESTMETHODNOTMETHOD = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_LANG_SHORT);
        JAVA_LANG_SHORT = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, FOREIGN_CLASS_TESTPATHANDRESULT);
        FOREIGN_CLASS_TESTPATHANDRESULT = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_UTIL_OPTIONAL_INT);
        JAVA_UTIL_OPTIONAL_INT = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, FOREIGN_CLASS_TESTENUMCLASS);
        FOREIGN_CLASS_TESTENUMCLASS = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, FOREIGN_CLASS_FOO);
        FOREIGN_CLASS_FOO = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, FOREIGN_CLASS_INTERFACE);
        FOREIGN_CLASS_INTERFACE = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_LANG_LONG);
        JAVA_LANG_LONG = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_LANG_EXCEPTION);
        JAVA_LANG_EXCEPTION = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_LANG_BYTE);
        JAVA_LANG_BYTE = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, FOREIGN_CLASS_OBSERVABLE);
        FOREIGN_CLASS_OBSERVABLE = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_UTIL_OPTIONAL_DOUBLE);
        JAVA_UTIL_OPTIONAL_DOUBLE = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, FOREIGN_CLASS_CONFIG);
        FOREIGN_CLASS_CONFIG = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, FOREIGN_CLASS_BOOLEANHOLDER);
        FOREIGN_CLASS_BOOLEANHOLDER = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, FOREIGN_ENUM_MYENUM);
        FOREIGN_ENUM_MYENUM = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_LANG_INTEGER);
        JAVA_LANG_INTEGER = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_LANG_DOUBLE);
        JAVA_LANG_DOUBLE = ::std::ptr::null_mut()
    }
}
