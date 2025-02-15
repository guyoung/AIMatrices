// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
use std::{
    borrow::Cow,
    path::{Component, Path, PathBuf, MAIN_SEPARATOR, MAIN_SEPARATOR_STR},
};

use llrt_utils::module::{export_default, ModuleInfo};
use rquickjs::{
    function::Opt,
    module::{Declarations, Exports, ModuleDef},
    prelude::{Func, Rest},
    Ctx, Object, Result,
};

pub struct PathModule;

#[cfg(windows)]
const DELIMITER: char = ';';
#[cfg(not(windows))]
const DELIMITER: char = ':';

#[cfg(windows)]
pub const CURRENT_DIR_STR: &str = ".\\";

#[cfg(windows)]
const FORWARD_SLASH_STR: &str = "/";

#[cfg(not(windows))]
pub const CURRENT_DIR_STR: &str = "./";

#[cfg(windows)]
use memchr::{memchr, memchr2, memchr2_iter};

#[cfg(windows)]
pub fn replace_backslash(path: impl Into<String>) -> String {
    let mut path = path.into();
    let bytes = unsafe { path.as_bytes_mut() };

    let mut start = 0;
    while let Some(pos) = memchr(b'\\', &bytes[start..]) {
        bytes[start + pos] = b'/';
        start += pos + 1;
    }
    path
}

#[cfg(not(windows))]
pub fn replace_backslash(path: impl Into<String>) -> String {
    path.into().replace('\\', "/")
}

#[cfg(windows)]
fn find_next_separator(s: &str) -> Option<usize> {
    memchr2(b'\\', b'/', s.as_bytes())
}

#[cfg(not(windows))]
fn find_next_separator(s: &str) -> Option<usize> {
    s.find(MAIN_SEPARATOR)
}

#[cfg(windows)]
fn find_last_sep(path: &str) -> Option<usize> {
    memchr2_iter(b'\\', b'/', path.as_bytes()).next_back()
}

#[cfg(not(windows))]
fn find_last_sep(path: &str) -> Option<usize> {
    path.rfind(MAIN_SEPARATOR)
}

pub fn dirname<'a, P: Into<Cow<'a, str>>>(path: P) -> String {
    let path = path.into();
    if path.is_empty() {
        return String::from(".");
    }

    #[cfg(windows)]
    {
        if path == MAIN_SEPARATOR_STR || path == FORWARD_SLASH_STR {
            return path.into_owned();
        }
    }
    #[cfg(not(windows))]
    {
        if path == MAIN_SEPARATOR_STR {
            return path.into_owned();
        }
    }

    let path = strip_last_sep(&path);
    let sep_pos = find_last_sep(path);

    match sep_pos {
        Some(idx) => {
            let parent = &path[..idx];
            if parent.is_empty() {
                MAIN_SEPARATOR_STR
            } else {
                parent
            }
        },
        None => ".",
    }
    .to_string()
}

pub fn name_extname(path: &str) -> (&str, &str) {
    let path = strip_last_sep(path);
    let sep_pos = find_last_sep(path);

    let path = match sep_pos {
        Some(idx) => &path[idx + 1..],
        None => path,
    };
    if path.starts_with('.') {
        return (path, "");
    }
    match path.rfind('.') {
        Some(idx) => path.split_at(idx),
        None => (path, ""),
    }
}

fn strip_last_sep(path: &str) -> &str {
    if ends_with_sep(path) {
        &path[..path.len() - 1]
    } else {
        path
    }
}

pub fn basename(path: String, suffix: Opt<String>) -> String {
    #[cfg(windows)]
    {
        if path.is_empty() || path == MAIN_SEPARATOR_STR || path == FORWARD_SLASH_STR {
            return String::from("");
        }
    }
    #[cfg(not(windows))]
    {
        if path.is_empty() || path == MAIN_SEPARATOR_STR {
            return String::from("");
        }
    }

    let (base, ext) = name_extname(&path);
    let mut name = [base, ext].concat();
    if let Some(suffix) = suffix.0 {
        if let Some(location) = name.rfind(&suffix) {
            name.truncate(location);
            return name;
        }
    }
    name
}

