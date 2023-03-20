#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props, Eq, PartialEq)]
pub struct CounterProps<'a> {
    count: u32,
    class: Option<&'a str>,
}

pub fn Counter<'a>(cx: Scope<'a, CounterProps<'a>>) -> Element {
    let class = if let Some(class) = cx.props.class {
        class
    } else {
        ""
    };

    cx.render(rsx!(
        span {
            class: "Counter {class}",
            "{cx.props.count}"
        }
    ))
}
