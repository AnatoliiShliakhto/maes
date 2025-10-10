use crate::prelude::*;

pub fn close_window() {
    if let Some(window) = web_sys::window() {
        window.close().ok();
    }

    spawn(async move {
        gloo_timers::future::TimeoutFuture::new(300).await;
        document::eval("window.close();");
        document::eval("window.location.href = window.location.origin;");
    });
}