#![allow(non_snake_case)]
#![allow(unused_braces)]

use dioxus::prelude::*;

#[derive(Props)]
pub struct TabContainerProps<'a> {
    class: Option<&'a str>,
    tabs: Element<'a>,
    children: Element<'a>,
}

pub fn TabContainer<'a>(cx: Scope<'a, TabContainerProps<'a>>) -> Element {
    let class = if let Some(class) = cx.props.class {
        class
    } else {
        ""
    };

    let class = format!("'Box {}'", class);

    cx.render(rsx!(
        {
            LazyNodes::new(|f| f.text(format_args!("<tab-container class={} data-view-component='true'>", class)))
        }
        div {
            class: "UnderlineNav",
            ul {
                role: "tablist", 
                "aria-label": "label",
                class: "tabnav-tabs",
                {&cx.props.tabs}
            }
        }
        {&cx.props.children}
        {
            LazyNodes::new(|f| f.text(format_args!("</tab-container>")))
        }
    ))
}

#[derive(Props)]
pub struct TabHeaderProps<'a> {
    selected: bool,
    tab: &'a str,
    name: &'a str,
}

pub fn TabHeader<'a>(cx: Scope<'a, TabHeaderProps<'a>>) -> Element {
    cx.render(rsx!(
        li {
            role: "presentation",
            class: "d-inline-flex",
            button {
                class: "UnderlineNav-item",
                "type": "button",
                role: "tab",
                "aria-controls": "{cx.props.tab}",
                "aria-selected": "{cx.props.selected}",
                "{cx.props.name}"
            }
        }
    ))
}

#[derive(Props)]
pub struct TabPanelProps<'a> {
    hidden: bool,
    id: &'a str,
    class: Option<&'a str>,
    children: Element<'a>,
}

pub fn TabPanel<'a>(cx: Scope<'a, TabPanelProps<'a>>) -> Element {
    let class = if let Some(class) = cx.props.class {
        class
    } else {
        ""
    };

    if cx.props.hidden {
        cx.render(rsx!(
            div {
                id: "{cx.props.id}",
                class: "{class}",
                hidden: "hidden",
                role: "tabpanel",
                &cx.props.children
            }
        ))
    } else {
        cx.render(rsx!(
            div {
                id: "{cx.props.id}",
                class: "{class}",
                role: "tabpanel",
                &cx.props.children
            }
        ))
    }
}
