use ::serde::{Deserialize, Serialize};
use ::shared::common::*;
use ::wasm_bindgen::prelude::*;

#[derive(Copy, Clone)]
pub struct BarcodeScanner;

impl BarcodeScanner {
    pub async fn scan() -> Result<String> {
        if &JsValue::from_str("granted") != request_permissions().await.as_ref() {
            Err("camera-permissions-denied".to_string())?
        }
        let opts = serde_wasm_bindgen::to_value(&ScanOptions {
            windowed: true,
            formats: vec!["QR_CODE".to_string()],
        })
        .map_err(|_| "serialization-error")?;
        let payload = scan(opts).await;
        let scanned = serde_wasm_bindgen::from_value::<ScanResult>(payload)
            .map_err(|_| "deserialization-error")?;
        Ok(scanned.content)
    }

    pub fn cancel() {
        cancel();
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "barcodeScanner"], js_name = requestPermissions)]
    async fn request_permissions() -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "barcodeScanner"], js_name = scan)]
    async fn scan(options: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "barcodeScanner"], js_name = cancel)]
    fn cancel() -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct ScanOptions {
    pub windowed: bool,
    pub formats: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct ScanResult {
    pub content: String,
    pub format: String,
}
