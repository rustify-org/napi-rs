use napi::{
  bindgen_prelude::{AbortSignal, AsyncTask},
  Error, JsFunction, JsNumber, Result, Task,
};
use napi_derive::napi;

struct AsyncFib {
  input: u32,
}

fn fib(n: u32) -> u32 {
  match n {
    0 => 0,
    1 => 1,
    _ => fib(n - 1) + fib(n - 2),
  }
}

impl Task for AsyncFib {
  type Output = u32;

  type JsValue = JsNumber;

  // 跑在libuv，耗时操作
  fn compute(&mut self) -> Result<Self::Output> {
    Ok(fib(self.input))
  }

  // 最终返回通过resolve
  fn resolve(&mut self, env: napi::Env, output: Self::Output) -> Result<Self::JsValue> {
    env.create_uint32(output)
  }
}

#[napi]
fn async_fib_task(input: u32) -> AsyncTask<AsyncFib> {
  AsyncTask::new(AsyncFib { input })
}

#[napi]
fn async_fib(input: u32, signal: AbortSignal) -> AsyncTask<AsyncFib> {
  AsyncTask::with_signal(AsyncFib { input }, signal)
}
