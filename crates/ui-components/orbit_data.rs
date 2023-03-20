use super::conjunctions_filter::ConjunctionFilter;
use super::data_upload::DataUpload;
use crate::trace_layout::{SideBar, TraceLayout};
use assets::files::button_plus_svg;
use db::OrbitData;
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq)]
struct OrbitDataProps {
    organisation_id: i32,
    orbit_data: Vec<OrbitData>,
    upload_action: String,
    filter_action: String,
    next_page_url: Option<String>,
}

pub fn orbit_data(
    orbit_data: Vec<OrbitData>,
    next_page_url: Option<String>,
    organisation_id: i32,
) -> String {
    fn app(cx: Scope<OrbitDataProps>) -> Element {
        cx.render(rsx! {
            TraceLayout {
                selected_item: SideBar::OrbitData,
                team_id: cx.props.organisation_id,
                title: "Orbit Data"
                header: cx.render(rsx!(
                    h3 { "Orbit Data" }
                    Button {
                        prefix_image_src: "{button_plus_svg.name}",
                        drawer_trigger: "upload-ccsds-data",
                        button_scheme: ButtonScheme::Primary,
                        "Upload CCSDS Data"
                    }
                ))
                Box {
                    BoxHeader {
                        class: "flex-justify-between",
                        title: "Orbit Data",
                        if let Some(url) = &cx.props.next_page_url {
                            cx.render(rsx! {
                                Pagination {
                                    next_page_url: url
                                }
                            })
                        } else {
                            cx.render(rsx! {
                                Pagination {
                                }
                            })
                        }
                        Button {
                            drawer_trigger: "filter-orbit-data",
                            "Filter"
                        }
                    }
                    BoxBody {
                        DataTable {
                            table {
                                thead {
                                    th { "Type" }
                                    th { "Version" }
                                    th { "Creation Date" }
                                    th { "Object Name" }
                                    th { "Probability of Collision" }
                                    th {
                                        class: "text-right",
                                        "Object Id"
                                    }
                                }
                                tbody {
                                    cx.props.orbit_data.iter().map(|orbit_data| rsx!(
                                        tr {
                                            td {
                                                "Test"
                                            }
                                            td {
                                                "Test"
                                            }
                                            td {
                                                "{orbit_data.created_at}"
                                            }
                                            td {
                                                "{orbit_data.originator}"
                                            }
                                            td {
                                                "{orbit_data.object_name}"
                                            }
                                            td {
                                                "{orbit_data.object_id}"
                                            }
                                            td {
                                                class: "text-right",
                                                SelectMenu {
                                                    alignment: SelectMenuAlignment::Right,
                                                    summary: cx.render(rsx!(
                                                        summary {
                                                            class: "btn btn-sm",
                                                            "aria-haspopup": "true",
                                                            "..."
                                                        }
                                                    ))
                                                    SelectMenuModal {
                                                        SelectMenuList {
                                                            button {
                                                                class: "SelectMenu-item",
                                                                role: "menuitemcheckbox",
                                                                "Not Implemented"
                                                            }
                                                        }
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
            DataUpload {
                upload_action: &cx.props.upload_action
            }
            ConjunctionFilter {
                filter_action: &cx.props.filter_action
            }
        })
    }

    let upload_action = crate::routes::data_upload::upload_route(organisation_id);
    let filter_action = crate::routes::orbit_data::index_route(organisation_id);

    let mut app = VirtualDom::new_with_props(
        app,
        OrbitDataProps {
            organisation_id,
            orbit_data,
            upload_action,
            filter_action,
            next_page_url,
        },
    );
    let _ = app.rebuild();
    dioxus::ssr::render_vdom(&app)
}
