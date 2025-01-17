use std::fs;
use zed_extension_api::{
    self as zed, settings::LspSettings, Architecture, Command, DownloadedFileType,
    GithubReleaseOptions, LanguageServerId, LanguageServerInstallationStatus, Os, Result, Worktree,
};

struct Extension {
    cached_binary_path: Option<String>,
}

impl Extension {
    fn server_bin_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<String> {
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        let (os, arch) = zed::current_platform();
        let binary_name = format!(
            "wat_server{extension}",
            extension = match os {
                Os::Mac | Os::Linux => "",
                Os::Windows => ".exe",
            }
        );

        if let Some(path) = worktree.which(&binary_name) {
            self.cached_binary_path = Some(path.clone());
            return Ok(path);
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let release = zed::latest_github_release(
            "g-plane/wasm-language-tools",
            GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;
        let version_dir = format!("wasm-language-tools-{}", release.version);
        let binary_path = format!("{version_dir}/{binary_name}");
        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            let url = format!(
                "https://github.com/g-plane/wasm-language-tools/releases/download/{}/wat_server-{}-{}.zip",
                release.version,
                match arch {
                    Architecture::Aarch64 => "arm",
                    Architecture::X8664 => "x86_64",
                    Architecture::X86=>"x86",
                },
                match os {
                    Os::Mac => "macos",
                    Os::Linux => "linux",
                    Os::Windows => "windows",
                },
            );
            zed::download_file(&url, &version_dir, DownloadedFileType::Zip)?;
            zed::make_file_executable(&binary_path)?;
            if let Ok(entries) = fs::read_dir(".") {
                for entry in entries {
                    if let Ok(entry) = entry {
                        if entry
                            .file_name()
                            .to_str()
                            .is_some_and(|name| name != &version_dir)
                        {
                            let _ = fs::remove_dir_all(entry.path());
                        }
                    }
                }
            }
        }

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
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command> {
        let mut command = None;
        let mut args = vec![];
        if let Some(binary) = LspSettings::for_worktree("wasm-language-tools", worktree)
            .ok()
            .and_then(|settings| settings.binary)
        {
            command = binary.path;
            if let Some(arguments) = binary.arguments {
                args = arguments;
            }
        }
        Ok(Command {
            command: if let Some(command) = command {
                command
            } else {
                self.server_bin_path(language_server_id, worktree)?
            },
            args,
            env: Default::default(),
        })
    }
}

zed::register_extension!(Extension);
