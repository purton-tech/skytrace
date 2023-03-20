#![allow(non_snake_case)]
use crate::trace_layout::{SideBar, TraceLayout};
use assets::files::empty_satellite_svg;
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq, Eq)]
pub struct EmptyProps {
    organisation_id: i32,
}

pub fn EmptySpaceObjects(cx: Scope<EmptyProps>) -> Element {
    cx.render(rsx!(
        TraceLayout {
            selected_item: SideBar::SpaceObjects,
            team_id: cx.props.organisation_id,
            title: "Space Objects"
            header: cx.render(rsx!(
                h3 { "Space Objects" }
            ))
            BlankSlate {
                heading: "Looks like you haven't registered any of your space objects",
                visual: empty_satellite_svg.name,
                description: "When you register your space objects we can then notify you of any high interest events.",
                primary_action_drawer: ("Add Space Object", "add-space-object")
            }
        }
    ))
}
