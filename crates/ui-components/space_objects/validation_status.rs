#![allow(non_snake_case)]
use db::ValidationStatus;
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq, Eq)]
pub struct Props<'a> {
    pub status: &'a ValidationStatus,
}

pub fn Status<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    match cx.props.status {
        ValidationStatus::Pending => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Severe,
                label_contrast: LabelContrast::Primary,
                "Pending Verification"
            }
        )),
        ValidationStatus::Verified => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Success,
                "Verified"
            }
        )),
    }
}
