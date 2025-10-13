use crate::{prelude::*, services::*};

#[component]
pub fn WiFiInstruction() -> Element {
    let config = ConfigService::read();
    let wifi_payload = format!(
        "WIFI:T:WPA;S:{};P:{};H:false;;",
        config.wifi.ssid, config.wifi.password
    );

    rsx! {
        div {
            class: "flex shrink-0 w-full min-h-0 print:hidden p-1",
            ul {
                class: "menu menu-horizontal p-0 m-0 text-base-content flex-nowrap",
                li {
                    button {
                        class: "hover:text-info",
                        onclick: move |event: MouseEvent| {
                            event.prevent_default();
                            event.stop_propagation();
                            document::eval("window.print()");
                        },
                        i { class: "bi bi-printer" }
                        { t!("print") }
                    }
                }
            }
        }
        div {
            class: "flex-scrollable p-4 gap-1 print-area",
            "data-theme": "lofi",
            div {
                class: "w-full text-center font-semibold text-xl",
                { t!("wifi-instruction") }
            }
            div {
                class: "flex w-full items-center justify-end gap-10 p-5",
                div {
                    class: "flex flex-col h-full justify-center gap-2 text-xl",
                    div {
                        class: "flex items-center gap-2 font-bold",
                        i { class: "bi bi bi-wifi text-base-content/40", }
                        "{config.wifi.ssid}"
                    }
                    div {
                        class: "flex items-center gap-2 font-bold",
                        i { class: "bi bi bi-key text-base-content/40", }
                        "{config.wifi.password}"
                    }
                }
                div {
                    class: "flex h-full max-w-30",
                    img {
                        class: "max-w-full h-auto object-contain overflow-hidden rounded-(--radius-box) ring-1 ring-base-300",
                        src: QrGenerator::text(wifi_payload, 300)
                    }
                }
            }
            div { class: "flex flex-1 flex-col p-4",
                h2 { class: "text-xl font-bold mb-3 border-b border-base-content/20 pb-1",
                    "Підключення до мережі Wi-Fi"
                }
                ol { class: "list-decimal **list-inside** space-y-3 pl-4 pb-4",
                    li {
                        "Відскануйте "
                        b { class: "font-semibold", "QR-код" }
                        " за допомогою "
                        b { class: "font-semibold", "Камери" }
                        " чи "
                        b { class: "font-semibold", "Додатка для сканування" }
                        " (наприклад, "
                        b { class: "font-semibold", "QRScanner" }
                        ", "
                        b { class: "font-semibold", "Viber" }
                        ")."
                    }
                    li {
                        "Натисніть "
                        b { class: "font-semibold", "\"Приєднатися до мережі\"" }
                        "."
                    }
                    li { class: "text-sm italic",
                        "Якщо на пристрої немає додатків із можливістю підключення через QR-код, ви можете приєднатися до мережі, використовуючи надані логін та пароль."
                    }
                }
                h2 { class: "text-xl font-bold mb-3 border-b border-base-content/50 pb-1 pt-2",
                    "Налаштування Пристроїв (Смартфони, Планшети)"
                }
                p { class: "pl-2 pb-3 text-sm",
                    "Щоб пристрої не відключалися автоматично, коли виявляють "
                    b { class: "font-semibold", "\"Wi-Fi без Інтернету\"" }
                    ", необхідно змінити деякі системні налаштування."
                }
                div { class: "font-semibold text-base pl-2 py-1",
                    "Для Android"
                }
                ol { class: "list-decimal **list-inside** space-y-3 pl-4 pb-4",
                    li {
                        b { class: "font-semibold",
                            "Тимчасово вимкніть мобільні дані"
                        }
                        " (Опціонально): Це гарантує, що телефон не переключиться на інтернет-провайдера."
                    }
                    li {
                        b { class: "font-semibold",
                            "Налаштування Wi-Fi (Для Стабільності):"
                        }
                        ol { class: "list-disc **list-inside** space-y-2 pl-6 pt-1 text-sm",
                            li {
                                "Перейдіть до "
                                b { class: "font-semibold", "Налаштування Wi-Fi" }
                                "."
                            }
                            li {
                                "Зайдіть у "
                                b { class: "font-semibold", "Додаткові налаштування" }
                                " або "
                                b { class: "font-semibold", "Параметри мережі" }
                                "."
                            }
                            li {
                                "Знайдіть та "
                                b { class: "font-semibold", "вимкніть" }
                                " опції на кшталт "
                                b { class: "font-semibold",
                                    "\"Інтелектуальне перемикання мереж\""
                                }
                                ", "
                                b { class: "font-semibold",
                                    "\"Автоматично перемикати на мобільну мережу\""
                                }
                                ", "
                                b { class: "font-semibold",
                                    "\"Виявляти непрацюючу мережу\""
                                }
                                " або "
                                b { class: "font-semibold", "\"Асистент Wi-Fi\"" }
                                "."
                            }
                        }
                    }
                }
                div { class: "font-semibold text-base pl-2 py-1",
                    "Для iOS (iPhone/iPad)"
                }
                ol { class: "list-decimal **list-inside** space-y-3 pl-4 pb-4",
                    li {
                        b { class: "font-semibold", "Вимкніть Мобільну Мережу" }
                        " (Опціонально): Тимчасово вимкніть "
                        b { class: "font-semibold", "Стільникові дані" }
                        " через "
                        b { class: "font-semibold", "Налаштування" }
                        " або "
                        b { class: "font-semibold", "Пункт керування" }
                        "."
                    }
                    li {
                        b { class: "font-semibold", "Вимкніть Wi-Fi Assist (Допомога Wi-Fi):" }
                        ol { class: "list-disc **list-inside** space-y-2 pl-6 pt-1 text-sm",
                            li {
                                "Перейдіть до "
                                b { class: "font-semibold",
                                    "Налаштування → Стільникові дані"
                                }
                                " (або "
                                b { class: "font-semibold", "Мобільні дані" }
                                ")."
                            }
                            li { "Прокрутіть униз до кінця." }
                            li {
                                "Знайдіть "
                                b { class: "font-semibold", "\"Допомога Wi-Fi\"" }
                                " (або "
                                b { class: "font-semibold", "\"Wi-Fi Assist\"" }
                                ") і "
                                b { class: "font-semibold", "вимкніть" }
                                " цю функцію."
                            }
                        }
                    }
                }
            }
        }
    }
}