fn extname(path: String) -> String {
    let (_, ext) = name_extname(&path);
    ext.to_string()
}

fn format(obj: Object) -> String {
    let dir: String = obj.get("dir").unwrap_or_default();
    let root: String = obj.get("root").unwrap_or_default();
    let base: String = obj.get("base").unwrap_or_default();
    let name: String = obj.get("name").unwrap_or_default();
    let ext: String = obj.get("ext").unwrap_or_default();

    let mut path = String::new();
    if !dir.is_empty() {
        path.push_str(&dir);
        if !ends_with_sep(&dir) {
            path.push(MAIN_SEPARATOR);
        }
    } else if !root.is_empty() {
        path.push_str(&root);
        if !ends_with_sep(&root) {
            path.push(MAIN_SEPARATOR);
        }
    }
    if !base.is_empty() {
        path.push_str(&base);
    } else {
        path.push_str(&name);
        if !ext.is_empty() {
            if !ext.starts_with('.') {
                path.push('.');
            }
            path.push_str(&ext);
        }
    }
    path
}

fn parse(ctx: Ctx, path_str: String) -> Result<Object> {
    let obj = Object::new(ctx)?;
    let path = Path::new(&path_str);
    let parent = path
        .parent()
        .map(|p| p.to_str().unwrap())
        .unwrap_or_default();
    let filename = path
        .file_name()
        .map(|n| n.to_str().unwrap())
        .unwrap_or_default();

    let (name, extension) = name_extname(filename);

    let root = path
        .components()
        .next()
        .and_then(|c| match c {
            Component::Prefix(prefix) => prefix.as_os_str().to_str(),
            Component::RootDir => c.as_os_str().to_str(),
            _ => Some(""),
        })
        .unwrap_or_default();

    obj.set("root", root)?;
    obj.set("dir", parent)?;
    obj.set("base", [name, extension].concat())?;
    obj.set("ext", extension)?;
    obj.set("name", name)?;

    Ok(obj)
}

fn join(parts: Rest<String>) -> String {
    join_path(parts.0.iter())
}

pub fn join_path<S, I>(parts: I) -> String
where
    S: AsRef<str>,
    I: IntoIterator<Item = S>,
{
    join_path_with_separator(parts, false)
}

pub fn join_path_with_separator<S, I>(parts: I, force_posix_sep: bool) -> String
where
    S: AsRef<str>,
    I: IntoIterator<Item = S>,
{
    //fine because we're either moving or storing references
    let parts_vec: Vec<S> = parts.into_iter().collect();
    //add one slash plus drive letter
    //max is probably parts+size
    let likely_max_size = parts_vec
        .iter()
        .map(|p| p.as_ref().len() + 1)
        .sum::<usize>()
        + 10;
    let result = String::with_capacity(likely_max_size);
    join_resolve_path(parts_vec, false, result, PathBuf::new(), force_posix_sep)
}

pub fn resolve_path<S, I>(parts: I) -> Result<String>
where
    S: AsRef<str>,
    I: IntoIterator<Item = S>,
{
    resolve_path_with_separator(parts, false)
}

pub fn resolve_path_with_separator<S, I>(parts: I, force_posix_sep: bool) -> Result<String>
where
    S: AsRef<str>,
    I: IntoIterator<Item = S>,
{
    let cwd = std::env::current_dir()?;

    let mut result = cwd.clone().into_os_string().into_string().unwrap();
    //add MAIN_SEPARATOR if we're not on already MAIN_SEPARATOR
    if !result.ends_with(MAIN_SEPARATOR) {
        result.push(MAIN_SEPARATOR);
    }
    #[cfg(windows)]
    {
        if force_posix_sep {
            result = result.replace(MAIN_SEPARATOR, FORWARD_SLASH_STR);
        }
    }
    Ok(join_resolve_path(parts, true, result, cwd, force_posix_sep))
}

