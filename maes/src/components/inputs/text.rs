use crate::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct TextInputProps {
    #[props(into, default = "text")]
    pub r#type: String,
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
    #[props(into, default = 100)]
    pub maxlength: i32,
    #[props(into, default = 0)]
    pub min: i32,
    #[props(into, default = 255)]
    pub max: i32,
    #[props(into, default = 1)]
    pub step: i32,
    #[props(into, default = "")]
    pub initial_value: String,
    #[props(into, default = rsx!())]
    pub label: Element,
}

#[component]
pub fn TextInputComponent(props: TextInputProps) -> Element {
    rsx! {
        label {
            class: format!("floating-label lowercase {class}", class = props.class),
            { props.label }
            input {
                class: "input validator w-full",
                r#type: props.r#type,
                name: props.name,
                placeholder: "{props.placeholder.to_lowercase()}",
                required: props.required,
                minlength: props.minlength,
                maxlength: props.maxlength,
                min: props.min,
                max: props.max,
                step: props.step,
                initial_value: props.initial_value,
            }
        }
    }
}
