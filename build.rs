extern crate bindgen;
extern crate gcc;
extern crate walkdir;

use walkdir::{WalkDir};

use std::env;
use std::path::{PathBuf};

fn main() {
  let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
  let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
  let cc = env::var("CC").unwrap_or("gcc".to_owned());

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

  // TODO: special compilation steps for "libImaging".
  let mut compiler = gcc::Build::new();
  compiler
    .compiler(&cc)
    .opt_level(2)
    .pic(true)
    .flag("-fwrapv")
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
    .flag("-DSIZEOF_SHORT=2")
    .flag("-DSIZEOF_INT=4")
    .flag("-DSIZEOF_LONG=8")
    .flag("-DSIZEOF_LONG_LONG=8")
    .flag("-DSTDC_HEADERS")
    .flag("-Wall")
    .flag("-Wformat")
    .flag("-Wstrict-prototypes")
    .flag("-Werror=format-security")
    .include(imaging_src_dir.as_os_str().to_str().unwrap());
  //compiler.file(&imaging_lib_path);
  for path in imaging_src_paths.iter() {
    compiler.file(path);
  }
  compiler.compile("libpillow_simd_imaging.a");

  // TODO: generate interesting bindings.
  bindgen::Builder::default()
    //.header("pillow-simd/libImaging/Imaging.h")
    .header("wrap.h")
    //.link(...)
    .whitelisted_type("Imaging")
    .whitelisted_function("ImagingResample")
    .whitelisted_function("ImagingTransform")
    .generate()
    .expect("")
    .write_to_file(out_dir.join("imaging_bind.rs"))
    .expect("");
}
