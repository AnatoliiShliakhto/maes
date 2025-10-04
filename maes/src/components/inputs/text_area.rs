use crate::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct TextAreaProps {
    #[props(into, default = "")]
    pub class: String,
    #[props(into, default = "")]
    pub name: String,
    #[props(into, default = "")]
    pub placeholder: String,
    #[props(into, default = false)]
    pub required: bool,
    #[props(into, default = 0)]
    pub minlength: i32,
    #[props(into, default = 1000)]
    pub maxlength: i32,
    #[props(into, default = "")]
    pub initial_value: String,
    #[props(into, default = "")]
    pub label: String,
}

#[component]
pub fn TextArea(props: TextAreaProps) -> Element {

    rsx! {
        fieldset {
            class: "fieldset w-full",
            legend {
                class: "text-base-content/70 lowercase",
                "{props.label}"
            }
            textarea {
                class: format!("textarea w-full resize-none validator overflow-hidden {}", props.class),
                style: "field-sizing: content;",
                name: props.name,
                placeholder: props.placeholder,
                required: props.required,
                minlength: props.minlength,
                maxlength: props.maxlength,
                initial_value: props.initial_value,
            }
        }
    }
}