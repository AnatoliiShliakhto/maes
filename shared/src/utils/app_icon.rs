use ::dioxus::desktop::tao::window::Icon;

fn load_icon_data(icon_bytes: &[u8]) -> Result<(u32, u32, Vec<u8>), Box<dyn std::error::Error>> {
    let img = image::load_from_memory(icon_bytes)?.to_rgba8();
    let (width, height) = img.dimensions();
    let rgba = img.into_raw();
    Ok((width, height, rgba))
}

pub fn create_window_icon(icon_bytes: &[u8]) -> Option<Icon> {
    if let Ok((width, height, rgba)) = load_icon_data(icon_bytes) {
        Icon::from_rgba(rgba, width, height).ok()
    } else {
        None
    }
}