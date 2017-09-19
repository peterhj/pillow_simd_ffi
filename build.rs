extern crate bindgen;
extern crate gcc;
extern crate walkdir;

use walkdir::{WalkDir};

use std::env;
use std::mem::{size_of};
use std::os::raw::{c_short, c_int, c_long, c_longlong};
use std::path::{PathBuf};

fn main() {
  let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
  let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
  //let cc = env::var("CC").unwrap_or("gcc".to_owned());

  println!("cargo:rustc-link-search=native={}", out_dir.display());
  println!("cargo:rerun-if-changed=build.rs");

  let imaging_src_dir = manifest_dir.join("pillow-simd").join("libImaging");
  let mut imaging_src_paths = vec![];
  for entry in WalkDir::new(imaging_src_dir.to_str().unwrap()) {
    let entry = entry.unwrap();
    match entry.path().extension() {
      None => continue,
      Some(ext) => if ext.to_str().unwrap() != "c" {
        continue;
      },
    }
    if entry.path().file_name().unwrap().to_str().unwrap() == "codec_fd.c" {
      continue;
    }
    if entry.path().file_name().unwrap().to_str().unwrap() == "ResampleSIMDHorizontalConv.c" {
      continue;
    }
    if entry.path().file_name().unwrap().to_str().unwrap() == "ResampleSIMDVerticalConv.c" {
      continue;
    }
    println!("cargo:rerun-if-changed={}", entry.path().display());
    imaging_src_paths.push(entry.path().as_os_str().to_str().unwrap().to_owned());
  }

  let sz_c_short = size_of::<c_short>();
  let sz_c_int = size_of::<c_int>();
  let sz_c_long = size_of::<c_long>();
  let sz_c_longlong = size_of::<c_longlong>();

  let mut compiler = gcc::Build::new();
  compiler
    //.compiler(&cc)
    .opt_level(2)
    .pic(true)
    //.flag("-fstack-protector-strong")
    .flag("-g")
    .flag("-msse4.1")
    .flag("-pthread")
    .flag("-D_FORTIFY_SOURCE=2")
    .flag("-DDISABLE_PYTHON=1")
    // FIXME: these macros are normally provided by cpython.
    .flag("-DHAVE_LIBJPEG")
    .flag("-DHAVE_LIBZ")
    .flag("-DHAVE_PROTOTYPES")
    .flag("-DNDEBUG")
    .flag(&format!("-DSIZEOF_SHORT={}", sz_c_short))
    .flag(&format!("-DSIZEOF_INT={}", sz_c_int))
    .flag(&format!("-DSIZEOF_LONG={}", sz_c_long))
    .flag(&format!("-DSIZEOF_LONG_LONG={}", sz_c_longlong))
    .flag("-DSTDC_HEADERS")
    .flag("-Wall")
    .flag("-Wformat")
    .flag("-Wstrict-prototypes")
    .flag("-Werror=format-security")
    .include(imaging_src_dir.as_os_str().to_str().unwrap());
  for path in imaging_src_paths.iter() {
    compiler.file(path);
  }
  compiler.compile("libpillow_simd_imaging.a");

  bindgen::Builder::default()
    .header("wrap.h")
    .clang_arg(format!("-DSIZEOF_SHORT={}", sz_c_short))
    .clang_arg(format!("-DSIZEOF_INT={}", sz_c_int))
    .clang_arg(format!("-DSIZEOF_LONG={}", sz_c_long))
    .clang_arg(format!("-DSIZEOF_LONG_LONG={}", sz_c_longlong))
    .link("pillow_simd_imaging")
    .whitelisted_type("Imaging")
    .whitelisted_type("ImagingSectionCookie")
    .whitelisted_function("ImagingSectionEnter")
    .whitelisted_function("ImagingSectionLeave")
    .whitelisted_function("ImagingNew")
    .whitelisted_function("ImagingNew2")
    .whitelisted_function("ImagingDelete")
    .whitelisted_function("ImagingResample")
    .whitelisted_function("ImagingTransform")
    /*.whitelisted_function("ImagingJpegDecode")
    .whitelisted_function("ImagingJpegDecodeCleanup")
    .whitelisted_function("ImagingJpegEncode")
    .whitelisted_function("ImagingZipDecode")
    .whitelisted_function("ImagingZipDecodeCleanup")
    .whitelisted_function("ImagingZipEncode")
    .whitelisted_function("ImagingZipEncodeCleanup")*/
    .whitelisted_var("IMAGING_TYPE_UINT8")
    .whitelisted_var("IMAGING_TYPE_INT32")
    .whitelisted_var("IMAGING_TYPE_FLOAT32")
    .whitelisted_var("IMAGING_TYPE_SPECIAL")
    .whitelisted_var("IMAGING_TRANSFORM_NEAREST")
    .whitelisted_var("IMAGING_TRANSFORM_BOX")
    .whitelisted_var("IMAGING_TRANSFORM_BILINEAR")
    .whitelisted_var("IMAGING_TRANSFORM_HAMMING")
    .whitelisted_var("IMAGING_TRANSFORM_BICUBIC")
    .whitelisted_var("IMAGING_TRANSFORM_LANCZOS")
    .generate()
    .expect("")
    .write_to_file(out_dir.join("imaging_bind.rs"))
    .expect("");
}
