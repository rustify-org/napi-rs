use napi_derive::napi;

// 构造函数
#[napi(constructor)]
pub struct Dog {
  pub name: String,
}

// 自定义构造函数
#[napi]
pub struct Cat {
  pub name: String,
}

#[napi]
impl Cat {
  #[napi(constructor)]
  pub fn new(name: String) -> Self {
    Cat { name }
  }
  // static修饰符
  #[napi(factory)]
  pub fn create(name: String) -> Self {
    Cat { name }
  }
  #[napi]
  pub fn say_name(&self) -> String {
    self.name.clone()
  }
  #[napi]
  pub fn set_name(&mut self, new_name: String) {
    self.name = new_name;
  }
  #[napi]
  pub fn get_name(&self) -> String {
    self.name.clone()
  }
}
