use crate::trace_layout::{SideBar, TraceLayout};
use db::RegisteredObject;
use dioxus::prelude::*;
use primer_rsx::*;

struct SpaceObjectsProps {
    organisation_id: i32,
    space_objects: Vec<RegisteredObject>,
}

pub fn space_objects(space_objects: Vec<RegisteredObject>, organisation_id: i32) -> String {
    fn app(cx: Scope<SpaceObjectsProps>) -> Element {
        if cx.props.space_objects.is_empty() {
            cx.render(rsx! {
                super::empty::EmptySpaceObjects {
                    organisation_id: cx.props.organisation_id
                }
                super::form::SpaceObjectForm{
                    organisation_id: cx.props.organisation_id
                }
            })
        } else {
            cx.render(rsx! {
                TraceLayout {
                    selected_item: SideBar::SpaceObjects,
                    team_id: cx.props.organisation_id,
                    title: "Space Objects"
                    header: cx.render(rsx!(
                        h3 { "Space Objects" }
                        Button {
                            drawer_trigger: "add-space-object",
                            button_scheme: ButtonScheme::Primary,
                            "Add Space Object"
                        }
                    ))
                    Box {
                        BoxHeader {
                            title: "Space Objects"
                        }
                        BoxBody {
                            DataTable {
                                table {
                                    thead {
                                        th { "Name" }
                                        th { "Designator" }
                                        th { "Status" }
                                        th { "Manoeuvrable?" }
                                        th { "Method" }
                                        th { "Data Sharing Policy" }
                                        th {
                                            class: "text-right",
                                            "Action"
                                        }
                                    }
                                    tbody {
                                        cx.props.space_objects.iter().map(|so| rsx!(
                                            tr {
                                                td {
                                                    strong {
                                                        "{so.name}"
                                                    }
                                                }
                                                td {
                                                    "{so.object_identifier}"
                                                }
                                                td {
                                                    super::validation_status::Status {
                                                        status: &so.validation_status
                                                    }
                                                }
                                                td {
                                                    super::yes_or_no::YesOrNo{
                                                        yes: so.manoeuvrable
                                                    }
                                                }
                                                td {
                                                    "{so.manoeuvre_strategy}"
                                                }
                                                td {
                                                    "{so.confidentiality}"
                                                }
                                                td {
                                                    class: "text-right",
                                                    SelectMenu {
                                                        alignment: SelectMenuAlignment::Right,
                                                        summary: cx.render(rsx!(
                                                            summary {
                                                                class: "btn",
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
                super::form::SpaceObjectForm{
                    organisation_id: cx.props.organisation_id
                }
            })
        }
    }

    let mut app = VirtualDom::new_with_props(
        app,
        SpaceObjectsProps {
            organisation_id,
            space_objects,
        },
    );
    let _ = app.rebuild();
    dioxus::ssr::render_vdom(&app)
}
