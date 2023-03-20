#![allow(non_snake_case)]
use assets::files::{commit_svg, handshake_svg, smiley_svg};
use db::{NegotiationAction, NegotiationStatus, TimeLine};
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq)]
pub struct NegotiationProps {
    send_message_action: String,
    time_line: Vec<TimeLine>,
}

pub fn Negotiation(cx: Scope<NegotiationProps>) -> Element {
    cx.render(rsx!(
        div {
            id: "negotiation-stream",
            class: "d-flex flex-column-reverse",
            cx.props.time_line.iter().rev().map(|event| {
                match (event.resulting_status, event.action) {
                    (NegotiationStatus::ManoeuvreNegotiate, NegotiationAction::NegotiationTriggered) => {
                        cx.render(rsx!(
                            TimeLine {
                                condensed: true,
                                TimeLineBadge {
                                    image_src: commit_svg.name
                                }
                                TimeLineBody {
                                    "{event.detail}"
                                }
                            }
                            TimeLine {
                                TimeLineBadge {
                                    class: "color-bg-success-emphasis color-fg-on-emphasis",
                                    image_src: smiley_svg.name
                                }
                                TimeLineBody {
                                    span {
                                        strong {
                                            "{event.subject} "
                                        }
                                        "("
                                        RelativeTime {
                                            format: RelativeTimeFormat::Relative,
                                            datetime: &event.created_at
                                        }
                                        ")"
                                    }
                                }
                            }
                        ))
                    },
                    (_, NegotiationAction::MessageSent) => cx.render(rsx!(
                        TimeLine {
                            condensed: true,
                            TimeLineBadge {
                                image_src: commit_svg.name
                            }
                            TimeLineBody {
                                "Sent by "
                                if let Some(initiator_email) = &event.initiator_email {
                                    cx.render(rsx!(
                                        strong {
                                            "{initiator_email}"
                                        }
                                    ))
                                } else {
                                    cx.render(rsx!(
                                        strong {
                                            "SkyTrace"
                                        }
                                    ))
                                }
                                " "
                                RelativeTime {
                                    format: RelativeTimeFormat::Relative,
                                    datetime: &event.created_at
                                }
                            }
                        }
                        TimeLine {
                            TimeLineBadge {
                                image_src: handshake_svg.name
                            }
                            TimeLineBody {
                                Alert {
                                    alert_color: AlertColor::Warn,
                                    "{event.detail}"
                                }
                            }
                        }
                    )),
                    (_, _) => cx.render(rsx!(
                        TimeLine {
                            condensed: true,
                            TimeLineBadge {
                                image_src: commit_svg.name
                            }
                            TimeLineBody {
                                "Status changed to "
                                super::negotiation_status::Status {
                                    negotiation_status: &event.resulting_status
                                }
                                " "
                                RelativeTime {
                                    format: RelativeTimeFormat::Relative,
                                    datetime: &event.created_at
                                }
                            }
                        }
                        TimeLine {
                            TimeLineBadge {
                                image_src: handshake_svg.name
                            }
                            TimeLineBody {
                                span {
                                    if let Some(initiator_email) = &event.initiator_email {
                                        cx.render(rsx!(
                                            strong {
                                                "{initiator_email}"
                                            }
                                        ))
                                    } else {
                                        cx.render(rsx!(
                                            strong {
                                                "SkyTrace"
                                            }
                                        ))
                                    }
                                    " performed the following action "
                                    super::negotiation_action::Action {
                                        negotiation_action: &event.action
                                    }
                                }
                            }
                        }
                    )),
                }
            })
        }
        div {
            class: "position-absolute bottom-0 p-2 width-full border-top color-bg-subtle rounded-bottom-2",
            form {
                class: "width-full d-flex flex-justify-between flex-items-center",
                method: "post",
                action: "{cx.props.send_message_action}",
                textarea {
                    class: "flex-1 mr-2 form-control",
                    placeholder: "All communications are end to end encrypted.",
                    rows: "2",
                    name: "message"
                }
                Button {
                    button_type: ButtonType::Submit,
                    button_scheme: ButtonScheme::Outline,
                    "Send Message"
                }
            }
        }
    ))
}
