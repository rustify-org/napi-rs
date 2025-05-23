use napi::{JsFunction, Result};
use napi_derive::napi;

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

#[napi]
pub struct Animal {
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

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
pub fn hello(name: String) -> String {
  format!("hello {}", name)
}

#[napi(js_name = "highOrderFunction")]
pub fn high_order_function(
  a_arg: u32,
  #[napi(ts_arg_type = "(s: number)=>string")] _callback: JsFunction,
) -> u32 {
  a_arg + 1
}

#[napi]
pub fn get_env(env: String) -> Option<String> {
  std::env::var(env).ok()
}

#[napi]
pub fn say_a(s: Option<bool>) -> Result<bool> {
  match s {
    Some(v) => Ok(v),
    None => Err(napi::Error::new(napi::Status::GenericFailure, "s is None")),
  }
}

#[napi]
pub fn get_sum(nums: Vec<i32>) -> i32 {
  nums.iter().sum()
}
