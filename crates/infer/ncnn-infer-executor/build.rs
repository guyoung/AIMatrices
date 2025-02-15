use std::env;
use std::path::PathBuf;
use path_absolutize::*;

fn main() {
    let ncnn_depends_dir = PathBuf::from("./../../depends/ncnn").absolutize().unwrap().to_path_buf();

    println!("ncnn_depends_dir: {:?}", &ncnn_depends_dir);

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let ncnn_include_dir = if let Ok(path) = env::var("NCNN_INCLUDE_DIR") {
        PathBuf::from(path)
    } else {
        ncnn_depends_dir.join("include/ncnn")
    };

    if !ncnn_include_dir.join("c_api.h").exists() {
        panic!(
            "ERROR: please set NCNN_INCLUDE_DIR,e.g2. export NCNN_INCLUDE_DIR=/path/to/ncnn/include"
        );
    }

    let ncnn_lib_dir = if let Ok(path) = env::var("NCNN_LIB_DIR") {
        PathBuf::from(path)
    } else {
        if cfg!(target_os = "windows") && cfg!(target_env = "gnu") {
            ncnn_depends_dir.join("lib").join("mingw-x64")
        } else {
            ncnn_depends_dir.join("lib").join("linux-x64")
        }
    };

    println!("cargo:rerun-if-changed=build.rs");

    // println!("cargo:rerun-if-env-changed=NCNN_INCLUDE_DIR");
    let bindings = bindgen::Builder::default()
        .header(format!("{}/gpu.h", ncnn_include_dir.display())) // 启用gpu相关的函数；# 对cpu模式，包含gpu.h头文件，所以不做处理
        .header(format!("{}/c_api.h", ncnn_include_dir.display())) // 通用入口
        // .clang_arg(format!("-I{}", ncnn_include_dir.display())) // 无效
        .clang_arg("-x")
        .clang_arg("c++")
        .allowlist_type("regex")
        .allowlist_function("ncnn.*")
        .allowlist_var("NCNN.*")
        .allowlist_type("ncnn.*")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-search={}", ncnn_lib_dir.display());

    println!("cargo:rustc-link-lib={}={}",  "static", "ncnn");
    //println!("cargo:rustc-link-lib={}={}",  "static", "GenericCodeGen");
    //println!("cargo:rustc-link-lib={}={}",  "static", "MachineIndependent");
    //println!("cargo:rustc-link-lib={}={}",  "static", "OSDependent");
    println!("cargo:rustc-link-lib={}={}",  "static", "glslang");
    println!("cargo:rustc-link-lib={}={}",  "static", "SPIRV");
    println!("cargo:rustc-link-lib={}={}",  "static", "SPIRV-Tools");
    println!("cargo:rustc-link-lib={}={}",  "static", "SPIRV-Tools-opt");
    println!("cargo:rustc-link-lib=vulkan-1");
    println!("cargo:rustc-link-lib={}={}", "static", "gomp");
    println!("cargo:rustc-link-lib={}={}",  "static", "stdc++")
}


