use crate::{prelude::*, services::*};

const _BOOTSTRAP_ICONS_WOFF2: Asset = asset!(
    "/assets/bootstrap-icons.woff2",
    AssetOptions::builder()
        .with_hash_suffix(false)
        .into_asset_options()
);
const _BOOTSTRAP_ICONS_WOFF: Asset = asset!(
    "/assets/bootstrap-icons.woff",
    AssetOptions::builder()
        .with_hash_suffix(false)
        .into_asset_options()
);
const _UAFSANS_REGULAR_TTF: Asset = asset!(
    "/assets/uafsans-regular.ttf",
    AssetOptions::builder()
        .with_hash_suffix(false)
        .into_asset_options()
);
const _UAFSANS_MEDIUM_TTF: Asset = asset!(
    "/assets/uafsans-medium.ttf",
    AssetOptions::builder()
        .with_hash_suffix(false)
        .into_asset_options()
);
const _UAFSANS_SEMIBOLD_TTF: Asset = asset!(
    "/assets/uafsans-semibold.ttf",
    AssetOptions::builder()
        .with_hash_suffix(false)
        .into_asset_options()
);
const _UAFSANS_BOLD_TTF: Asset = asset!(
    "/assets/uafsans-bold.ttf",
    AssetOptions::builder()
        .with_hash_suffix(false)
        .into_asset_options()
);
const APP_ICON: Asset = asset!(
    "/assets/icon.png",
    AssetOptions::builder()
        .with_hash_suffix(false)
        .into_asset_options()
);
const _OK_CHEVRON: Asset = asset!(
    "/assets/ok.svg",
    AssetOptions::builder()
        .with_hash_suffix(false)
        .into_asset_options()
);
const _FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const MAIN_JS: Asset = asset!("/assets/main.bundle.js");
const BOOTSTRAP_ICONS_CSS: Asset = asset!("/assets/bootstrap-icons.css");

#[component]
pub fn Head(is_main: bool) -> Element {
    let _ = (
        _OK_CHEVRON.resolve(),
        _BOOTSTRAP_ICONS_WOFF2.resolve(),
        _BOOTSTRAP_ICONS_WOFF.resolve(),
        _UAFSANS_REGULAR_TTF.resolve(),
        _UAFSANS_MEDIUM_TTF.resolve(),
        _UAFSANS_SEMIBOLD_TTF.resolve(),
        _UAFSANS_BOLD_TTF.resolve(),
    );
    let config = ConfigService::read();

    rsx! {
        if is_main {
            document::Script { r#"document.documentElement.style.visibility='hidden';"#}
        } else {
            document::Script {
                r#"
                    document.documentElement.style.visibility='hidden';
                    document.documentElement.setAttribute('data-theme', '{config.theme}');
                "#
            }
        }
        document::Script { src: MAIN_JS }
        document::Link { rel: "icon", href: APP_ICON }
        document::Link { rel: "stylesheet", href: BOOTSTRAP_ICONS_CSS }
        document::Link {
            rel: "stylesheet",
            href: TAILWIND_CSS,
            onload: r#"document.documentElement.style.visibility='visible';"#,
        }
    }
}
