use std::{env, fs};
use zed::settings::LspSettings;
use zed_extension_api::{self as zed, serde_json::json, LanguageServerId, Result};

const SERVER_PATH: &str = "node_modules/.bin/stylelint-lsp";
const PACKAGE_NAME: &str = "stylelint-lsp";

struct StylelintPlusExtension {
    did_find_server: bool,
}

impl StylelintPlusExtension {
    fn server_exists(&self) -> bool {
        fs::metadata(SERVER_PATH).map_or(false, |stat| stat.is_file())
    }

    fn server_script_path(&mut self, language_server_id: &LanguageServerId) -> Result<String> {
        let server_exists = self.server_exists();
        if self.did_find_server && server_exists {
            return Ok(SERVER_PATH.to_string());
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let version = zed::npm_package_latest_version(PACKAGE_NAME)?;

        if !server_exists
            || zed::npm_package_installed_version(PACKAGE_NAME)?.as_ref() != Some(&version)
        {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            let result = zed::npm_install_package(PACKAGE_NAME, &version);
            match result {
                Ok(()) => {
                    if !self.server_exists() {
                        Err(format!(
                            "installed package '{PACKAGE_NAME}' did not contain expected path '{SERVER_PATH}'",
                        ))?;
                    }
                }
                Err(error) => {
                    if !self.server_exists() {
                        Err(error)?;
                    }
                }
            }
        }

        self.did_find_server = true;
        Ok(SERVER_PATH.to_string())
    }
}

impl zed::Extension for StylelintPlusExtension {
    fn new() -> Self {
        Self {
            did_find_server: false,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let server_path = self.server_script_path(language_server_id)?;
        Ok(zed::Command {
            command: env::current_dir()
                .unwrap()
                .join(&server_path)
                .to_string_lossy()
                .to_string(),
            args: vec!["--stdio".to_string()],
            env: vec![Default::default()],
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        let settings = LspSettings::for_worktree(server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| {
                lsp_settings
                    .settings
                    .map(|settings| {
                        json!({
                            "": {
                                "stylelintplus": settings,
                            }
                        })
                    })
                    .or_else(|| {
                        Some(json!({
                            "": {
                                "stylelintplus": {},
                            }
                        }))
                    })
            })
            .unwrap_or_default();
        Ok(Some(settings))
    }
}

zed::register_extension!(StylelintPlusExtension);
