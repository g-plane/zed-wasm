use std::fs;
use zed::Result;
use zed_extension_api as zed;

struct Extension {
    cached_binary_path: Option<String>,
}

impl Extension {
    fn server_bin_path(&mut self) -> Result<String> {
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        let (os, _arch) = zed::current_platform();
        let binary_path = format!(
            "wat_server{extension}",
            extension = match os {
                zed::Os::Mac | zed::Os::Linux => "",
                zed::Os::Windows => ".exe",
            }
        );

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for Extension {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: self.server_bin_path()?,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(Extension);
