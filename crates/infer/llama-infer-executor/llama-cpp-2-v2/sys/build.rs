use cmake::Config;
use glob::glob;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::DirEntry;

macro_rules! debug_log {
    ($($arg:tt)*) => {
        if std::env::var("BUILD_DEBUG").is_ok() {
            println!("cargo:warning=[DEBUG] {}", format!($($arg)*));
        }
    };
}

fn get_cargo_target_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let path = PathBuf::from(out_dir);
    let target_dir = path
        .ancestors()
        .nth(3)
        .ok_or("OUT_DIR is not deep enough")?;
    Ok(target_dir.to_path_buf())
}

fn extract_lib_names(out_dir: &Path) -> Vec<String> {
    let lib_pattern = if cfg!(target_os = "windows") && !cfg!(target_env = "gnu") {
        "*.lib"
    } else {
        "*.a"
    };

    let libs_dir = out_dir.join("lib*");
    let pattern = libs_dir.join(lib_pattern);
    debug_log!("Extract libs {}", pattern.display());

    let mut lib_names: Vec<String> = Vec::new();

    // Process the libraries based on the pattern
    for entry in glob(pattern.to_str().unwrap()).unwrap() {
        match entry {
            Ok(path) => {
                let stem = path.file_stem().unwrap();
                let stem_str = stem.to_str().unwrap();

                // Remove the "lib" prefix if present
                let lib_name = if stem_str.starts_with("lib") {
                    stem_str.strip_prefix("lib").unwrap_or(stem_str)
                } else {
                    if path.extension() == Some(std::ffi::OsStr::new("a")) {
                        // panic!("renaming {:?} to {:?}", &path, path.join(format!("lib{}.a", stem_str)));
                        let target = path.parent().unwrap().join(format!("lib{}.a", stem_str));
                        std::fs::rename(&path, &target).unwrap_or_else(|e| {
                            panic!("Failed to rename {path:?} to {target:?}: {e:?}");
                        })
                    }
                    stem_str
                };
                lib_names.push(lib_name.to_string());
            }
            Err(e) => println!("cargo:warning=error={}", e),
        }
    }
    lib_names
}

fn macos_link_search_path() -> Option<String> {
    let output = Command::new("clang")
        .arg("--print-search-dirs")
        .output()
        .ok()?;
    if !output.status.success() {
        println!(
            "failed to run 'clang --print-search-dirs', continuing without a link search path"
        );
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.contains("libraries: =") {
            let path = line.split('=').nth(1)?;
            return Some(format!("{}/lib/darwin", path));
        }
    }

    println!("failed to determine link search path, continuing without it");
    None
}

