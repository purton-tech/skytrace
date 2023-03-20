#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub enum SelectSize {
    #[default]
    Default,
    Small,
    Large,
}

impl SelectSize {
    pub fn to_string(&self) -> &'static str {
        match self {
            SelectSize::Default => "",
            SelectSize::Small => "sm",
            SelectSize::Large => "large",
        }
    }
}

#[derive(Props)]
pub struct SelectProps<'a> {
    children: Element<'a>,
    select_size: Option<SelectSize>,
    pub name: &'a str,
    pub id: Option<&'a str>,
    pub value: Option<&'a str>,
    pub label: Option<&'a str>,
    pub help_text: Option<&'a str>,
    pub required: Option<bool>,
    pub disabled: Option<bool>,
    pub readonly: Option<bool>,
    pub label_class: Option<&'a str>,
}

pub fn Select<'a>(cx: Scope<'a, SelectProps<'a>>) -> Element {
    let select_size = if cx.props.select_size.is_some() {
        cx.props.select_size.unwrap()
    } else {
        Default::default()
    };

    let value = cx.props.value.unwrap_or("");

    let class = select_size.to_string();

    let id = if let Some(id) = cx.props.id { id } else { "" };

    let label_class = if let Some(label_class) = cx.props.label_class {
        label_class
    } else {
        ""
    };

    cx.render(rsx!(
        match cx.props.label {
            Some(l) => cx.render(rsx!(
                label {
                    class: "{label_class}",
                    "{l}"
                }
            )),
            None => None
        }
        select {
            id: "{id}",
            class: "form-select {class}",
            value: "{value}",
            name: "{cx.props.name}",
            &cx.props.children
        }
        match cx.props.help_text {
            Some(l) => cx.render(rsx!(
                span {
                    class: "note mb-3",
                    "{l}"
                }
            )),
            None => None
        }
    ))
}
