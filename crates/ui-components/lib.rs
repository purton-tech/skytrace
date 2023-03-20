pub mod api_keys;
pub mod conjunctions;
pub mod conjunctions_filter;
pub mod dashboard;
pub mod data_upload;
pub mod logout_form;
pub mod negotiations;
pub mod orbit_data;
pub mod profile;
pub mod profile_popup;
pub mod space_objects;
pub mod team_members;
pub mod teams;
pub mod trace_layout;

pub mod routes {
    pub mod dashboard {
        pub static INDEX: &str = "/app/team/:team_id/dashboard";

        pub fn index_route(organisation_id: i32) -> String {
            format!("/app/team/{}/dashboard", organisation_id)
        }
    }

    pub mod data_upload {

        pub static UPLOAD: &str = "/app/team/:team_id/upload";

        pub fn upload_route(organisation_id: i32) -> String {
            format!("/app/team/{}/upload", organisation_id)
        }
    }

    pub mod team {
        pub fn index_route(organisation_id: i32) -> String {
            format!("/app/team/{}/team", organisation_id)
        }

        pub fn switch_route(organisation_id: i32) -> String {
            format!("/app/team/{}/switch", organisation_id)
        }

        pub fn teams_popup_route(organisation_id: i32) -> String {
            format!("/app/team/{}/teams_popup", organisation_id)
        }

        pub fn create_route(organisation_id: i32) -> String {
            format!("/app/team/{}/create_invite", organisation_id)
        }

        pub fn delete_route(organisation_id: i32) -> String {
            format!("/app/team/{}/delete", organisation_id)
        }

        pub fn set_name_route(organisation_id: i32) -> String {
            format!("/app/team/{}/set_name", organisation_id)
        }

        pub fn new_team_route(organisation_id: i32) -> String {
            format!("/app/team/{}/new", organisation_id)
        }
    }

    pub mod space_objects {

        pub static INDEX: &str = "/app/team/:team_id/space_objects";
        pub static NEWOBJECT: &str = "/app/team/:team_id/add_space_object";

        pub fn index_route(organisation_id: i32) -> String {
            format!("/app/team/{}/space_objects", organisation_id)
        }

        pub fn new_route(organisation_id: i32) -> String {
            format!("/app/team/{}/add_space_object", organisation_id)
        }
    }

    pub mod profile {

        pub fn set_details_route(organisation_id: i32) -> String {
            format!("/app/team/{}/set_details", organisation_id)
        }

        pub fn index_route(organisation_id: i32) -> String {
            format!("/app/team/{}/profile", organisation_id)
        }

        pub fn profile_popup_route(organisation_id: i32) -> String {
            format!("/app/team/{}/profile_popup", organisation_id)
        }
    }

    pub mod negotiations {
        pub static INDEX: &str = "/app/team/:team_id/negotiations";
        pub static NEGOTIATION: &str = "/app/team/:team_id/negotiation/:idor_negotiation_id";
        pub static SEND_MESSAGE: &str = "/app/team/:team_id/send_message/:idor_negotiation_id";
        pub static CHANGE_STATUS: &str = "/app/team/:team_id/status/:idor_negotiation_id";

        pub fn index_route(organisation_id: i32) -> String {
            format!("/app/team/{}/negotiations", organisation_id)
        }

        pub fn count_route(organisation_id: i32) -> String {
            format!("/app/team/{}/negotiations_count", organisation_id)
        }

        pub fn negotiation_route(organisation_id: i32, negotiation_id: i32) -> String {
            format!(
                "/app/team/{}/negotiation/{}",
                organisation_id, negotiation_id
            )
        }

        pub fn send_message_route(organisation_id: i32, negotiation_id: i32) -> String {
            format!(
                "/app/team/{}/send_message/{}",
                organisation_id, negotiation_id
            )
        }

        pub fn change_status_route(organisation_id: i32, negotiation_id: i32) -> String {
            format!("/app/team/{}/status/{}", organisation_id, negotiation_id)
        }
    }

    pub mod conjunctions {
        pub static INDEX: &str = "/app/team/:team_id/conjunctions";

        pub fn index_route(organisation_id: i32) -> String {
            format!("/app/team/{}/conjunctions", organisation_id)
        }
    }

    pub mod orbit_data {
        pub static INDEX: &str = "/app/team/:team_id/orbit_data";

        pub fn index_route(organisation_id: i32) -> String {
            format!("/app/team/{}/orbit_data", organisation_id)
        }
    }

    pub mod tracking_data {
        pub static INDEX: &str = "/app/team/:team_id/tracking_data";

        pub fn index_route(organisation_id: i32) -> String {
            format!("/app/team/{}/tracking_data", organisation_id)
        }
    }

    pub mod api_keys {
        pub static INDEX: &str = "/app/team/:team_id/api_keys";
        pub static NEW: &str = "/app/team/:team_id/api_keys/new";

        pub fn index_route(organisation_id: i32) -> String {
            format!("/app/team/{}/api_keys", organisation_id)
        }

        pub fn new_route(organisation_id: i32) -> String {
            format!("/app/team/{}/api_keys/new", organisation_id)
        }
    }
}