pub fn relative<F, T>(from: F, to: T) -> Result<String>
where
    F: AsRef<str>,
    T: AsRef<str>,
{
    let from_ref = from.as_ref();
    let to_ref = to.as_ref();
    if from_ref == to_ref {
        return Ok("".into());
    }

    let mut abs_from = None;

    if !is_absolute(from_ref) {
        abs_from = Some(
            std::env::current_dir()?.to_string_lossy().to_string() + MAIN_SEPARATOR_STR + from_ref,
        );
    }

    let mut abs_to = None;

    if !is_absolute(to_ref) {
        abs_to = Some(
            std::env::current_dir()?.to_string_lossy().to_string() + MAIN_SEPARATOR_STR + to_ref,
        );
    }

    let from_ref = abs_from.as_deref().unwrap_or(from_ref);
    let to_ref = abs_to.as_deref().unwrap_or(to_ref);

    let mut from_index = 0;
    let mut to_index = 0;
    // skip common prefix
    while from_index < from_ref.len() && to_index < to_ref.len() {
        let from_next = find_next_separator(&from_ref[from_index..])
            .unwrap_or(from_ref.len() - from_index)
            + from_index;
        let to_next =
            find_next_separator(&to_ref[to_index..]).unwrap_or(to_ref.len() - to_index) + to_index;
        if from_ref[from_index..from_next] != to_ref[to_index..to_next] {
            break;
        }
        from_index = from_next + 1; //move past the separator
        to_index = to_next + 1; //move past the separator
    }
    let mut relative = String::new();
    // add ".." for each remaining component in 'from'
    while from_index < from_ref.len() {
        let from_next = find_next_separator(&from_ref[from_index..])
            .unwrap_or(from_ref.len() - from_index)
            + from_index;
        if !relative.is_empty() {
            relative.push(MAIN_SEPARATOR);
        }
        relative.push_str("..");
        from_index = from_next + 1; // Move past the separator
    }
    // add the remaining components from 'to'
    while to_index < to_ref.len() {
        let to_next =
            find_next_separator(&to_ref[to_index..]).unwrap_or(to_ref.len() - to_index) + to_index;
        if !relative.is_empty() {
            relative.push(MAIN_SEPARATOR);
        }
        let component = &to_ref[to_index..to_next];
        if component != "." {
            relative.push_str(component);
        }
        to_index = to_next + 1; // Move past the separator
    }
    Ok(if relative.is_empty() {
        ".".into()
    } else {
        relative
    })
}

fn join_resolve_path<S, I>(
    parts: I,
    resolve: bool,
    mut result: String,
    cwd: PathBuf,
    force_posix_sep: bool,
) -> String
where
    S: AsRef<str>,
    I: IntoIterator<Item = S>,
{
    let (sep, sep_str) = if force_posix_sep {
        ('/', "/")
    } else {
        (MAIN_SEPARATOR, MAIN_SEPARATOR_STR)
    };

    let mut resolve_cow: Cow<str>;
    let mut empty = true;
    let mut prefix_len = 0;

    let mut index_stack = Vec::with_capacity(16);

    for part in parts {
        let mut part_ref: &str = part.as_ref();
        let mut start = 0;
        if resolve {
            if cfg!(not(windows)) {
                if part_ref.starts_with(MAIN_SEPARATOR) {
                    empty = false;
                    result = MAIN_SEPARATOR.into();
                    start = 1;
                }
            } else {
                let starts_with_sep = starts_with_sep(part_ref);
                if starts_with_sep {
                    let (prefix, _) = get_path_prefix(&cwd);
                    prefix_len = prefix.len();
                    result = prefix;
                    empty = false;
                    result.push(sep);
                } else {
                    let path_buf: PathBuf = PathBuf::from(part_ref);
                    if path_buf.is_absolute() {
                        empty = false;
                        let (prefix, mut components) = get_path_prefix(&path_buf);
                        if !prefix.is_empty() {
                            components.next(); //consume prefix
                        }
                        prefix_len = prefix.len();
                        result = prefix;
                        result.push(sep);
                        resolve_cow = components
                            .map(|comp| comp.as_os_str().to_str().unwrap_or_default()) // Convert each component to &str
                            .collect::<Vec<&str>>() // Collect into a vector of &str
                            .join(sep_str)
                            .into();
                        part_ref = resolve_cow.as_ref();
                    }
                }
            }
        } else if starts_with_sep(part_ref) && empty {
            empty = false;
            result.push(sep);
            start = 1;
        }

        while start < part_ref.len() {
            let end = find_next_separator(&part_ref[start..]).map_or(part_ref.len(), |i| i + start);
            match &part_ref[start..end] {
                ".." => {
                    empty = false;
                    if let Some(last_index) = index_stack.pop() {
                        result.truncate(last_index);
                    }
                },
                "" | "." => {
                    //ignore
                },
                sub_part => {
                    let len = result.len();
                    result.push_str(sub_part);
                    result.push(sep);
                    empty = false;
                    index_stack.push(len);
                },
            }
            start = end + 1;
        }
    }

    if result.len() > prefix_len + 1 && ends_with_sep(&result) {
        result.truncate(result.len() - 1);
    }

    result
}

