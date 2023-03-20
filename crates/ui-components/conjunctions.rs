use super::conjunctions_filter::ConjunctionFilter;
use super::data_upload::DataUpload;
use crate::trace_layout::{SideBar, TraceLayout};
use assets::files::button_plus_svg;
use db::Conjunction;
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq)]
struct ConjunctionsProps {
    conjunctions: Vec<Conjunction>,
    upload_action: String,
    filter_action: String,
    next_page_url: Option<String>,
    organisation_id: i32,
}

pub fn conjunctions(
    conjunctions: Vec<Conjunction>,
    next_page_url: Option<String>,
    organisation_id: i32,
) -> String {
    fn app(cx: Scope<ConjunctionsProps>) -> Element {
        cx.render(rsx! {
            TraceLayout {
                selected_item: SideBar::Conjunctions,
                team_id: cx.props.organisation_id,
                title: "Conjunctions"
                header: cx.render(rsx!(
                    h3 { "Conjunctions" }
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
                        title: "Conjunctions",
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
                            drawer_trigger: "filter-conjunction-data",
                            "Filter"
                        }
                    }
                    BoxBody {
                        DataTable {
                            table {
                                thead {
                                    th { "ID" }
                                    th { "First Object" }
                                    th { "Secondary Object" }
                                    th { "Time of closest Approach" }
                                    th { "Miss Distance" }
                                    th { "Probability of Collision" }
                                    th {
                                        class: "text-right",
                                        "Action"
                                    }
                                }
                                tbody {
                                    cx.props.conjunctions.iter().map(|conjunction| rsx!(
                                        tr {
                                            td {
                                                "{conjunction.id}"
                                            }
                                            td {
                                                "{conjunction.first_object}"
                                            }
                                            td {
                                                "{conjunction.secondary_object}"
                                            }
                                            td {
                                                RelativeTime {
                                                    format: RelativeTimeFormat::Relative,
                                                    datetime: &conjunction.time_of_closest_approach
                                                }
                                            }
                                            td {
                                                Label {
                                                    label_color: LabelColor::Open,
                                                    "{conjunction.miss_distance}"
                                                }
                                            }
                                            td {
                                                if conjunction.collision_probability > 0.005 {
                                                    cx.render(rsx!(
                                                        Label {
                                                            label_color: LabelColor::Danger,
                                                            "{conjunction.collision_probability}"
                                                        }
                                                    ))
                                                } else {
                                                    cx.render(rsx!(
                                                        Label {
                                                            label_color: LabelColor::Open,
                                                            "{conjunction.collision_probability}"
                                                        }
                                                    ))
                                                }
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
    let filter_action = crate::routes::conjunctions::index_route(organisation_id);

    let mut app = VirtualDom::new_with_props(
        app,
        ConjunctionsProps {
            organisation_id,
            conjunctions,
            upload_action,
            filter_action,
            next_page_url,
        },
    );
    let _ = app.rebuild();
    dioxus::ssr::render_vdom(&app)
}
