use ::base64::{Engine, engine::general_purpose::STANDARD};

pub struct QrGenerator;

impl QrGenerator {
    pub fn text(text: impl Into<String>, size: usize) -> String {
        let svg_content = qrcode_generator::to_svg_to_string(
            text.into(),
            qrcode_generator::QrCodeEcc::Low,
            size,
            None::<&str>,
        )
        .unwrap_or_default();
        let base64_svg = STANDARD.encode(svg_content);
        format!("data:image/svg+xml;base64,{base64_svg}")
    }
}
