pub const SUPPORTED_EXTENSIONS: &[&str] = &[".js", ".mjs", ".cjs"];

pub const JS_EXTENSIONS: &[&str] = &[".js", ".mjs", ".cjs"];

pub fn is_supported_ext(ext: &str) -> bool {
    let arr = vec![".js", ".mjs", ".cjs"];

    if arr.contains(&ext) {
        return true;
    }

    return false;
}
