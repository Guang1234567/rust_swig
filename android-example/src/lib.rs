#![allow(
clippy::enum_variant_names,
clippy::unused_unit,
clippy::let_and_return,
clippy::not_unsafe_ptr_arg_deref,
clippy::cast_lossless,
clippy::blacklisted_name,
clippy::too_many_arguments,
clippy::trivially_copy_pass_by_ref,
clippy::let_unit_value,
clippy::clone_on_copy
)]
#![allow(non_upper_case_globals, dead_code, non_camel_case_types, improper_ctypes, non_snake_case, unused_imports, bare_trait_objects)]

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use diesel::sql_types::Bool;
use log::{trace, debug, info, warn, error};

#[cfg(target_os = "android")]
use load_dotenv::{load_dotenv, load_dotenv_from_filename};

use crate::android_c_headers::*;
use crate::java_glue::*;

// load your .env.android file at compile time
load_dotenv_from_filename!(".env.android");

mod android_c_headers;
#[macro_use]
mod java_glue;

// ANCHOR: rust_code

struct BooleanHolder {
    value: bool,
}

impl BooleanHolder {
    fn new() -> BooleanHolder {
        BooleanHolder { value: false }
    }

    fn set(&mut self, v: bool) {
        self.value = v;
    }

    fn get(&self) -> bool {
        self.value
    }
}

struct StringHolder {
    value: String,
}

impl StringHolder {
    fn new() -> StringHolder {
        StringHolder { value: "".to_owned() }
    }

    fn set(&mut self, v: String) {
        self.value = v;
    }

    fn get(&self) -> &str {
        &self.value
    }
}

trait BuildList {
    fn hasNext(&self) -> BooleanHolder;

    fn next(&self) -> StringHolder;
}

//----------------------------

struct Config {
    name: String,
}

impl Config {
    pub fn new(name: String) -> Config {
        Config { name }
    }
}

struct Session {
    a: i32,
    cfg: Config,
}

impl Session {
    pub fn new(cfg: Config) -> Session {
        #[cfg(target_os = "android")]
            android_logger::init_once(
            android_logger::Config::default()
                .with_min_level(log::Level::Debug)
                .with_tag("Hello"),
        );
        log_panics::init(); // log panics rather than printing them
        info!("init log system - done");

        info!("run config - {}", cfg.name);

        Session { a: 2, cfg }
    }

    pub fn add_and1(&self, val: i32) -> i32 {
        self.a + val + 1
    }

    // Greeting with full, no-runtime-cost support for newlines and UTF-8
    pub fn greet(to: &str) -> String {
        format!("Hello {} ✋\nIt's a pleasure to meet you!", to)
    }

    pub fn f(&self, _: i32, _: i32) -> i32 {
        123
    }

    pub fn inputList(&self, list: Box<BuildList>) {
        info!(">------------------- begin iterator -------------------");
        info!("<------------------- end iterator -------------------");
    }
}

//----------------------------

#[derive(Clone, Copy)]
enum MyEnum {
    Item1,
    Item2,
    Item3,
}

//----------------------------

#[derive(Default)]
struct Moo;

impl Moo {
    fn f1(&mut self, v: MyEnum) -> i32 {
        if let MyEnum::Item2 = v {
            17
        } else {
            -5
        }
    }

    fn next_enum(v: MyEnum) -> MyEnum {
        use MyEnum::*;
        match v {
            Item1 => Item2,
            Item2 => Item3,
            Item3 => Item1,
        }
    }
}

trait EnumObserver {
    fn on_state_changed(&self, item: MyEnum, is_ok: bool);
}


//----------------------------

trait OnEvent {
    fn something_change(&self, x: i32, s: &str);
}

#[derive(Default)]
struct Observable {
    observers: Vec<Box<OnEvent>>,
}

impl Observable {
    fn subscribe(&mut self, cb: Box<OnEvent>) {
        self.observers.push(cb);
    }
    fn change(&self, x: i32, s: &str) {
        debug!("Observable::change x {}, s {}", x, s);
        for cb in &self.observers {
            cb.something_change(x, s);
        }
    }
}

//----------------------------

pub trait Interface {
    fn f(&self, _: i32) -> i32;
    fn set(&mut self, _: i32);
}

struct InterfaceImpl {
    base: i32,
}

impl Interface for InterfaceImpl {
    fn f(&self, x: i32) -> i32 {
        self.base + x
    }
    fn set(&mut self, x: i32) {
        self.base = x;
    }
}

fn create_interface() -> Box<Box<Interface>> {
    Box::new(Box::new(InterfaceImpl { base: 17 }))
}

//----------------------------

pub struct TestMethodNotMethod;

impl TestMethodNotMethod {
    fn new() -> Self {
        TestMethodNotMethod
    }
}

