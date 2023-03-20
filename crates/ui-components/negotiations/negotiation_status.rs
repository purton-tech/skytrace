#![allow(non_snake_case)]
use db::NegotiationStatus;
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq, Eq)]
pub struct Props<'a> {
    pub negotiation_status: &'a NegotiationStatus,
}

pub fn Status<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    match cx.props.negotiation_status {
        NegotiationStatus::ManoeuvreExecuted => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Done,
                label_contrast: LabelContrast::Primary,
                "Manoeuvre Executed"
            }
        )),
        NegotiationStatus::ManoeuvreFailed => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Accent,
                "Manoeuvre Failed"
            }
        )),
        NegotiationStatus::ManoeuvreNegotiate => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Accent,
                "Manoeuvre Negotiate"
            }
        )),
        NegotiationStatus::ManoeuvreNotExecuted => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Accent,
                "Manoeuvre Not Executed"
            }
        )),
        NegotiationStatus::ManoeuvreNotRequired => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Accent,
                "Manoeuvre Not Required"
            }
        )),
        NegotiationStatus::ManoeuvrePlanned => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Accent,
                "Manoeuvre Planned"
            }
        )),
        NegotiationStatus::ManoeuvreRequired => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Accent,
                "Manoeuvre Required"
            }
        )),
        NegotiationStatus::ManoeuvreScreened => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Accent,
                "Manoeuvre Screened"
            }
        )),
        NegotiationStatus::ManoeuvreSigned => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Accent,
                "Manoeuvre Signed"
            }
        )),
        NegotiationStatus::ManoeuvreUplinked => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Accent,
                "Manoeuvre Uplinked"
            }
        )),
    }
}
