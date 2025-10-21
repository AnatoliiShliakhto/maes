use crate::{prelude::*, services::*};
use ::shared::services::dispatcher::*;

pub fn bind_msg_dispatcher() -> Coroutine<()> {
    dispatcher().msg_send(DispatcherMessage::None);
    use_coroutine(move |_rx: UnboundedReceiver<()>| async move {
        let mut rx = dispatcher().msg_subscribe();
        loop {
            if rx.changed().await.is_err() {
                break;
            }
            match &*rx.borrow() {
                DispatcherMessage::None => {},
                DispatcherMessage::Info(msg) => { ToastService::info(t!(msg)) },
                DispatcherMessage::Warning(msg) => { ToastService::warning(t!(msg)) },
                DispatcherMessage::Error(msg) => { ToastService::error(t!(msg)) },
                DispatcherMessage::Success(msg) => { ToastService::success(t!(msg)) },
            };
        }
    })
}

pub fn bind_task_dispatcher(on_success: Option<Callback>, on_error: Option<Callback>) -> Coroutine<()> {
    dispatcher().task_send(DispatcherTask::None);
    use_coroutine(move |_rx: UnboundedReceiver<()>| async move {
        let mut rx = dispatcher().task_subscribe();
        loop {
            if rx.changed().await.is_err() {
                break;
            }
            match &*rx.borrow() {
                DispatcherTask::Finished => {
                    if let Some(on_success) = on_success {
                        on_success.call(());
                    }
                }
                DispatcherTask::Failed => {
                    if let Some(on_error) = on_error {
                        on_error.call(());
                    }
                }
                _ => ()
            };
        }
    })
}