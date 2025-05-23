use napi::{Error, Result};
use napi_derive::napi;

#[napi]
pub fn error_func(u: u32) -> Result<()> {
  if u & 1 != 0 {
    Ok(())
  } else {
    Err(Error::from_reason(format!("参数不可以为偶数：i={}", u)))
  }
}
