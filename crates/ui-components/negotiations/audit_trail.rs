#![allow(non_snake_case)]
use db::TimeLine;
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq)]
pub struct AuditTrailProps {
    time_line: Vec<TimeLine>,
}

pub fn AuditTrail(cx: Scope<AuditTrailProps>) -> Element {
    cx.render(rsx!(
        DataTable {
            table {
                thead {
                    th { "Initiator" }
                    th { "Action" }
                    th {
                        "Resulting Status"
                    }
                    th {
                        class: "text-right",
                        "Date"
                    }
                }
                tbody {
                    cx.props.time_line.iter().map(|audit| rsx!(
                        tr {
                            td {
                                if let Some(initiator_email) = &audit.initiator_email {
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
                            }
                            td {
                                super::negotiation_action::Action {
                                    negotiation_action: &audit.action
                                }
                            }
                            td {
                                super::negotiation_status::Status {
                                    negotiation_status: &audit.resulting_status
                                }
                            }
                            td {
                                class: "text-right",
                                RelativeTime {
                                    format: RelativeTimeFormat::Relative,
                                    datetime: &audit.created_at
                                }
                            }
                        }
                    ))
                }
            }
        }
    ))
}
