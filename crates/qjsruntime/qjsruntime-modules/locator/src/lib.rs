// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

pub use self::modules::*;

mod modules {
    #[cfg(feature = "assert")]
    pub use llrt_module_assert as assert;
    #[cfg(feature = "buffer")]
    pub use llrt_module_buffer as buffer;
    #[cfg(feature = "crypto")]
    pub use llrt_module_crypto as crypto;
    #[cfg(feature = "events")]
    pub use llrt_module_events as events;
    #[cfg(feature = "exceptions")]
    pub use llrt_module_exceptions as exceptions;
    #[cfg(feature = "fs")]
    pub use llrt_module_fs as fs;
    #[cfg(feature = "path")]
    pub use llrt_module_path as path;
    #[cfg(feature = "perf-hooks")]
    pub use llrt_module_perf_hooks as perf_hooks;
    #[cfg(feature = "timers")]
    pub use llrt_module_timers as timers;
    #[cfg(feature = "url")]
    pub use llrt_module_url as url;

    #[cfg(feature = "navigator")]
    pub use qjsruntime_module_navigator as navigator;
    #[cfg(feature = "process")]
    pub use qjsruntime_module_process as process;
    #[cfg(feature = "zlib")]
    pub use qjsruntime_module_zlib as zlib;


    #[cfg(feature = "http")]
    pub use qjsruntime_module_http as http;
    #[cfg(feature = "key-value")]
    pub use qjsruntime_module_key_value as key_value;
    #[cfg(feature = "variables")]
    pub use qjsruntime_module_variables as variables;

    #[cfg(feature = "openai")]
    pub use qjsruntime_module_openai as openai;
    #[cfg(feature = "ollama")]
    pub use qjsruntime_module_ollama as ollama;

    #[cfg(feature = "dbs")]
    pub use qjsruntime_module_dbs as dbs;
}
pub use llrt_utils::time;
