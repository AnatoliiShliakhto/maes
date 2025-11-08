use ::dioxus::core::use_drop;
use crate::{prelude::*, services::*};

#[component]
pub fn QrScanner() -> Element {
    let navigator = use_navigator();
    let mut scanned_data = use_signal(String::new);

    use_effect(move || {
        spawn(async move {
            if let Ok(data) = BarcodeScanner::scan().await {
                scanned_data.set(data);
            }
        });
    });

    use_drop(BarcodeScanner::cancel);

    rsx! {
        div {
            class: "flex-visible",

            div {
                class: "relative w-full h-full flex flex-col items-center justify-between p-8",

                div {
                    class: "w-full max-w-md z-20",
                    div {
                        class: "bg-black/20 backdrop-blur-lg rounded-2xl p-4",
                        div {
                            class: "flex items-center gap-3",
                            div {
                                class: "w-12 h-12 rounded-full bg-gradient-to-br from-green-400 to-emerald-500 flex items-center justify-center shadow-lg animate-pulse",
                                i { class: "bi bi-qr-code-scan text-white text-xl" }
                            }
                            div {
                                class: "flex-1",
                                h2 { class: "text-white font-bold text-lg", "Сканування..." }
                                p { class: "text-gray-300 text-sm", "Наведіть камеру на QR-код" }
                            }
                        }
                    }
                }

                div {
                    class: "relative flex items-center justify-center z-10",

                    div {
                        class: "relative w-72 h-72 rounded-3xl",
                        style: "box-shadow: 0 0 0 9999px rgba(0, 0, 0, 0.5);",

                        div {
                            class: "absolute top-0 left-0 w-16 h-16 border-t-4 border-l-4 border-slate-400 rounded-tl-3xl",
                        }
                        div {
                            class: "absolute top-0 right-0 w-16 h-16 border-t-4 border-r-4 border-slate-400 rounded-tr-3xl",
                        }
                        div {
                            class: "absolute bottom-0 left-0 w-16 h-16 border-b-4 border-l-4 border-slate-400 rounded-bl-3xl",
                        }
                        div {
                            class: "absolute bottom-0 right-0 w-16 h-16 border-b-4 border-r-4 border-slate-400 rounded-br-3xl",
                        }

                        div {
                            class: "absolute left-0 right-0 h-1",
                            style: "animation: scan 2s ease-in-out infinite;",
                            div {
                                class: "h-full bg-gradient-to-r from-transparent via-slate-400 to-transparent",
                                style: "box-shadow: 0 0 20px rgba(148, 163, 184, 0.5);"
                            }
                        }

                        div {
                            class: "absolute inset-0 flex items-center justify-center pointer-events-none",
                            i { class: "bi bi-qr-code text-6xl text-slate-400/50" }
                        }
                    }
                }

                div {
                    class: "w-full max-w-md z-20 space-y-4",

                    div {
                        class: "flex gap-2 justify-center",
                        div { class: "w-2 h-2 rounded-full bg-neutral-300 animate-pulse" }
                        div { class: "w-2 h-2 rounded-full bg-neutral-300 animate-pulse", style: "animation-delay: 0.2s;" }
                        div { class: "w-2 h-2 rounded-full bg-neutral-300 animate-pulse", style: "animation-delay: 0.4s;" }
                    }

                    div {
                        class: "flex gap-3",

                        // button {
                        //     class: "flex-1 bg-black bg-opacity-60 backdrop-blur-lg border border-white border-opacity-20 rounded-2xl p-4 hover:bg-opacity-80 transition-all shadow-xl",
                        //     div {
                        //         class: "flex flex-col items-center gap-2",
                        //         i { class: "bi bi-lightning-charge-fill text-yellow-400 text-2xl" }
                        //         span { class: "text-white text-sm font-medium", "Вспышка" }
                        //     }
                        // }

                        button {
                            class: "flex-1 bg-gradient-to-r from-red-700 to-red-500 rounded-2xl p-4 transition-all shadow-2xl",
                            onclick: move |_| {
                                BarcodeScanner::cancel();
                                navigator.go_back()
                            },
                            div {
                                class: "flex flex-col items-center gap-2",
                                i { class: "bi bi-stop-circle-fill text-white text-2xl" }
                                span { class: "text-white text-sm font-bold", "Скасувати" }
                            }
                        }

                        // button {
                        //     class: "flex-1 bg-black bg-opacity-60 backdrop-blur-lg border border-white border-opacity-20 rounded-2xl p-4 hover:bg-opacity-80 transition-all shadow-xl",
                        //     div {
                        //         class: "flex flex-col items-center gap-2",
                        //         i { class: "bi bi-image text-blue-400 text-2xl" }
                        //         span { class: "text-white text-sm font-medium", "Галерея" }
                        //     }
                        // }
                    }
                }
            }
        }
    }
}