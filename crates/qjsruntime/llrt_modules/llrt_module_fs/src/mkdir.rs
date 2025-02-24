// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
#[cfg(unix)]
use std::os::unix::prelude::PermissionsExt;

use llrt_utils::result::ResultExt;
use ring::rand::{SecureRandom, SystemRandom};
use rquickjs::{function::Opt, Ctx, Object, Result};

pub fn mkdir_sync<'js>(ctx: Ctx<'js>, path: String, options: Opt<Object<'js>>) -> Result<String> {
    let (recursive, mode) = get_params(options);

    if recursive {
        std::fs::create_dir_all(&path)
    } else {
        std::fs::create_dir(&path)
    }
    .or_throw_msg(&ctx, &["Can't create dir \"", &path, "\""].concat())?;

    #[cfg(unix)]
    {
        std::fs::set_permissions(&path, PermissionsExt::from_mode(mode))
            .or_throw_msg(&ctx, &["Can't set permissions of \"", &path, "\""].concat())?;
    }
    #[cfg(not(unix))]
    {
        _ = mode;
    }

    Ok(path)
}

fn get_params(options: Opt<Object>) -> (bool, u32) {
    let mut recursive = false;
    let mut mode = 0o777;

    if let Some(options) = options.0 {
        recursive = options.get("recursive").unwrap_or_default();
        mode = options.get("mode").unwrap_or(0o777);
    }
    (recursive, mode)
}

const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

fn random_chars(len: usize) -> String {
    let random = SystemRandom::new();

    let mut bytes = vec![0u8; len];
    random.fill(&mut bytes).unwrap();
    bytes
        .iter()
        .map(|&byte| {
            let idx = (byte as usize) % CHARS.len();
            CHARS[idx] as char
        })
        .collect::<String>()
}

pub fn mkdtemp_sync(ctx: Ctx<'_>, prefix: String) -> Result<String> {
    let path = [prefix.as_str(), random_chars(6).as_str()].join(",");
    std::fs::create_dir_all(&path)
        .or_throw_msg(&ctx, &["Can't create dir \"", &path, "\""].concat())?;
    Ok(path)
}
