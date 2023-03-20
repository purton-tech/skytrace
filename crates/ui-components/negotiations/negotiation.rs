use super::audit_trail::AuditTrail;
use super::next_steps::NextSteps;
use super::time_line::Negotiation;
use crate::trace_layout::{SideBar, TraceLayout};
use db::{Negotiation, TimeLine};
use dioxus::prelude::*;
use primer_rsx::*;

struct NegotiationProps {
    send_message_action: String,
    change_status_action: String,
    organisation_id: i32,
    title: String,
    negotiation: Negotiation,
    time_line: Vec<TimeLine>,
}

pub fn negotiation(
    negotiation: Negotiation,
    time_line: Vec<TimeLine>,
    organisation_id: i32,
) -> String {
    fn app(cx: Scope<NegotiationProps>) -> Element {
        cx.render(rsx! {
            TraceLayout {
                selected_item: SideBar::Negotiations,
                team_id: cx.props.organisation_id,
                title: "Negotiation"
                header: cx.render(rsx!(
                    h3 { "{cx.props.title}" }
                    Button {
                        drawer_trigger: "next-steps-drawer",
                        button_scheme: ButtonScheme::Primary,
                        "Next Steps"
                    }
                ))
                TabContainer {
                    class: "height-full position-relative",
                    tabs: cx.render(rsx! {
                        TabHeader {
                            selected: true,
                            tab: "negotiation-panel",
                            name: "Negotiation"
                        }
                        TabHeader {
                            selected: false,
                            tab: "audit-panel"
                            name: "Audit Trail"
                        }
                    })
                    TabPanel {
                        hidden: false,
                        class: ""
                        id: "negotiation-panel",
                        Negotiation {
                            send_message_action: cx.props.send_message_action.clone(),
                            time_line: cx.props.time_line.clone()
                        }
                    }
                    TabPanel {
                        hidden: true,
                        id: "audit-panel",
                        AuditTrail {
                            time_line: cx.props.time_line.clone()
                        }
                    }
                }
                NextSteps {
                    negotiation: cx.props.negotiation.clone(),
                    change_status_action: cx.props.change_status_action.clone()
                }
            }
        })
    }

    let title = format!(
        "Negotiation Between {} and {}",
        negotiation.object1_name, negotiation.object2_name
    );

    let change_status_action =
        crate::routes::negotiations::change_status_route(organisation_id, negotiation.id);

    let send_message_action =
        crate::routes::negotiations::send_message_route(organisation_id, negotiation.id);

    let mut app = VirtualDom::new_with_props(
        app,
        NegotiationProps {
            send_message_action,
            change_status_action,
            organisation_id,
            title,
            negotiation,
            time_line,
        },
    );
    let _ = app.rebuild();
    dioxus::ssr::render_vdom(&app)
}
