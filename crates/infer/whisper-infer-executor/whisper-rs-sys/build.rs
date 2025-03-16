#![allow(clippy::uninlined_format_args)]

extern crate bindgen;

use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");

    let target = env::var("TARGET").unwrap();



    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let whisper_root = out.join("whisper.cpp/");

    if !whisper_root.exists() {
        std::fs::create_dir_all(&whisper_root).unwrap();
        fs_extra::dir::copy("./whisper.cpp", &out, &Default::default()).unwrap_or_else(|e| {
            panic!(
                "Failed to copy whisper sources into {}: {}",
                whisper_root.display(),
                e
            )
        });
    }

    if env::var("WHISPER_DONT_GENERATE_BINDINGS").is_ok() {
        let _: u64 = std::fs::copy("src/bindings.rs", out.join("bindings.rs"))
            .expect("Failed to copy bindings.rs");
    } else {
        let bindings = bindgen::Builder::default().header("wrapper.h");

        let bindings = bindings
            .clang_arg("-I./whisper.cpp/")
            .clang_arg("-I./whisper.cpp/include")
            .clang_arg("-I./whisper.cpp/ggml/include")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .generate();

        match bindings {
            Ok(b) => {
                let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
                b.write_to_file(out_path.join("bindings.rs"))
                    .expect("Couldn't write bindings!");
            }
            Err(e) => {
                println!("cargo:warning=Unable to generate bindings: {}", e);
                println!("cargo:warning=Using bundled bindings.rs, which may be out of date");
                // copy src/bindings.rs to OUT_DIR
                std::fs::copy("src/bindings.rs", out.join("bindings.rs"))
                    .expect("Unable to copy bindings.rs");
            }
        }
    };

    // stop if we're on docs.rs
    if env::var("DOCS_RS").is_ok() {
        return;
    }

    let mut config = Config::new(&whisper_root);

    config
        .profile("Release")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("WHISPER_ALL_WARNINGS", "OFF")
        .define("WHISPER_ALL_WARNINGS_3RD_PARTY", "OFF")
        .define("WHISPER_BUILD_TESTS", "OFF")
        .define("WHISPER_BUILD_EXAMPLES", "OFF")
        .very_verbose(true)
        .pic(true);

    /***
    if cfg!(target_os = "windows") {
        config.cxxflag("/utf-8");
    }
    ***/

    if cfg!(feature = "vulkan") {
        config.define("GGML_VULKAN", "ON");
        if cfg!(windows) {
            /***
            println!("cargo:rerun-if-env-changed=VULKAN_SDK");
            ***/

            println!("cargo:rustc-link-lib=vulkan-1");

            /***
            let vulkan_path = match env::var("VULKAN_SDK") {
                Ok(path) => PathBuf::from(path),
                Err(_) => panic!(
                    "Please install Vulkan SDK and ensure that VULKAN_SDK env variable is set"
                ),
            };
            let vulkan_lib_path = vulkan_path.join("Lib");
            println!("cargo:rustc-link-search={}", vulkan_lib_path.display());
             ***/
        } else if cfg!(target_os = "macos") {
            /***
            println!("cargo:rerun-if-env-changed=VULKAN_SDK");
            ***/

            println!("cargo:rustc-link-lib=vulkan");

            /***
            let vulkan_path = match env::var("VULKAN_SDK") {
                Ok(path) => PathBuf::from(path),
                Err(_) => panic!(
                    "Please install Vulkan SDK and ensure that VULKAN_SDK env variable is set"
                ),
            };
            let vulkan_lib_path = vulkan_path.join("lib");
            println!("cargo:rustc-link-search={}", vulkan_lib_path.display());
             ***/
        } else {
            println!("cargo:rustc-link-lib=vulkan");
        }
    }

    // Metal is enabled by default, so we need to explicitly disable it
    config.define("GGML_METAL", "OFF");


    if cfg!(debug_assertions) {
        // debug builds are too slow to even remotely be usable,
        // so we build with optimizations even in debug mode
        config.define("CMAKE_BUILD_TYPE", "RelWithDebInfo");
        config.cxxflag("-DWHISPER_DEBUG");
    }

    // Allow passing any WHISPER or CMAKE compile flags
    for (key, value) in env::vars() {
        let is_whisper_flag =
            key.starts_with("WHISPER_") && key != "WHISPER_DONT_GENERATE_BINDINGS";
        let is_cmake_flag = key.starts_with("CMAKE_");
        if is_whisper_flag || is_cmake_flag {
            config.define(&key, &value);
        }
    }

    config.define("GGML_OPENMP", "OFF");


    let destination = config.build();

    // Search paths
    println!("cargo:rustc-link-search=native={}", out.join("lib").display());
    println!("cargo:rustc-link-search=native={}", out.join("lib64").display());

    println!("cargo:rustc-link-search=native={}", destination.display());

    let _ = add_lib_prefix_to_files(&format!("{}", out.join("lib").display()));
    let _ = add_lib_prefix_to_files(&format!("{}", out.join("lib64").display()));

    println!("cargo:rustc-link-lib=static=whisper");
    println!("cargo:rustc-link-lib=static=ggml");
    println!("cargo:rustc-link-lib=static=ggml-base");
    println!("cargo:rustc-link-lib=static=ggml-cpu");

    if cfg!(feature = "vulkan") {
        println!("cargo:rustc-link-lib=static=ggml-vulkan");
    }

    // Link macOS Accelerate framework for matrix calculations
    if target.contains("apple") {
        println!("cargo:rustc-link-lib=framework=Accelerate");
        //println!("cargo:rustc-link-lib=framework=Foundation");
        //println!("cargo:rustc-link-lib=framework=Metal");
        //println!("cargo:rustc-link-lib=framework=MetalKit");
    }

    add_cpp_link_stdlib();

    // for whatever reason this file is generated during build and triggers cargo complaining
    _ = std::fs::remove_file("bindings/javascript/package.json");
}



fn add_cpp_link_stdlib() {
    let target = env::var("TARGET").unwrap();

    if cfg!(target_os = "windows") && cfg!(target_env = "msvc") {
        println!("cargo:rustc-link-lib=dylib=msvcrtd");
    } else if cfg!(target_os = "windows") && cfg!(target_env = "gnu") {
        println!("cargo:rustc-link-lib={}={}", "static", "stdc++");
    } else if target.contains("apple") || target.contains("freebsd") || target.contains("openbsd") {
        println!("cargo:rustc-link-lib={}={}", "static", "c++");
    } else if target.contains("android") {
        println!("cargo:rustc-link-lib={}={}", "static", "c++_shared");
    } else {
        println!("cargo:rustc-link-lib={}={}", "static", "stdc++");
    }
}

fn add_lib_prefix_to_files(dir_path: &str) -> Result<(), std::io::Error> {
    if !std::fs::exists(dir_path)? {
        return Ok(());
    }

    let entries = std::fs::read_dir(dir_path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(file_name) = path.file_name() {
                let file_name_str = file_name.to_string_lossy();
                if!file_name_str.starts_with("lib") {
                    let new_file_name = format!("lib{}", file_name_str);
                    let new_path = path.with_file_name(new_file_name);
                    std::fs::rename(&path, &new_path)?;
                }
            }
        }
    }

    Ok(())
}