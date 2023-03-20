#![allow(non_snake_case)]
use db::{Negotiation, NegotiationStatus};
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq)]
pub struct NextStepsProps {
    negotiation: Negotiation,
    change_status_action: String,
}

pub fn NextSteps(cx: Scope<NextStepsProps>) -> Element {
    cx.render(rsx!(
        Drawer {
            label: "Next Steps",
            trigger_id: "next-steps-drawer",
            DrawerBody {
                match cx.props.negotiation.status {
                    NegotiationStatus::ManoeuvreNegotiate => {
                        cx.render(rsx!(
                            Alert {
                                h4 {
                                    "We will Perform a Manoeuvre"
                                }
                                p {
                                    "If you are happy to perform the manoeuvre then select this option and
                                    the manoeuvre will be assigned to your team."
                                }
                                form {
                                    method: "post",
                                    action: "{cx.props.change_status_action}",
                                    input {
                                        "type": "hidden",
                                        name: "action",
                                        value: "ManoeuvreAssigned"
                                    }
                                    Button {
                                        button_type: ButtonType::Submit,
                                        button_scheme: ButtonScheme::Default,
                                        "Yes, We will perform the manoeuvre."
                                    }
                                }
                            }
                            Alert {
                                class: "mt-4",
                                h4 {
                                    "A Manoeuvre is not required"
                                }
                                p {
                                    "The outcome of negotiation has determined that no avoidance manoeuvre is required."
                                }
                                form {
                                    method: "post",
                                    action: "{cx.props.change_status_action}",
                                    input {
                                        "type": "hidden",
                                        name: "action",
                                        value: "ManoeuvreNotRequired"
                                    }
                                    Button {
                                        button_type: ButtonType::Submit,
                                        button_scheme: ButtonScheme::Default,
                                        "A Manoeuvre is not required."
                                    }
                                }
                            }
                        ))
                    }
                    NegotiationStatus::ManoeuvreRequired => {
                        cx.render(rsx!(
                            Alert {
                                h4 {
                                    "The Manoeuvre has been planned"
                                }
                                p {
                                    "We have planned our manoeuvre."
                                }
                                form {
                                    method: "post",
                                    action: "{cx.props.change_status_action}",
                                    input {
                                        "type": "hidden",
                                        name: "action",
                                        value: "ManoeuvrePlanned"
                                    }
                                    Button {
                                        button_type: ButtonType::Submit,
                                        button_scheme: ButtonScheme::Default,
                                        "Yes, The manoeuvre is planned."
                                    }
                                }
                            }
                        ))
                    }
                    NegotiationStatus::ManoeuvrePlanned => {
                        cx.render(rsx!(
                            Alert {
                                h4 {
                                    "The Manoeuvre has been uplinked"
                                }
                                p {
                                    "Indicates that the manoeuvre has been uplinked for execution."
                                }
                                form {
                                    method: "post",
                                    action: "{cx.props.change_status_action}",
                                    input {
                                        "type": "hidden",
                                        name: "action",
                                        value: "ManoeuvreUplinked"
                                    }
                                    Button {
                                        button_type: ButtonType::Submit,
                                        button_scheme: ButtonScheme::Default,
                                        "Yes, The Manoeuvre has been uplinked."
                                    }
                                }
                            }
                            Alert {
                                class: "mt-4",
                                h4 {
                                    "The manoeuvre required re-planning"
                                }
                                p {
                                    "Indicates that the manoeuvre required re-planning. Transitions back to
                                    `ManoeuvreRequired` without changing the manoeuvre responsibility."
                                }
                                form {
                                    method: "post",
                                    action: "{cx.props.change_status_action}",
                                    input {
                                        "type": "hidden",
                                        name: "action",
                                        value: "ManoeuvreChange"
                                    }
                                    Button {
                                        button_type: ButtonType::Submit,
                                        button_scheme: ButtonScheme::Default,
                                        "The manoeuvre required re-planning."
                                    }
                                }
                            }
                        ))
                    }
                    NegotiationStatus::ManoeuvreUplinked => {
                        cx.render(rsx!(
                            Alert {
                                h4 {
                                    "The manoeuvre has been screened"
                                }
                                p {
                                    "Indicate that post-trajectory screening of the manoeuvre plans has been performed."
                                }
                                form {
                                    method: "post",
                                    action: "{cx.props.change_status_action}",
                                    input {
                                        "type": "hidden",
                                        name: "action",
                                        value: "ManoeuvreScreened"
                                    }
                                    Button {
                                        button_type: ButtonType::Submit,
                                        button_scheme: ButtonScheme::Default,
                                        "Yes, The manoeuvre has been screened."
                                    }
                                }
                            }
                        ))
                    }
                    NegotiationStatus::ManoeuvreScreened => {
                        cx.render(rsx!(
                            Alert {
                                h4 {
                                    "The manoeuvre has been signed"
                                }
                                p {
                                    "Indicates that the manoeuvre has been approved by the O/O (with or
                                        without trajectory screening by a third party)."
                                }
                                form {
                                    method: "post",
                                    action: "{cx.props.change_status_action}",
                                    input {
                                        "type": "hidden",
                                        name: "action",
                                        value: "ManoeuvreSigned"
                                    }
                                    Button {
                                        button_type: ButtonType::Submit,
                                        button_scheme: ButtonScheme::Default,
                                        "Yes, The manoeuvre has been signed."
                                    }
                                }
                            }
                        ))
                    }
                    NegotiationStatus::ManoeuvreSigned => {
                        cx.render(rsx!(
                            Alert {
                                h4 {
                                    "The manoeuvre has been executed"
                                }
                                p {
                                    "TCA has been reached and manoeuvre has been executed (final state)."
                                }
                                form {
                                    method: "post",
                                    action: "{cx.props.change_status_action}",
                                    input {
                                        "type": "hidden",
                                        name: "action",
                                        value: "ManoeuvreExecuted"
                                    }
                                    Button {
                                        button_type: ButtonType::Submit,
                                        button_scheme: ButtonScheme::Default,
                                        "Yes, The manoeuvre has been executed."
                                    }
                                }
                            }
                            Alert {
                                class: "mt-4",
                                h4 {
                                    "The manoeuvre has failed"
                                }
                                p {
                                    "TCA has been reached and manoeuvre has failed (final state)."
                                }
                                form {
                                    method: "post",
                                    action: "{cx.props.change_status_action}",
                                    input {
                                        "type": "hidden",
                                        name: "action",
                                        value: "ManoeuvreFailed"
                                    }
                                    Button {
                                        button_type: ButtonType::Submit,
                                        button_scheme: ButtonScheme::Default,
                                        "Yes, The manoeuvre has failed."
                                    }
                                }
                            }
                        ))
                    }
                    NegotiationStatus::ManoeuvreNotRequired => {
                        cx.render(rsx!(
                            div {
                            }
                        ))
                    }
                    // End States
                    NegotiationStatus::ManoeuvreFailed => {
                        cx.render(rsx!(
                            div {
                            }
                        ))
                    }
                    NegotiationStatus::ManoeuvreNotExecuted => {
                        cx.render(rsx!(
                            div {
                            }
                        ))
                    }
                    NegotiationStatus::ManoeuvreExecuted => {
                        cx.render(rsx!(
                            div {
                            }
                        ))
                    }
                }
            }
            DrawerFooter {
            }
        }
    ))
}
