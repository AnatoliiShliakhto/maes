use crate::{common::*, payloads::*};
use ::arboard::Clipboard as ArBoard;
use ::csv::ReaderBuilder;
use ::serde::{Serialize, de::DeserializeOwned};

#[derive(Copy, Clone)]
pub struct Clipboard;

impl Clipboard {
    pub fn copy_text(payload: impl AsRef<str>) -> Result<()> {
        let mut clipboard = ArBoard::new().map_err(map_log_err)?;
        clipboard.set_text(payload.as_ref()).map_err(map_log_err)
    }

    pub fn paste_text() -> Result<String> {
        let mut clipboard = ArBoard::new().map_err(map_log_err)?;
        clipboard.get_text().map_err(map_log_err)
    }

    pub fn copy_json<T: Serialize + 'static>(payload: T) -> Result<()> {
        let text = serde_json::to_string_pretty(&payload).map_err(map_log_err)?;
        let mut clipboard = ArBoard::new().map_err(map_log_err)?;
        clipboard.set_text(text).map_err(map_log_err)
    }

    pub fn paste_json<T: DeserializeOwned + 'static>() -> Result<T> {
        let mut clipboard = ArBoard::new().map_err(map_log_err)?;
        let text = clipboard.get_text().map_err(map_log_err)?;
        serde_json::from_str(&text).map_err(map_log_err)
    }

    pub fn paste_students() -> Result<Vec<AddStudentPayload>> {
        let data = Self::paste_text()?;
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'\t')
            .flexible(true)
            .from_reader(data.as_bytes());

        let mut out = Vec::new();
        for rec in rdr.records() {
            let rec = match rec {
                Ok(r) => r,
                Err(_) => continue,
            };
            let first = rec.get(0).unwrap_or("").trim();
            let second = rec.get(1).map(|s| s.trim());

            if let Some(name) = second {
                out.push(AddStudentPayload {
                    rank: if first.is_empty() { None } else { Some(first.to_string()) },
                    name: name.to_string(),
                });
            } else {
                out.push(AddStudentPayload {
                    rank: None,
                    name: first.to_string(),
                });
            }
        }
        Ok(out)
    }
}
