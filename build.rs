#![allow(dead_code)]
#![allow(unused_variables, unused_imports)]

use std::env;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

#[cfg(unix)]
fn cc_build() {
    cc::Build::new()
        .file("./ftdc2c_ctp/ftdc2c_ctp.cpp")
        .cpp(true)
        .warnings(false)
        .flag("-Wno-unused-parameter")
        .flag("-Wno-attributes")
        .flag("-std=c++11")
        .compile("ftdc2c_ctp");

    println!("cargo:rustc-link-search={}", "ftdc2c_ctp/api/linux");
}

#[cfg(unix)]
fn copy_lib_file() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", out_dir);

    let dll = "thostmduserapi_se.so";
    let target = Path::new(&out_dir).join(dll);
    if !target.exists() {
        std::fs::copy(format!("./ftdc2c_ctp/api/linux/{}", dll), &target)
            .expect(&format!("failed to copy {} to outdir", dll));
    }

    let dll = "thosttraderapi_se.so";
    let target = Path::new(&out_dir).join(dll);
    if !target.exists() {
        std::fs::copy(format!("./ftdc2c_ctp/api/linux/{}", dll), &target)
            .expect(&format!("failed to copy {} to outdir", dll));
    }
}

#[cfg(windows)]
fn cc_build() {
    cc::Build::new()
        .file("./ftdc2c_ctp/ftdc2c_ctp.cpp")
        .cpp(true)
        .warnings(false)
        // .flag("-Wno-unused-parameter")
        // .flag("-Wno-attributes")
        .flag("-std=c++11")
        .compile("ftdc2c_ctp");

    println!("cargo:rustc-link-search={}", "ftdc2c_ctp/api/win_x64");
}

#[cfg(windows)]
fn copy_lib_files() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", out_dir);

    let dll = "thostmduserapi_se.dll";
    let target = Path::new(&out_dir).join(dll);
    if !target.exists() {
        std::fs::copy(format!("./ftdc2c_ctp/api/win_x64/{}", dll), &target)
            .expect(&format!("failed to copy {} to outdir", dll));
    }

    let dll = "thosttraderapi_se.dll";
    let target = Path::new(&out_dir).join(dll);
    if !target.exists() {
        std::fs::copy(format!("./ftdc2c_ctp/api/win_x64/{}", dll), &target)
            .expect(&format!("failed to copy {} to outdir", dll));
    }

    // println!(
    //     "cargo:resource={}",
    //     target.to_str().expect("Path to str failed")
    // );
}

fn main() {
    cc_build();

    println!("cargo:rustc-link-lib=dylib=thostmduserapi_se");
    println!("cargo:rustc-link-lib=dylib=thosttraderapi_se");

    println!("cargo:rerun-if-changed=ftdc2c_ctp/ftdc2c_ctp.cpp");
    println!("cargo:rerun-if-changed=ftdc2c_ctp/ftdc2c_ctp.h");
    println!("cargo:rerun-if-changed=ftdc2c_ctp/enums.h");
    println!("cargo:rerun-if-changed=ftdc2c_ctp/quoter.h");
    println!("cargo:rerun-if-changed=ftdc2c_ctp/trader.h");
    println!("cargo:rerun-if-changed=src/wrapper.hpp");
    println!("cargo:rerun-if-changed=build.rs");

    let binding = bindgen::builder()
        .header("./src/wrapper.hpp")
        .clang_arg("-Iftdc2c_ctp")
        .rustified_enum(".*")
        .derive_debug(true)
        .derive_default(true)
        .layout_tests(false)
        .generate_comments(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut output_file =
        std::fs::File::create("src/ctp/generated.rs").expect("cannot create bindings file");

    // output_file
    //     .write("#![allow(dead_code)]\n#![allow(non_upper_case_globals)]\n#![allow(non_camel_case_types)]\n#![allow(non_snake_case)]\n"
    //     .as_bytes())
    //     .unwrap();
    // output_file
    //     .write("#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]\n\n"
    //     .as_bytes())
    //     .unwrap();

    // const char* ppInstrumentID[]
    let output = binding.to_string().replace(
        "*mut *const ::std::os::raw::c_char",
        "*const *const ::std::os::raw::c_char",
    );
    let output = output.replace(" = ::std::os::raw::c_char;", " = ::std::os::raw::c_uchar;");
    let output = output.replace("[::std::os::raw::c_char;", "[::std::os::raw::c_uchar;");
    output_file
        .write_all(output.as_bytes())
        .expect("Couldn't write bindings!");

    copy_lib_files();
}
