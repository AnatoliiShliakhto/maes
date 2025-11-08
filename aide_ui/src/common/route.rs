use crate::{elements::*, pages::*, prelude::*};

#[derive(Clone, PartialEq, Routable)]
#[rustfmt::skip]
#[allow(clippy::enum_variant_names)]
pub enum Route {
    #[layout(CleanLayout)]
    #[route("/qr_scanner")]
    QrScanner {},
    #[end_layout]
    
    #[layout(AppLayout)]
    #[route("/")]
    Home {}
}