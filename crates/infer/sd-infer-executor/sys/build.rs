use std::{
    env,
    fs::{self, create_dir_all},
    path::{Path, PathBuf},
};

use cmake::Config;
use fs_extra::dir;

// Inspired by https://github.com/tazz4843/whisper-rs/blob/master/sys/build.rs

fn main() {
    let target = env::var("TARGET").unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");

    // Copy stable-diffusion code into the build script directory
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let diffusion_root = out.join("stable-diffusion.cpp/");

    if !diffusion_root.exists() {
        create_dir_all(&diffusion_root).unwrap();
        dir::copy("./stable-diffusion.cpp", &out, &Default::default()).unwrap_or_else(|e| {
            panic!(
                "Failed to copy stable-diffusion sources into {}: {}",
                diffusion_root.display(),
                e
            )
        });

        remove_default_params_stb(&diffusion_root.join("thirdparty/stb_image_write.h"))
            .unwrap_or_else(|e| panic!("Failed to remove default parameters from stb: {}", e));
    }

    // Bindgen
    if env::var("DIFFUSION_SKIP_BINDINGS").is_ok() {
        fs::copy("src/bindings.rs", out.join("bindings.rs")).expect("Failed to copy bindings.rs");
    } else {
        let bindings = bindgen::Builder::default()
            .header("wrapper.h")
            .clang_arg("-I./stable-diffusion.cpp")
            .clang_arg("-I./stable-diffusion.cpp/ggml/include")
            .rustified_non_exhaustive_enum(".*")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .generate()
            .unwrap()
            .write_to_file(out.join("bindings.rs"));

        if let Err(e) = bindings {
            println!("cargo:warning=Unable to generate bindings: {}", e);
            println!("cargo:warning=Using bundled bindings.rs, which may be out of date");
            // copy src/bindings.rs to OUT_DIR
            fs::copy("src/bindings.rs", out.join("bindings.rs"))
                .expect("Unable to copy bindings.rs");
        }
    }

    // Configure cmake for building
    let mut config = Config::new(&diffusion_root);

    #[cfg(feature = "vulkan")]
    {
        if cfg!(windows) {
            println!("cargo:rustc-link-lib=vulkan-1");
        }

        if cfg!(target_os = "linux") {
            println!("cargo:rustc-link-lib=vulkan");
        }

        config.define("SD_VULKAN", "ON");
    }

    #[cfg(feature = "flashattn")]
    {
        config.define("SD_FLASH_ATTN", "ON");
    }

    // Build stable-diffusion
    config
        .profile("Release")
        .define("SD_BUILD_SHARED_LIBS", "OFF")
        .define("SD_BUILD_EXAMPLES", "OFF")
        .define("GGML_OPENMP", "OFF")
        .very_verbose(true)
        .pic(true);

    let destination = config.build();

    // Search paths
    println!("cargo:rustc-link-search=native={}", out.join("lib").display());
    println!("cargo:rustc-link-search=native={}", out.join("lib64").display());
    println!("cargo:rustc-link-search=native={}", out.join("build").display());
    println!("cargo:rustc-link-search=native={}", destination.display());

    let _ = add_lib_prefix_to_files(&format!("{}", out.join("lib").display()));
    let _ = add_lib_prefix_to_files(&format!("{}", out.join("lib64").display()));

    println!("cargo:rustc-link-lib=static=stable-diffusion");
    println!("cargo:rustc-link-lib=static=ggml");
    println!("cargo:rustc-link-lib=static=ggml-base");
    println!("cargo:rustc-link-lib=static=ggml-cpu");

    #[cfg(feature = "vulkan")]
    println!("cargo:rustc-link-lib=static=ggml-vulkan");

    if target.contains("apple") {
        println!("cargo:rustc-link-lib=framework=Accelerate");
        //println!("cargo:rustc-link-lib=framework=Foundation");
        //println!("cargo:rustc-link-lib=framework=Metal");
        //println!("cargo:rustc-link-lib=framework=MetalKit");
    }

    add_cpp_link_stdlib();
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


fn remove_default_params_stb(file: &Path) -> std::io::Result<()> {
    let data = fs::read_to_string(file)?;
    let new_data = data.replace("const char* parameters = NULL", "const char* parameters");
    fs::write(file, new_data)
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