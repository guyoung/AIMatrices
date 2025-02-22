// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use llrt_utils::{bytes::ObjectBytes, result::ResultExt};
use rquickjs::{Ctx, Result};

pub fn write_file_sync<'js>(ctx: Ctx<'js>, path: String, bytes: ObjectBytes<'js>) -> Result<()> {
    std::fs::write(&path, bytes.as_bytes())
        .or_throw_msg(&ctx, &["Can't write \"{}\"", &path].concat())?;

    Ok(())
}
