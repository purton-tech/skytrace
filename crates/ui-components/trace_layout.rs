#![allow(non_snake_case)]
use super::logout_form::LogoutForm;
use assets::files::*;
use dioxus::prelude::*;
use primer_rsx::{AppLayout, NavGroup, NavItem};

#[derive(PartialEq, Eq, Debug)]
pub enum SideBar {
    ApiKeys,
    Dashboard,
    Users,
    SpaceObjects,
    Conjunctions,
    Negotiations,
    PhoneBook,
    ServiceRequests,
    CryptoKeys,
    Delegations,
    Profile,
    OrbitData,
    TrackingData,
    Team,
    Switch,
    Members,
}

impl std::fmt::Display for SideBar {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

#[derive(Props)]
pub struct TraceLayoutProps<'a> {
    selected_item: SideBar,
    title: &'a str,
    header: Element<'a>,
    children: Element<'a>,
    team_id: i32,
}

pub fn TraceLayout<'a>(cx: Scope<'a, TraceLayoutProps<'a>>) -> Element {
    cx.render(rsx! {
        AppLayout {
            title: cx.props.title,
            css_href1: primer_view_components_css.name,
            css_href2: index_css.name,
            js_href: index_js.name,
            fav_icon_src: favicon_svg.name,
            header: cx.render(rsx!(
                &cx.props.header
            ))
            sidebar: cx.render(rsx!(
                NavGroup {
                    heading: "Workflow",
                    content:  cx.render(rsx!(
                        NavItem {
                            id: SideBar::SpaceObjects.to_string(),
                            selected_item_id: cx.props.selected_item.to_string(),
                            href: super::routes::space_objects::index_route(cx.props.team_id),
                            icon: nav_space_objects_svg.name,
                            title: "Space Objects"
                        }
                        NavItem {
                            id: SideBar::Negotiations.to_string(),
                            selected_item_id: cx.props.selected_item.to_string(),
                            href: super::routes::negotiations::index_route(cx.props.team_id),
                            icon: handshake_svg.name,
                            title: "Negotiations",
                            trailing: cx.render(rsx!(
                                {
                                    LazyNodes::new(|f|
                                        f.text(format_args!(
                                        "<turbo-frame id='negotiation-count' src='{}'></turbo-frame>", 
                                        super::routes::negotiations::count_route(cx.props.team_id))))
                                }
                            ))
                        }
                    ))
                }
                NavGroup {
                    heading: "Data Sharing",
                    content:  cx.render(rsx!(
                        NavItem {
                            id: SideBar::Conjunctions.to_string(),
                            selected_item_id: cx.props.selected_item.to_string(),
                            href: super::routes::conjunctions::index_route(cx.props.team_id),
                            icon: nav_ccsds_data_svg.name,
                            title: "Conjunctions"
                        }
                        NavItem {
                            id: SideBar::OrbitData.to_string(),
                            selected_item_id: cx.props.selected_item.to_string(),
                            href: super::routes::orbit_data::index_route(cx.props.team_id),
                            icon: nav_ccsds_data_svg.name,
                            title: "Orbit Data"
                        }
                        NavItem {
                            id: SideBar::ApiKeys.to_string(),
                            selected_item_id: cx.props.selected_item.to_string(),
                            href: super::routes::api_keys::index_route(cx.props.team_id),
                            icon: nav_api_keys_svg.name,
                            title: "API Keys"
                        }
                    ))
                }
                NavGroup {
                    heading: "Collaboration",
                    content:  cx.render(rsx!(
                        NavItem {
                            id: SideBar::Team.to_string(),
                            selected_item_id: cx.props.selected_item.to_string(),
                            href: super::routes::team::index_route(cx.props.team_id),
                            icon: nav_members_svg.name,
                            title: "Team Members"
                        }
                        NavItem {
                            id: SideBar::Switch.to_string(),
                            selected_item_id: cx.props.selected_item.to_string(),
                            href: super::routes::team::switch_route(cx.props.team_id),
                            icon: nav_teams_svg.name,
                            title: "Your Teams"
                        }
                    ))
                }
                NavGroup {
                    heading: "System Admin",
                    content:  cx.render(rsx!(
                        NavItem {
                            id: SideBar::Dashboard.to_string(),
                            selected_item_id: cx.props.selected_item.to_string(),
                            href: super::routes::dashboard::index_route(cx.props.team_id),
                            icon: nav_dashboard_svg.name,
                            title: "Dashboard"
                        }
                    ))
                }
            ))
            sidebar_header: cx.render(rsx!(
                {
                    LazyNodes::new(|f| f.text(format_args!("<turbo-frame id='teams-popup' class='width-full' src='{}'></turbo-frame>", 
                        super::routes::team::teams_popup_route(cx.props.team_id))))
                }
            ))
            sidebar_footer: cx.render(rsx!(
                {
                    LazyNodes::new(|f| f.text(format_args!("<turbo-frame id='profile-popup' class='width-full' src='{}'></turbo-frame>",
                    super::routes::profile::profile_popup_route(cx.props.team_id))))
                }
            )),
            &cx.props.children
            {
                LazyNodes::new(|f| f.text(format_args!("<snack-bar></snack-bar>")))
            }
        }
        LogoutForm {}
    })
}
