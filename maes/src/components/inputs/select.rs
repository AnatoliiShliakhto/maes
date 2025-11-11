use crate::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct SelectProps {
    #[props(into, default = "")]
    pub name: String,
    #[props(into, default = false)]
    pub required: bool,
    #[props(into, default = rsx!())]
    pub label: Element,
    #[props(into, default = rsx!())]
    pub children: Element,
}

#[component]
pub fn Select(props: SelectProps) -> Element {
    rsx! {
        label {
            class: "floating-label mt-5 lowercase",
            { props.label }
            select {
                class: "select validator w-full",
                name: props.name,
                required: props.required,
                { props.children }
            }
        }
    }
}
