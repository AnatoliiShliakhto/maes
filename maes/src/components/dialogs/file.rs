use crate::{prelude::*, services::*};
use ::rfd;

pub fn add_image_dialog(entity: impl Into<String>, id: impl Into<String>, on_success: Callback) {
    let entity_id = entity.into();
    let id = id.into();

    spawn(async move {
        let config = ConfigService::read();
        let Some(path) = rfd::AsyncFileDialog::new()
            .set_title(t!("image-dialog-title"))
            .set_directory(&config.recent.images)
            .add_filter(
                t!("image-dialog-filter"),
                &["jpeg", "png", "bmp", "jpg", "webp"],
            )
            .pick_file()
            .await
        else {
            return;
        };
        ConfigService::with_mut(|config| {
             if let Some(path) = path.path().parent() {
                 config.recent.images = path.to_path_buf()
             }

        }).ok();
        api_call!(
            POST,
            format!("/api/v1/manager/images/{entity_id}/{item_id}", entity_id = entity_id, item_id = id),
            AddImagePayload {
                path: path.path().to_string_lossy().to_string(),
            },
            on_success = move || on_success.call(())
        )
    });
}