fn is_hidden(e: &DirEntry) -> bool {
    e.file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or_default()
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let target = env::var("TARGET").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let target_dir = get_cargo_target_dir().unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("Failed to get CARGO_MANIFEST_DIR");
    let llama_src = Path::new(&manifest_dir).join("llama.cpp");

    let profile = env::var("LLAMA_LIB_PROFILE").unwrap_or("Release".to_string());
    let static_crt = env::var("LLAMA_STATIC_CRT")
        .map(|v| v == "1")
        .unwrap_or(false);

    println!("cargo:rerun-if-env-changed=LLAMA_LIB_PROFILE");
    println!("cargo:rerun-if-env-changed=LLAMA_BUILD_SHARED_LIBS");
    println!("cargo:rerun-if-env-changed=LLAMA_STATIC_CRT");

    debug_log!("TARGET: {}", target);
    debug_log!("CARGO_MANIFEST_DIR: {}", manifest_dir);
    debug_log!("TARGET_DIR: {}", target_dir.display());
    debug_log!("OUT_DIR: {}", out_dir.display());

    // Make sure that changes to the llama.cpp project trigger a rebuild.
    let rebuild_on_children_of = [
        llama_src.join("src"),
        llama_src.join("ggml/src"),
        llama_src.join("common"),
    ];
    for entry in walkdir::WalkDir::new(&llama_src)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
    {
        let entry = entry.expect("Failed to obtain entry");
        let rebuild = entry
            .file_name()
            .to_str()
            .map(|f| f.starts_with("CMake"))
            .unwrap_or_default()
            || rebuild_on_children_of
                .iter()
                .any(|src_folder| entry.path().starts_with(src_folder));
        if rebuild {
            println!("cargo:rerun-if-changed={}", entry.path().display());
        }
    }

    // Speed up build
    env::set_var(
        "CMAKE_BUILD_PARALLEL_LEVEL",
        std::thread::available_parallelism()
            .unwrap()
            .get()
            .to_string(),
    );

    // Bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", llama_src.join("include").display()))
        .clang_arg(format!("-I{}", llama_src.join("ggml/include").display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .derive_partialeq(true)
        .allowlist_function("ggml_.*")
        .allowlist_type("ggml_.*")
        .allowlist_function("llama_.*")
        .allowlist_type("llama_.*")
        .prepend_enum_name(false)
        .generate()
        .expect("Failed to generate bindings");

    // Write the generated bindings to an output file
    let bindings_path = out_dir.join("bindings.rs");
    bindings
        .write_to_file(bindings_path)
        .expect("Failed to write bindings");

    println!("cargo:rerun-if-changed=wrapper.h");

    debug_log!("Bindings Created");

    // Build with Cmake

    let mut config = Config::new(&llama_src);

    // Would require extra source files to pointlessly
    // be included in what's uploaded to and downloaded from
    // crates.io, so deactivating these instead
    config.define("LLAMA_BUILD_TESTS", "OFF");
    config.define("LLAMA_BUILD_EXAMPLES", "OFF");
    config.define("LLAMA_BUILD_SERVER", "OFF");

    config.define("BUILD_SHARED_LIBS", "OFF");

    if cfg!(target_os = "macos") {
        config.define("GGML_BLAS", "OFF");
    }

    if cfg!(windows) {
        config.static_crt(static_crt);
    }

    if target.contains("android") {
        // build flags for android taken from this doc
        // https://github.com/ggerganov/llama.cpp/blob/master/docs/android.md
        let android_ndk = env::var("ANDROID_NDK")
            .expect("Please install Android NDK and ensure that ANDROID_NDK env variable is set");

        println!("cargo::rerun-if-env-changed=ANDROID_NDK");

        config.define(
            "CMAKE_TOOLCHAIN_FILE",
            format!("{android_ndk}/build/cmake/android.toolchain.cmake"),
        );
        if env::var("ANDROID_PLATFORM").is_ok() {
            println!("cargo::rerun-if-env-changed=ANDROID_PLATFORM");
        } else {
            config.define("ANDROID_PLATFORM", "android-28");
        }
        if target.contains("aarch64") {
            config.cflag("-march=armv8.7a");
            config.cxxflag("-march=armv8.7a");
        } else if target.contains("armv7") {
            config.cflag("-march=armv8.7a");
            config.cxxflag("-march=armv8.7a");
        } else if target.contains("x86_64") {
            config.cflag("-march=x86-64");
            config.cxxflag("-march=x86-64");
        } else if target.contains("i686") {
            config.cflag("-march=i686");
            config.cxxflag("-march=i686");
        } else {
            // Rather than guessing just fail.
            panic!("Unsupported Android target {target}");
        }
        config.define("GGML_LLAMAFILE", "OFF");
    }

    if cfg!(feature = "vulkan") {
        config.define("GGML_VULKAN", "ON");
    }

    config.define("GGML_OPENMP", "OFF");

    // General
    config
        .profile(&profile)
        .very_verbose(std::env::var("CMAKE_VERBOSE").is_ok()) // Not verbose by default
        .always_configure(false);

    let build_dir = config.build();

    // Search paths
    println!("cargo:rustc-link-search={}", out_dir.join("lib").display());
    println!(
        "cargo:rustc-link-search={}",
        out_dir.join("lib64").display()
    );
    println!("cargo:rustc-link-search={}", build_dir.display());

    // Link libraries
    let llama_libs_kind = "static";
    let llama_libs = extract_lib_names(&out_dir);
    assert_ne!(llama_libs.len(), 0);

    for lib in llama_libs {
        println!("cargo:rustc-link-lib={}={}", llama_libs_kind, lib);
    }

    if cfg!(feature = "vulkan") {
        if cfg!(windows) {
            println!("cargo:rustc-link-lib=vulkan-1");
        }

        if cfg!(target_os = "linux") {
            println!("cargo:rustc-link-lib=vulkan");
        }
    }

    // OpenMP
    // println!("cargo:rustc-link-lib={}={}", llama_libs_kind, "gomp");

    // // macOS
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=Metal");
        println!("cargo:rustc-link-lib=framework=MetalKit");
        println!("cargo:rustc-link-lib=framework=Accelerate");
    }

    add_cpp_link_stdlib();

    if target.contains("apple") {
        // On (older) OSX we need to link against the clang runtime,
        // which is hidden in some non-default path.
        //
        // More details at https://github.com/alexcrichton/curl-rust/issues/279.
        if let Some(path) = macos_link_search_path() {
            println!("cargo:rustc-link-lib=clang_rt.osx");
            println!("cargo:rustc-link-search={}", path);
        }
    }
}

fn add_cpp_link_stdlib() {
    let target = env::var("TARGET").unwrap();

    if cfg!(target_os = "windows") && cfg!(target_env = "msvc") {
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
