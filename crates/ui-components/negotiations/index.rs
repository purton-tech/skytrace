use crate::trace_layout::{SideBar, TraceLayout};
use assets::files::handshake_svg;
use db::Negotiation;
use dioxus::prelude::*;
use primer_rsx::*;

struct NegotiationProps {
    organisation_id: i32,
    negotiations: Vec<Negotiation>,
}

pub fn negotiations(negotiations: Vec<Negotiation>, organisation_id: i32) -> String {
    fn app(cx: Scope<NegotiationProps>) -> Element {
        if cx.props.negotiations.is_empty() {
            cx.render(rsx! {
                TraceLayout {
                    selected_item: SideBar::Negotiations,
                    team_id: cx.props.organisation_id,
                    title: "Negotiations"
                    header: cx.render(rsx!(
                        h3 { "Negotiations" }
                    ))
                    BlankSlate {
                        heading: "We don't have any negotiations related to your objects yet",
                        visual: handshake_svg.name,
                        description: "Add some more objects or wait until we notify you of any events",
                    }
                }
            })
        } else {
            cx.render(rsx! {
                TraceLayout {
                    selected_item: SideBar::Negotiations,
                    team_id: cx.props.organisation_id,
                    title: "Negotiations"
                    header: cx.render(rsx!(
                        h3 { "Negotiations" }
                    ))
                    Box {
                        BoxHeader {
                            title: "Negotiations"
                        }
                        BoxBody {
                            DataTable {
                                table {
                                    thead {
                                        th { "Event ID" }
                                        th { "First Object" }
                                        th { "Second Object" }
                                        th { "Negotiation Status" }
                                        th {
                                            class: "text-right",
                                            "Action"
                                        }
                                    }
                                    tbody {
                                        cx.props.negotiations.iter().map(|negotiation| rsx!(
                                            tr {
                                                td {
                                                    a {
                                                        href: "negotiation/{negotiation.id}",
                                                        "{negotiation.id}",
                                                    }
                                                }
                                                td {
                                                    "{negotiation.object1_name}"
                                                }
                                                td {
                                                    "{negotiation.object2_name}"
                                                }
                                                td {
                                                    super::negotiation_status::Status {
                                                        negotiation_status: &negotiation.status
                                                    }
                                                }
                                                td {
                                                    class: "text-right",
                                                    DropDown {
                                                        direction: Direction::SouthWest,
                                                        button_text: "...",
                                                        DropDownLink {
                                                            href: crate::routes::negotiations::negotiation_route(
                                                                cx.props.organisation_id,
                                                                negotiation.id),
                                                            "View Negotiation"
                                                        }
                                                    }
                                                }
                                            }
                                        ))
                                    }
                                }
                            }
                        }
                    }
                }
            })
        }
    }

    let mut app = VirtualDom::new_with_props(
        app,
        NegotiationProps {
            organisation_id,
            negotiations,
        },
    );
    let _ = app.rebuild();
    dioxus::ssr::render_vdom(&app)
}
