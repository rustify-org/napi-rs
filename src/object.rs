use napi_derive::napi;

// interface 约束
#[napi(object)]
pub struct Pet {
  pub name: String,
  pub kind: u32,
}

#[napi]
pub fn print_pet(pet: Pet) {
  println!("name: {}, kind: {}", pet.name, pet.kind);
}
