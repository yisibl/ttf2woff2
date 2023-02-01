#![deny(clippy::all)]

use napi::{bindgen_prelude::*, JsBuffer};
use woff2::Woff2MemoryOut;

#[macro_use]
extern crate napi_derive;

mod woff2 {
  #[repr(C)]
  pub struct Woff2MemoryOutInner {
    _unused: [u8; 0],
  }

  #[repr(C)]
  pub struct Woff2MemoryOut {
    pub data: *const u8,
    pub length: usize,
    inner: *mut Woff2MemoryOutInner,
  }

  unsafe impl Send for Woff2MemoryOut {}

  impl Drop for Woff2MemoryOut {
    fn drop(&mut self) {
      unsafe {
        if !self.inner.is_null() {
          FreeMemoryOutput(self.inner);
        }
      }
    }
  }

  extern "C" {
    pub fn MaxWOFF2CompressedSize(data: *const u8, length: usize) -> usize;
    pub fn ComputeWOFF2FinalSize(data: *const u8, length: usize) -> usize;
    pub fn ConvertTTFToWOFF2(
      data: *const u8,
      length: usize,
      result: *mut u8,
      result_length: *mut usize,
    ) -> bool;
    pub fn ConvertWOFF2ToTTF(
      data: *const u8,
      length: usize,
      result: *mut u8,
      result_length: *mut usize,
    ) -> bool;
    pub fn FreeMemoryOutput(out: *mut Woff2MemoryOutInner);
  }
}

// `#[inline]` always causes llvm codegen error on i686-pc-windows-msvc
#[cfg_attr(not(target_arch = "x86"), inline)]
fn convert_to_woff2(input_buf_value: &[u8]) -> Result<Vec<u8>> {
  let len =
    unsafe { woff2::MaxWOFF2CompressedSize(input_buf_value.as_ptr(), input_buf_value.len()) };
  let mut output_buf = Vec::with_capacity(len);
  let mut output_buf_len = len;
  let ok = unsafe {
    woff2::ConvertTTFToWOFF2(
      input_buf_value.as_ptr(),
      input_buf_value.len(),
      output_buf.as_mut_ptr(),
      &mut output_buf_len as *mut usize,
    )
  };
  if !ok {
    return Err(Error::new(
      Status::GenericFailure,
      "ConvertTTFToWOFF2 failed".to_owned(),
    ));
  }
  unsafe { output_buf.set_len(output_buf_len) };
  Ok(output_buf)
}

// `#[inline]` always causes llvm codegen error on i686-pc-windows-msvc
#[cfg_attr(not(target_arch = "x86"), inline)]
fn convert_to_ttf(input_buf_value: &[u8]) -> Result<Vec<u8>> {
  let len =
    unsafe { woff2::ComputeWOFF2FinalSize(input_buf_value.as_ptr(), input_buf_value.len()) };
  let mut output_buf = Vec::with_capacity(len);
  let mut output_buf_len = len;
  let ok = unsafe {
    woff2::ConvertWOFF2ToTTF(
      input_buf_value.as_ptr(),
      input_buf_value.len(),
      output_buf.as_mut_ptr(),
      &mut output_buf_len as *mut usize,
    )
  };

  if !ok {
    return Err(Error::new(
      Status::GenericFailure,
      "ConvertWOFF2ToTTF failed".to_owned(),
    ));
  }
  unsafe { output_buf.set_len(output_buf_len) };
  Ok(output_buf)
}

#[napi(js_name = "convertTTFToWOFF2")]
pub fn convert_ttf_to_woff2(input: JsBuffer) -> Result<Buffer> {
  let input_buf_value = input.into_value()?;

  Ok(convert_to_woff2(input_buf_value.as_ref())?.into())
}

pub struct ConvertTTFToWOFF2Task {
  input: Buffer,
}

#[napi]
impl Task for ConvertTTFToWOFF2Task {
  type Output = Vec<u8>;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    convert_to_woff2(self.input.as_ref())
  }
  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output.into())
  }
}

#[napi(js_name = "convertTTFToWOFF2Async")]
pub fn convert_ttf_to_woff2_async(
  input: Buffer,
  signal: Option<AbortSignal>,
) -> AsyncTask<ConvertTTFToWOFF2Task> {
  AsyncTask::with_optional_signal(ConvertTTFToWOFF2Task { input }, signal)
}

pub struct ConvertWOFF2ToTTFTask {
  input: Buffer,
}

#[napi]
impl Task for ConvertWOFF2ToTTFTask {
  type Output = Vec<u8>;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    convert_to_ttf(self.input.as_ref())
  }
  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output.into())
  }
}

#[napi(js_name = "convertWOFF2ToTTF")]
pub fn convert_woff2_to_ttf(input: JsBuffer) -> Result<Buffer> {
  let input_buf_value = input.into_value()?;

  Ok(convert_to_ttf(input_buf_value.as_ref())?.into())
}

#[napi(js_name = "convertWOFF2ToTTFAsync")]
pub fn convert_woff2_to_ttf_async(
  input: Buffer,
  signal: Option<AbortSignal>,
) -> AsyncTask<ConvertWOFF2ToTTFTask> {
  AsyncTask::with_optional_signal(ConvertWOFF2ToTTFTask { input }, signal)
}