fn method_not_method(_this: &TestMethodNotMethod) {}

#[derive(Clone)]
pub struct Foo {
    data: i32,
    name: String,
}

//----------------------------

impl Foo {
    fn new(val: i32, name: &str) -> Foo {
        println!("Foo::new {}  {}", val, name);
        Foo {
            data: val,
            name: name.to_string(),
        }
    }

    fn f(&self, a: i32, b: i32) -> i32 {
        println!("Foo::f {} {} {}", self.data, a, b);
        self.data + a + b
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn f_double(&self, a: f64, b: f64) -> f64 {
        a.hypot(b) + self.data as f64
    }
}

struct Boo {
    a: i32,
}

impl Boo {
    fn new() -> Boo {
        Boo { a: 17 }
    }
    fn test(&self, a: bool) -> f32 {
        if a {
            ::std::f32::consts::E
        } else {
            ::std::f32::consts::PI
        }
    }
    fn set_a(&mut self, val: i32) {
        self.a = val;
    }
    fn get_a(&self) -> i32 {
        self.a
    }
}

fn create_boo() -> Rc<RefCell<Boo>> {
    Rc::new(RefCell::new(Boo::new()))
}

//----------------------------

struct TestPathAndResult {
    path: PathBuf,
    boo: Rc<RefCell<Boo>>,
}

impl TestPathAndResult {
    fn empty() -> Result<TestPathAndResult, String> {
        println!("TestPathAndResult::empty");
        Err("test error path".into())
    }
    fn new(path: &Path) -> Result<TestPathAndResult, String> {
        println!("TestPathAndResult::new path: {:?}", path);
        Ok(TestPathAndResult {
            path: path.into(),
            boo: create_boo(),
        })
    }

    fn get_path(&self) -> String {
        self.path.to_str().unwrap().into()
    }

    fn get_boo(&self) -> Rc<RefCell<Boo>> {
        self.boo.clone()
    }

    fn get_foo_list(&self) -> Vec<Foo> {
        let mut ret = Vec::new();
        for i in 0..10 {
            ret.push(Foo::new(i, &format!("foo arr: {}", i)));
        }
        ret
    }

    fn get_result_foo_list(generate_err: bool) -> Result<Vec<Foo>, String> {
        if !generate_err {
            let mut ret = Vec::new();
            for i in 0..10 {
                ret.push(Foo::new(i, &format!("foo arr: {}", i)));
            }
            Ok(ret)
        } else {
            Err("bad list".into())
        }
    }
}


#[no_mangle]
pub fn Java_com_example_rust_TestPathAndResult_do_1testHandArrayReturn(
    env: *mut JNIEnv,
    _: jclass,
    _: jlong,
) -> jobjectArray {
    let class_id = swig_c_str!("net/akaame/myapplication/generated/rust_jni_interface/Boo");
    let jcls: jclass = unsafe { (**env).FindClass.unwrap()(env, class_id) };
    if jcls.is_null() {
        panic!("Can not find class_id {:?}", class_id);
    }
    let n = 10;
    let ret: jobjectArray =
        unsafe { (**env).NewObjectArray.unwrap()(env, n, jcls, ::std::ptr::null_mut()) };
    if ret.is_null() {
        panic!("Can not create object array");
    }

    let field_id = swig_c_str!("mNativeObj");
    let type_id = swig_c_str!("J");
    let field_id: jfieldID = unsafe { (**env).GetFieldID.unwrap()(env, jcls, field_id, type_id) };
    assert!(!field_id.is_null());

    for i in 0..n {
        let elem: jobject = unsafe { (**env).AllocObject.unwrap()(env, jcls) };
        let boo: Rc<RefCell<Boo>> = create_boo();
        boo.borrow_mut().set_a(i);
        unsafe {
            (**env).SetLongField.unwrap()(env, elem, field_id, <Rc<RefCell<Boo>>>::box_object(boo));
            (**env).SetObjectArrayElement.unwrap()(env, ret, i, elem);
            (**env).DeleteLocalRef.unwrap()(env, elem);
        }
    }
    ret
}

//----------------------------

struct CircularDepsA {
    s: String,
}

impl CircularDepsA {
    fn java_new(s: &str) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(CircularDepsA { s: s.into() }))
    }

    fn a(&self, b: &CircularDepsB) -> String {
        let mut ret = self.s.clone();
        ret.push_str(&b.s);
        ret
    }
}

struct CircularDepsB {
    s: String,
}

impl CircularDepsB {
    fn new(a: f64) -> Self {
        CircularDepsB {
            s: format!("{}", a),
        }
    }
    fn b(&self, a: &CircularDepsA) -> String {
        let mut ret = self.s.clone();
        ret.push_str(&a.s);
        ret
    }
}

// ANCHOR_END: rust_code
