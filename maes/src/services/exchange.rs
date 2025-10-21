use crate::{prelude::*, services::*};
use ::chrono::Local;

#[derive(Copy, Clone)]
pub struct Exchange;

impl Exchange {
    pub fn export(entities: Vec<String>) {
        spawn(async move {
            let claims = AuthService::claims();
            let now = Local::now();
            let filename = format!(
                "{}.{}.maes",
                claims
                    .workspace
                    .replace("/", "")
                    .replace("\\", "")
                    .replace("\"", "")
                    .replace("'", ""),
                if entities.is_empty() { "workspace".to_string() } else { now.format("%Y%m%d%H%M").to_string() }
            );
            let config = ConfigService::read();
            let Some(path) = rfd::AsyncFileDialog::new()
                .set_title(t!("export-dialog-title"))
                .set_directory(&config.recent.export)
                .set_can_create_directories(true)
                .set_file_name(filename)
                .add_filter(t!("maes-dialog-filter"), &["maes"])
                .save_file()
                .await
            else {
                return;
            };
            ConfigService::with_mut(|config| {
                if let Some(path) = path.path().parent() {
                    config.recent.export = path.to_path_buf()
                }
            })
            .ok();
            api_call!(
                POST,
                "/api/v1/exchange/export",
                ExchangeExportPayload {
                    path: path.path().to_string_lossy().to_string(),
                    entities,
                },
            )
        });
    }

    pub fn import() {
        spawn(async move {
            let config = ConfigService::read();
            let Some(path) = rfd::AsyncFileDialog::new()
                .set_title(t!("import-dialog-title"))
                .set_directory(&config.recent.import)
                .set_can_create_directories(true)
                .add_filter(t!("maes-dialog-filter"), &["maes"])
                .pick_file().await else { return};
            ConfigService::with_mut(|config| {
                if let Some(path) = path.path().parent() {
                    config.recent.import = path.to_path_buf()
                }
            })
                .ok();
            api_call!(
                POST,
                "/api/v1/exchange/import",
                ExchangeImportPayload {
                    path: path.path().to_string_lossy().to_string(),
                },
            )
        });
    }
}
