use ffi::*;

use std::ptr::{null};

pub mod ffi;

#[derive(Clone, Copy, Debug)]
pub enum PILFilter {
  Nearest,
  Box_,
  Bilinear,
  Hamming,
  Bicubic,
  Lanczos,
}

impl PILFilter {
  pub fn to_raw(&self) -> u32 {
    match *self {
      PILFilter::Nearest    => IMAGING_TRANSFORM_NEAREST,
      PILFilter::Box_       => IMAGING_TRANSFORM_BOX,
      PILFilter::Bilinear   => IMAGING_TRANSFORM_BILINEAR,
      PILFilter::Hamming    => IMAGING_TRANSFORM_HAMMING,
      PILFilter::Bicubic    => IMAGING_TRANSFORM_BICUBIC,
      PILFilter::Lanczos    => IMAGING_TRANSFORM_LANCZOS,
    }
  }
}

pub struct PILImage {
  raw:  Imaging,
}

impl Drop for PILImage {
  fn drop(&mut self) {
    unsafe { ImagingDelete(self.raw) };
  }
}

impl PILImage {
  pub fn new(xdim: i32, ydim: i32) -> Self {
    PILImage{
      // FIXME: need non-null mode string.
      raw:  unsafe { ImagingNew(null(), xdim, ydim) },
    }
  }

  pub fn resample(&self, new_xdim: i32, new_ydim: i32, filter: PILFilter) -> Self {
    PILImage{
      raw:  unsafe { ImagingResample(self.raw, new_xdim, new_ydim, filter.to_raw() as i32) },
    }
  }
}
