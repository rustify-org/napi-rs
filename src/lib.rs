#![deny(clippy::all)]

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
