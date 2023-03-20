#![allow(non_snake_case)]
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq, Eq)]
pub struct Props {
    pub yes: bool,
}

pub fn YesOrNo(cx: Scope<Props>) -> Element {
    if cx.props.yes {
        cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Success,
                "Yes"
            }
        ))
    } else {
        cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Severe,
                label_contrast: LabelContrast::Primary,
                "No"
            }
        ))
    }
}