pub fn resolve(path: Rest<String>) -> Result<String> {
    resolve_path(path.iter())
}

fn get_path_prefix(cwd: &Path) -> (String, std::iter::Peekable<std::path::Components<'_>>) {
    let mut components = cwd.components().peekable();

    let prefix = if let Some(Component::Prefix(prefix)) = components.peek() {
        prefix.as_os_str().to_str().unwrap().to_string()
    } else {
        "".into()
    };

    (prefix, components)
}

pub fn normalize<P: AsRef<str>>(path: P) -> String {
    join_path([path].iter())
}

#[allow(dead_code)] //used by windows
fn starts_with_sep(path: &str) -> bool {
    matches!(path.as_bytes().first().unwrap_or(&0), b'/' | b'\\')
}

#[cfg(windows)]
fn ends_with_sep(path: &str) -> bool {
    matches!(path.as_bytes().last().unwrap_or(&0), b'/' | b'\\')
}

#[cfg(not(windows))]
fn ends_with_sep(path: &str) -> bool {
    path.ends_with(MAIN_SEPARATOR)
}

#[cfg(windows)]
pub fn is_absolute(path: &str) -> bool {
    starts_with_sep(path) || PathBuf::from(path).is_absolute()
}

#[cfg(not(windows))]
pub fn is_absolute(path: &str) -> bool {
    path.starts_with(MAIN_SEPARATOR)
}

impl ModuleDef for PathModule {
    fn declare(declare: &Declarations) -> Result<()> {
        declare.declare("basename")?;
        declare.declare("dirname")?;
        declare.declare("extname")?;
        declare.declare("format")?;
        declare.declare("parse")?;
        declare.declare("join")?;
        declare.declare("resolve")?;
        declare.declare("relative")?;
        declare.declare("normalize")?;
        declare.declare("isAbsolute")?;
        declare.declare("delimiter")?;
        declare.declare("sep")?;

        declare.declare("default")?;
        Ok(())
    }

    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &Exports<'js>) -> Result<()> {
        export_default(ctx, exports, |default| {
            default.set("dirname", Func::from(dirname::<String>))?;
            default.set("basename", Func::from(basename))?;
            default.set("extname", Func::from(extname))?;
            default.set("format", Func::from(format))?;
            default.set("parse", Func::from(parse))?;
            default.set("join", Func::from(join))?;
            default.set("relative", Func::from(relative::<String, String>))?;
            default.set("resolve", Func::from(resolve))?;
            default.set("normalize", Func::from(normalize::<String>))?;
            default.set("isAbsolute", Func::from(|s: String| is_absolute(&s)))?;
            default.prop("delimiter", DELIMITER.to_string())?;
            default.prop("sep", MAIN_SEPARATOR.to_string())?;
            Ok(())
        })
    }
}

impl From<PathModule> for ModuleInfo<PathModule> {
    fn from(val: PathModule) -> Self {
        ModuleInfo {
            name: "path",
            module: val,
        }
    }
}

