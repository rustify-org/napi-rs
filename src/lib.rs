#![deny(clippy::all)]

use napi::JsFunction;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
pub fn hello(name: String) -> String {
  format!("hello {}", name)
}

#[napi]
struct Animal {
  pub name: String,
  pub age: i32,
}

#[napi]
impl Animal {
  #[napi(constructor)]
  pub fn new(name: String, age: i32) -> Self {
    Animal { name, age }
  }
  #[napi]
  pub fn change_name(&mut self, new_name: String) {
    self.name = new_name;
  }
}

#[napi(string_enum)]
pub enum Kind {
  Dog,
  Cat,
}

#[napi(js_name = "coolFunction")]
pub fn transform_to_string(kind: Kind) -> String {
  match kind {
    Kind::Dog => "dog".to_string(),
    Kind::Cat => "cat".to_string(),
  }
}

// 高阶函数
#[napi(js_name = "highOrderFunction")]
pub fn high_order_function(
  a_arg: u32,
  #[napi(ts_arg_type = "(s: number)=>string")] callback: JsFunction,
) -> u32 {
  a_arg + 1
}

#[napi]
fn get_env(env: String) -> Option<String> {
  match std::env::var(env) {
    Ok(env) => Some(env),
    Err(_) => None,
  }
}
