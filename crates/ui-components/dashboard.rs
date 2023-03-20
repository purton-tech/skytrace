use crate::trace_layout::{SideBar, TraceLayout};
use assets::files::*;
use db::Metrics;
use dioxus::prelude::*;
use primer_rsx::{BlankSlate, Box, BoxBody, BoxHeader};

#[derive(Props, PartialEq)]
struct DashboardProps {
    organisation_id: i32,
    metrics: Metrics,
}

pub fn index(organisation_id: i32, metrics: Metrics) -> String {
    fn app(cx: Scope<DashboardProps>) -> Element {
        cx.render(rsx! {
            TraceLayout {
                selected_item: SideBar::Dashboard,
                team_id: cx.props.organisation_id,
                title: "Dashboard"
                header: cx.render(rsx!(
                    h3 { "Dashboard" }
                ))
                BlankSlate {
                    heading: "Dashboard",
                    visual: nav_dashboard_svg.name,
                    description: "This dashboard will only be viewable by system admins"
                }
                section {
                    class: "d-flex flex-justify-between gap-1",
                    Box {
                        class: "flex-1",
                        BoxHeader {
                            title: "Total Number of Users"
                        }
                        BoxBody {
                            "{cx.props.metrics.total_users}"
                        }
                    }
                    Box {
                        class: "flex-1",
                        BoxHeader {
                            title: "Registrations in the last 7 days"
                        }
                        BoxBody {
                            "{cx.props.metrics.registration_past_7_days}"
                        }
                    }
                }
            }
        })
    }

    let mut app = VirtualDom::new_with_props(
        app,
        DashboardProps {
            organisation_id,
            metrics,
        },
    );
    let _ = app.rebuild();
    dioxus::ssr::render_vdom(&app)
}
