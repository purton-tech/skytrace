#![allow(non_snake_case)]
use db::NegotiationAction;
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq, Eq)]
pub struct Props<'a> {
    pub negotiation_action: &'a NegotiationAction,
}

pub fn Action<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    match cx.props.negotiation_action {
        NegotiationAction::NegotiationTriggered => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Done,
                label_contrast: LabelContrast::Primary,
                "Negotiation Triggered"
            }
        )),
        NegotiationAction::TriggerDeltaNegotiation => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Done,
                label_contrast: LabelContrast::Primary,
                "Trigger Delta Negotiation"
            }
        )),
        NegotiationAction::ManoeuvreNotRequired => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Done,
                label_contrast: LabelContrast::Primary,
                "Manoeuvre Not Required"
            }
        )),
        NegotiationAction::ManoeuvreAssigned => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Done,
                label_contrast: LabelContrast::Primary,
                "Manoeuvre Assigned"
            }
        )),
        NegotiationAction::ManoeuvrePlanned => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Done,
                label_contrast: LabelContrast::Primary,
                "Manoeuvre Planned"
            }
        )),
        NegotiationAction::ManoeuvreScreened => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Done,
                label_contrast: LabelContrast::Primary,
                "Manoeuvre Screened"
            }
        )),
        NegotiationAction::ManoeuvreChange => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Done,
                label_contrast: LabelContrast::Primary,
                "Manoeuvre Change"
            }
        )),
        NegotiationAction::ManoeuvreSigned => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Done,
                label_contrast: LabelContrast::Primary,
                "Manoeuvre Signed"
            }
        )),
        NegotiationAction::ManoeuvreUplinked => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Done,
                label_contrast: LabelContrast::Primary,
                "Manoeuvre Uplinked"
            }
        )),
        NegotiationAction::ManoeuvreFailed => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Done,
                label_contrast: LabelContrast::Primary,
                "Manoeuvre Failed"
            }
        )),
        NegotiationAction::MessageSent => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Done,
                label_contrast: LabelContrast::Primary,
                "Message Sent"
            }
        )),
        NegotiationAction::NewCCSDSData => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Done,
                label_contrast: LabelContrast::Primary,
                "New CCSDS Data"
            }
        )),
    }
}
