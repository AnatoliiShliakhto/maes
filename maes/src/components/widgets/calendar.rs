use crate::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct CalendarProps {
    #[props(into, default = "".to_string())]
    pub class: String,
    #[props(into, default = "".to_string())]
    pub placeholder: String,
    pub onchange: Callback<FormEvent>,
}

#[component]
pub fn Calendar(props: CalendarProps) -> Element {
    let id = safe_nanoid!();
    let mut value = use_signal(String::new);

    rsx! {
        div {
            button {
                id: "cally-button-{id}",
                class: format!("input input-border cursor-pointer {props} {class}", props= props.class,
                    class = if value.read().is_empty() { "text-base-content/50" } else { "" }
                ),
                style: "anchor-name:--cally-{id}",
                popovertarget: "cally-{id}-popover",
                if value.read().is_empty() {
                    "{props.placeholder}"
                } else {
                    "{value}"
                }
            }
            div {
                popover: "auto",
                id: "cally-{id}-popover",
                class: "dropdown bg-base-100 rounded-box shadow-lg",
                style: "position-anchor:--cally-{id}",
                input {
                    id: "input-date-{id}",
                    r#type: "hidden",
                    onchange: move |evt| {
                        value.set(evt.value());
                        props.onchange.call(evt);
                        // document::eval(r#"document.getElementById('cally-{id}-popover')?.hidePopover?.()"#);
                    },
                }
                calendar-date {
                    id: "cally-{id}",
                    class: "cally",
                    locale: "uk-UA",
                    "onchange": r#"
                        var el = document.getElementById('input-date-{id}');
                        if(!el) return;
                        el.value = this.value;
                        el.dispatchEvent(new Event('change', {{ bubbles: true }}));
                    "#,
                    i { class: "bi bi-caret-left-fill", slot: "previous" }
                    i { class: "bi bi-caret-right-fill", slot: "next" }
                    calendar-month {}
                    div {
                        class: "flex justify-end",
                        button {
                            class: "btn btn-sm btn-ghost lowercase",
                            "onclick": r#"
                                var el = document.getElementById('cally-{id}');
                                if(!el) return;
                                el.value = '';
                                el.removeAttribute && el.removeAttribute('value');
                                el.dispatchEvent(new Event('input', {{ bubbles: true }}));
                                el.dispatchEvent(new Event('change', {{ bubbles: true }}));
                            "#,
                            i { class: "bi bi-eraser-fill" }
                            // { t!("clear") }
                        }
                    }
                }
            }
        }
    }
}

pub fn clear_calendars() {
    document::eval(
        r#"
            document.querySelectorAll('.cally').forEach(el => {
                el.value = '';
                el.removeAttribute && el.removeAttribute('value');
                el.dispatchEvent(new Event('input', { bubbles: true }));
                el.dispatchEvent(new Event('change', { bubbles: true }));
            });
        "#    
    );
}