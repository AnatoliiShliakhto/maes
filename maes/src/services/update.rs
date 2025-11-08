use crate::prelude::*;
use ::semver::Version;
use ::serde::Deserialize;
use ::std::io;
use ::tokio::{fs, process::Command};

#[derive(Clone, Deserialize)]
pub struct GitHubAsset {
    pub name: String,
    pub browser_download_url: String,
}

#[derive(Clone, Deserialize)]
struct GitHubEntity {
    pub name: String,
    pub tag_name: String,
    pub assets: Vec<GitHubAsset>,
}

pub struct UpdateService;

impl UpdateService {
    pub fn check_latest_release(mut download_url_sig: Signal<String>) {
        let url = "https://api.github.com/repos/AnatoliiShliakhto/maes/releases/latest";

        let on_success = move |body: GitHubEntity| {
            if let Some(app_ver) = parse_semver(env!("CARGO_PKG_VERSION"))
                && let Some(server_ver) = parse_semver(&body.tag_name)
                && app_ver < server_ver
            {
                let Some(download_url) = body
                    .assets
                    .iter()
                    .find(|a| a.name.contains(".msi"))
                    .map(|a| a.browser_download_url.clone())
                else {
                    return;
                };

                download_url_sig.set(download_url)
            }
        };

        api_fetch!(GET, url, on_success = on_success, on_error = move |_| (),);
    }

    pub fn update(url: String) {
        let Some(path) = dirs::download_dir() else { return };
        let path = path.join("maes.msi");
        tokio::spawn(async move {
            if path.exists() {
                fs::remove_file(&path).await.ok();
            }
            if download_to(&path, &url).await.is_err() {
                return;
            }
            _ = run_msi(&path).await;
        });
    }
}

fn parse_semver(s: &str) -> Option<Version> {
    let s = s.strip_prefix('v').unwrap_or(s);
    Version::parse(s).ok()
}

async fn download_to(path: &std::path::Path, url: &str) -> io::Result<()> {
    if let Some(p) = path.parent() {
        fs::create_dir_all(p).await?;
    }
    let resp = reqwest::get(url).await.map_err(to_io)?;
    let bytes = resp.bytes().await.map_err(to_io)?;
    fs::write(path, &bytes).await
}

fn to_io<E: std::fmt::Display>(e: E) -> io::Error {
    io::Error::other(e.to_string())
}

async fn run_msi(path: &std::path::Path) -> io::Result<()> {
    let status = Command::new("msiexec")
        .args(["/i", path.to_string_lossy().as_ref()])
        .status()
        .await?;
    if !status.success() {
        return Err(io::Error::other(format!("exit: {status}")));
    }
    Ok(())
}
