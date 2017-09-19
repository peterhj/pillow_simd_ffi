#[macro_use]
extern crate lazy_static;

use ffi::*;

use std::ffi::{CString};
use std::mem::{transmute};
use std::os::raw::{c_char};
use std::slice::{from_raw_parts, from_raw_parts_mut};

pub mod ffi;

lazy_static! {
  static ref MODE_1:        CString = CString::new("1").unwrap();
  static ref MODE_L:        CString = CString::new("L").unwrap();
  static ref MODE_P:        CString = CString::new("P").unwrap();
  static ref MODE_I:        CString = CString::new("I").unwrap();
  static ref MODE_F:        CString = CString::new("F").unwrap();
  static ref MODE_RGB:      CString = CString::new("RGB").unwrap();
  static ref MODE_RGBA:     CString = CString::new("RGBA").unwrap();
  static ref MODE_CMYK:     CString = CString::new("CMYK").unwrap();
  static ref MODE_YCBCR:    CString = CString::new("YCbCr").unwrap();
  static ref MODE_LAB:      CString = CString::new("LAB").unwrap();
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PILType {
  Uint8,
  Int32,
  Float32,
}

impl PILType {
  pub fn from_raw(raw: u32) -> Self {
    match raw {
      IMAGING_TYPE_UINT8    => PILType::Uint8,
      IMAGING_TYPE_INT32    => PILType::Int32,
      IMAGING_TYPE_FLOAT32  => PILType::Float32,
      IMAGING_TYPE_SPECIAL  => unimplemented!(),
      _ => unreachable!(),
    }
  }

  pub fn to_raw(&self) -> u32 {
    match *self {
      PILType::Uint8    => IMAGING_TYPE_UINT8,
      PILType::Int32    => IMAGING_TYPE_INT32,
      PILType::Float32  => IMAGING_TYPE_FLOAT32,
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PILMode {
  Unit,
  L,
  P,
  I,
  F,
  RGB,
  RGBA,
  CMYK,
  YCbCr,
  LAB,
}

impl PILMode {
  pub fn to_raw(&self) -> *const c_char {
    match *self {
      PILMode::Unit     => MODE_1.as_c_str().as_ptr(),
      PILMode::L        => MODE_L.as_c_str().as_ptr(),
      PILMode::P        => MODE_P.as_c_str().as_ptr(),
      PILMode::I        => MODE_I.as_c_str().as_ptr(),
      PILMode::F        => MODE_F.as_c_str().as_ptr(),
      PILMode::RGB      => MODE_RGB.as_c_str().as_ptr(),
      PILMode::RGBA     => MODE_RGBA.as_c_str().as_ptr(),
      PILMode::CMYK     => MODE_CMYK.as_c_str().as_ptr(),
      PILMode::YCbCr    => MODE_YCBCR.as_c_str().as_ptr(),
      PILMode::LAB      => MODE_LAB.as_c_str().as_ptr(),
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

unsafe impl Send for PILImage {}
unsafe impl Sync for PILImage {}

impl Drop for PILImage {
  fn drop(&mut self) {
    unsafe { ImagingDelete(self.raw) };
  }
}

impl PILImage {
  pub fn new(mode: PILMode, xdim: i32, ydim: i32) -> Self {
    PILImage{
      raw:  unsafe { ImagingNew(mode.to_raw(), xdim, ydim) },
    }
  }

  pub fn line_size_bytes(&self) -> i32 {
    unsafe { (*self.raw).linesize }
  }

  pub fn x_size(&self) -> i32 {
    unsafe { (*self.raw).xsize }
  }

  pub fn y_size(&self) -> i32 {
    unsafe { (*self.raw).ysize }
  }

  pub fn pixel_type(&self) -> PILType {
    PILType::from_raw(unsafe { (*self.raw).type_ } as u32)
  }

  pub fn pixel_channels(&self) -> i32 {
    unsafe { (*self.raw).bands }
  }

  pub fn pixel_size_bytes(&self) -> i32 {
    unsafe { (*self.raw).pixelsize }
  }

  pub fn line_u8(&self, yidx: i32) -> Option<&[u8]> {
    match (self.pixel_type(), self.pixel_size_bytes()) {
      (PILType::Uint8, 1) => {
        let line_ptr = unsafe { *((*self.raw).image8.offset(yidx as isize)) };
        Some(unsafe { from_raw_parts(line_ptr, self.x_size() as usize) })
      }
      (PILType::Uint8, 4) => {
        let line_ptr = unsafe { *((*self.raw).image8.offset(yidx as isize)) };
        Some(unsafe { from_raw_parts(line_ptr, (4 * self.x_size()) as usize) })
      }
      _ => None,
    }
  }

  pub fn line_u8_mut(&mut self, yidx: i32) -> Option<&mut [u8]> {
    match (self.pixel_type(), self.pixel_size_bytes()) {
      (PILType::Uint8, 1) => {
        let line_ptr = unsafe { *((*self.raw).image8.offset(yidx as isize)) };
        Some(unsafe { from_raw_parts_mut(line_ptr, self.x_size() as usize) })
      }
      (PILType::Uint8, 4) => {
        let line_ptr = unsafe { *((*self.raw).image8.offset(yidx as isize)) };
        Some(unsafe { from_raw_parts_mut(line_ptr, (4 * self.x_size()) as usize) })
      }
      _ => None,
    }
  }

  pub fn line_i32(&self, yidx: i32) -> Option<&[i32]> {
    match (self.pixel_type(), self.pixel_size_bytes()) {
      (PILType::Int32, 4) => {
        let line_ptr = unsafe { *((*self.raw).image32.offset(yidx as isize)) };
        Some(unsafe { from_raw_parts(line_ptr, self.x_size() as usize) })
      }
      _ => None,
    }
  }

  pub fn line_i32_mut(&mut self, yidx: i32) -> Option<&mut [i32]> {
    match (self.pixel_type(), self.pixel_size_bytes()) {
      (PILType::Int32, 4) => {
        let line_ptr = unsafe { *((*self.raw).image32.offset(yidx as isize)) };
        Some(unsafe { from_raw_parts_mut(line_ptr, self.x_size() as usize) })
      }
      _ => None,
    }
  }

  pub fn line_f32(&self, yidx: i32) -> Option<&[f32]> {
    match (self.pixel_type(), self.pixel_size_bytes()) {
      (PILType::Float32, 4) => {
        let line_ptr = unsafe { *((*self.raw).image32.offset(yidx as isize)) };
        Some(unsafe { from_raw_parts(transmute(line_ptr), self.x_size() as usize) })
      }
      _ => None,
    }
  }

  pub fn line_f32_mut(&mut self, yidx: i32) -> Option<&mut [f32]> {
    match (self.pixel_type(), self.pixel_size_bytes()) {
      (PILType::Float32, 4) => {
        let line_ptr = unsafe { *((*self.raw).image32.offset(yidx as isize)) };
        Some(unsafe { from_raw_parts_mut(transmute(line_ptr), self.x_size() as usize) })
      }
      _ => None,
    }
  }

  pub fn render_pixels(&self, pixel_buf: &mut [u8]) -> Result<usize, ()> {
    let px_sz = self.pixel_size_bytes() as usize;
    let x_size = self.x_size() as usize;
    let y_size = self.y_size() as usize;
    let ch_size = self.pixel_channels() as usize;
    assert!(ch_size <= px_sz);
    for yidx in 0 .. y_size {
      match self.line_u8(yidx as _) {
        None => return Err(()),
        Some(line) => {
          for xidx in 0 .. x_size {
            for c in 0 .. ch_size {
              pixel_buf[c + ch_size * (xidx + x_size * yidx)] = line[c + px_sz * xidx];
            }
          }
        }
      }
    }
    Ok(ch_size * x_size * y_size)
  }

  pub fn render_planes(&self, plane_buf: &mut [u8]) -> Result<usize, ()> {
    let px_sz = self.pixel_size_bytes() as usize;
    let x_size = self.x_size() as usize;
    let y_size = self.y_size() as usize;
    let ch_size = self.pixel_channels() as usize;
    assert!(ch_size <= px_sz);
    for yidx in 0 .. y_size {
      match self.line_u8(yidx as _) {
        None => return Err(()),
        Some(line) => {
          for xidx in 0 .. x_size {
            for c in 0 .. ch_size {
              plane_buf[xidx + x_size * (yidx + c * y_size)] = line[c + px_sz * xidx];
            }
          }
        }
      }
    }
    Ok(ch_size * x_size * y_size)
  }

  pub fn decode() -> Self {
    // TODO
    unimplemented!();
  }

  pub fn encode(&self) {
    // TODO
    unimplemented!();
  }

  pub fn resample(&self, new_xdim: i32, new_ydim: i32, filter: PILFilter) -> Self {
    PILImage{
      raw:  unsafe { ImagingResample(self.raw, new_xdim, new_ydim, filter.to_raw() as i32) },
    }
  }
}
