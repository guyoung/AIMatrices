use std::path::{Path, PathBuf};

use anyhow::{ensure, Context};

use path_absolutize::*;

use spin_app::AppComponent;
use spin_common::ui::quoted_path;

use crate::FilesMounter;

pub struct SpinFilesMounter {
    working_dir: PathBuf,
    allow_transient_writes: bool,
}

impl SpinFilesMounter {
    pub fn new(working_dir: impl Into<PathBuf>, allow_transient_writes: bool) -> Self {
        Self {
            working_dir: working_dir.into(),
            allow_transient_writes,
        }
    }
}

impl FilesMounter for SpinFilesMounter {
    fn mount_files(
        &self,
        app_component: &AppComponent,
        mut ctx: crate::MountFilesContext,
    ) -> anyhow::Result<()> {
        let working_dir = self.working_dir.absolutize()?.to_path_buf();

        let component_id = app_component.locked.id.to_string();

        let codes_path = working_dir.join("codes").join(component_id.as_str());

        if codes_path.exists() {
            ctx.preopened_dir(codes_path, "/codes", self.allow_transient_writes)?;
        }

        let shared_path = working_dir.join("files").join("__shared__");

        if !shared_path.exists() {
            std::fs::create_dir_all(&shared_path)?;
        }

        let cached_path = working_dir.join("files").join("__cached__");

        if !cached_path.exists() {
            std::fs::create_dir_all(&cached_path)?;
        }

        ctx.preopened_dir(shared_path, "/__shared__", self.allow_transient_writes)?;
        ctx.preopened_dir(cached_path, "/__cached__", self.allow_transient_writes)?;

        for content_dir in app_component.files() {
            let path = content_dir
                .content
                .source
                .as_deref()
                .with_context(|| format!("Missing 'source' on files mount {content_dir:?}"))?;

            let base_dir = working_dir.join("files").join(component_id.as_str());

            let source_path = sanitize_path(&Path::new(path), &base_dir);

            if !source_path.exists() {
                std::fs::create_dir_all(&source_path)?;
            }

            ensure!(
                source_path.is_dir(),
                "SpinFilesMounter only supports directory mounts; {} is not a directory",
                quoted_path(&source_path),
            );

            let guest_path = &content_dir.path;

            let guest_path = guest_path
                .to_str()
                .with_context(|| format!("guest path {guest_path:?} not valid UTF-8"))?;

            ctx.preopened_dir(source_path, guest_path, self.allow_transient_writes)?;
        }
        Ok(())
    }
}

fn sanitize_path(path: &Path, base_dir: &Path) -> PathBuf {
    let mut sanitized_path = PathBuf::from(base_dir);

    for component in path.components() {
        match component {
            std::path::Component::Normal(name) => sanitized_path.push(name),
            std::path::Component::RootDir | std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {}
            std::path::Component::Prefix(_) => {}
        }
    }

    sanitized_path
}
