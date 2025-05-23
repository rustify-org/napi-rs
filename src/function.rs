use napi_derive::napi;

#[napi]
pub fn say_hi<F>(#[napi(ts_arg_type = "(msg: string)=>void")] f: F)
where
  F: Fn(String) -> napi::Result<()>,
{
  let r = f("fuzhiqiang".to_string());
  println!("r: {:?}", r);
}

#[napi(ts_args_type = "f: (msg: string)=>void, demo: string")]
pub fn say_hi2<F>(f: F, demo: String)
where
  F: Fn(String) -> napi::Result<()>,
{
  let r = f("fuzhiqiang".to_string());
  println!("r: {:?}", r);
}
