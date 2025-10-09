#![allow(dead_code)]
use crate::prelude::*;
use ::indexmap::IndexMap;
use ::std::time::Duration;

#[derive(Clone, PartialEq)]
pub enum ToastType {
    Success,
    Error,
    Warning,
    Info,
}

impl ToastType {
    fn css_class(&self) -> &'static str {
        match self {
            ToastType::Success => "alert-success",
            ToastType::Error => "alert-error",
            ToastType::Warning => "alert-warning",
            ToastType::Info => "alert-info",
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            ToastType::Success => "bi bi-check-circle-fill",
            ToastType::Error => "bi bi-x-circle-fill",
            ToastType::Warning => "bi bi-exclamation-triangle-fill",
            ToastType::Info => "bi bi-info-circle-fill",
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Toast {
    pub id: String,
    pub message: String,
    pub r#type: ToastType,
    pub duration: u64, // millis
    pub created_at: u64,
}

impl Toast {
    pub fn new(message: String, r#type: ToastType, duration: Option<u64>) -> Self {
        Self {
            id: safe_nanoid!(),
            message,
            r#type,
            duration: duration.unwrap_or(3000),
            created_at: chrono::Utc::now().timestamp_millis() as u64,
        }
    }

    pub fn success(message: String) -> Self {
        Self::new(message, ToastType::Success, None)
    }
    pub fn error(message: String) -> Self {
        Self::new(message, ToastType::Error, Some(5000))
    }
    pub fn warning(message: String) -> Self {
        Self::new(message, ToastType::Warning, Some(4000))
    }
    pub fn info(message: String) -> Self {
        Self::new(message, ToastType::Info, None)
    }
}

static TOASTS: GlobalSignal<IndexMap<String, Toast>> = Signal::global(IndexMap::new);

pub struct ToastService;

impl ToastService {
    pub fn show(toast: Toast) {
        TOASTS.write().insert(toast.id.clone(), toast);
    }

    #[inline]
    pub fn success(message: impl Into<String>) {
        Self::show(Toast::success(message.into()));
    }
    #[inline]
    pub fn error(message: impl Into<String>) {
        Self::show(Toast::error(message.into()));
    }
    #[inline]
    pub fn warning(message: impl Into<String>) {
        Self::show(Toast::warning(message.into()));
    }
    #[inline]
    pub fn info(message: impl Into<String>) {
        Self::show(Toast::info(message.into()));
    }

    pub fn remove(id: &str) {
        TOASTS.write().shift_remove(id);
    }

    pub fn clear_all() {
        TOASTS.write().clear();
    }
}

#[component]
fn ToastComponent(toast: Toast) -> Element {
    let toast_id = toast.id.clone();
    let mut is_visible = use_signal(|| false);

    let _show_animation = use_memo(move || {
        spawn(async move {
            tokio::time::sleep(Duration::from_millis(50)).await;
            is_visible.set(true);
        });
        toast.created_at
    });

    use_future(move || async move {
        tokio::time::sleep(Duration::from_millis(30)).await;
        is_visible.set(true);
    });

    let handle_remove = use_callback(move |_| {
        ToastService::remove(&toast_id);
    });

    let visibility_class = if is_visible() {
        "opacity-100 translate-x-0"
    } else {
        "opacity-0 translate-x-full"
    };

    rsx! {
        div {
            class: "alert {toast.r#type.css_class()} shadow-lg mb-2 cursor-pointer \
                    transform transition-all duration-300 ease-in-out {visibility_class}",
            onclick: move |_| handle_remove.call(()),
            div {
                class: "flex items-center justify-between w-full",
                div {
                    class: "flex items-center gap-3",
                    i { class: "{toast.r#type.icon()} text-lg" }
                    span { class: "text-sm", "{toast.message}" }
                }
            }
        }
    }
}

#[component]
pub fn ToastContainer() -> Element {
    let toasts = use_memo(move || TOASTS.read().values().cloned().collect::<Vec<_>>())();

    use_future(move || async move {
        loop {
            tokio::time::sleep(Duration::from_millis(500)).await;
            let now = chrono::Utc::now().timestamp_millis() as u64;
            TOASTS
                .write()
                .retain(|_, t| now.saturating_sub(t.created_at) < t.duration);
        }
    });

    rsx! {
        div {
            class: "toast z-100",
            for toast in toasts {
                ToastComponent {
                    key: "{toast.id}",
                    toast
                }
            }
        }
    }
}
