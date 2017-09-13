extern crate bindgen;
extern crate gcc;

use std::env;
use std::path::{PathBuf};

fn main() {
  let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

  // TODO: special compilation steps for "libImaging".

  // TODO: generate interesting bindings.
  bindgen::Builder::default()
    .header("pillow-simd/libImaging/Imaging.h")
    //.link(...)
    .whitelisted_type("Imaging")
    .whitelisted_function("ImagingResample")
    .whitelisted_function("ImagingTransform")
    .generate()
    .expect("")
    .write_to_file(out_dir.join("imaging_bind.rs"))
    .expect("");
}
