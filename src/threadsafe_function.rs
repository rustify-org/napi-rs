use napi_derive::napi;
use std::thread;

use napi::{
  bindgen_prelude::*,
  threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode},
  Error,
};

#[napi(ts_args_type = "callback: (err: null | Error, result: number) => void")]
pub fn call_threadsafe_function(callback: JsFunction) -> Result<()> {
  let tsfn: ThreadsafeFunction<u32, ErrorStrategy::CalleeHandled> = callback
    .create_threadsafe_function(0, |ctx| {
      ctx.env.create_uint32(ctx.value + 1).map(|v| vec![v])
    })?;
  for n in 0..100 {
    // 多线程
    let tsfn = tsfn.clone(); // 获取所有权，保证线程安全
    thread::spawn(move || {
      if n & 1 == 0 {
        // 模拟正常
        tsfn.call(Ok(n), ThreadsafeFunctionCallMode::Blocking);
      } else {
        // 模拟错误
        tsfn.call(
          Err(Error::new(Status::GenericFailure, format!("n={}", n))),
          ThreadsafeFunctionCallMode::Blocking,
        );
      }
    });
  }
  Ok(())
}
