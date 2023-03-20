#![allow(non_snake_case)]
#![allow(unused_braces)]

use dioxus::prelude::*;

#[derive(Props)]
pub struct DataTableProps<'a> {
    children: Element<'a>,
}

pub fn DataTable<'a>(cx: Scope<'a, DataTableProps<'a>>) -> Element {
    cx.render(rsx!(
        {
            LazyNodes::new(|f| f.text(format_args!("<data-table density='condensed'>")))
        }
        {&cx.props.children}
        {
            LazyNodes::new(|f| f.text(format_args!("</data-table>")))
        }
    ))
}
