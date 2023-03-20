#![allow(non_snake_case)]
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq, Eq)]
pub struct FormProps {
    organisation_id: i32,
}

pub fn SpaceObjectForm(cx: Scope<FormProps>) -> Element {
    cx.render(rsx!(
        Drawer {
            label: "New Space Object",
            trigger_id: "add-space-object",
            submit_action: crate::routes::space_objects::new_route(cx.props.organisation_id),
            DrawerBody {
                div {
                    class: "d-flex flex-column",
                    Input {
                        input_type: InputType::Text,
                        placeholder: "ISS",
                        help_text: "The objects name"
                        required: true,
                        label: "Name",
                        name: "name"
                    }
                    Input {
                        input_type: InputType::Text,
                        label_class: "mt-3",
                        label: "The COSPAR ID",
                        name: "cospar_id",
                        placeholder: "1998-067A",
                        help_text: "Spacecraft ID (Cospar)"
                        required: true
                    }
                    Input {
                        input_type: InputType::Text,
                        label_class: "mt-3",
                        label: "The NORAD ID",
                        name: "norad_id",
                        placeholder: "25544",
                        help_text: "Spacecraft ID (Norad)"
                        required: true
                    }
                    Select {
                        label: "Can this object perform manoeuvres?",
                        label_class: "mt-3",
                        name: "manoeuvrability",
                        required: true,
                        option {
                            value: "true",
                            "This object can perform manuevres"
                        }
                        option {
                            value: "false",
                            "This object CAN NOT perform manuevres"
                        }
                    }
                    Input {
                        input_type: InputType::Text,
                        label_class: "mt-3",
                        label: "Manoeuvre Implementation Latency",
                        name: "implementation_latency",
                        help_text: "(minimum time required between a manoeuvre assignment and its
                            execution considering nominal ground-station availability)"
                        required: true
                    }
                    Select {
                        label: "Data sharing policy",
                        label_class: "mt-3",
                        name: "public",
                        required: true,
                        option {
                            value: "true",
                            "Public - This data will be available for all users to see"
                        }
                        option {
                            value: "false",
                            "Private - Only you can see this data"
                        }
                    }
                    Select {
                        label: "Avoidance Strategy",
                        label_class: "mt-3",
                        name: "avoidance_strategy",
                        option {
                            value: "Thrusting",
                            "Thrusting"
                        }
                        option {
                            value: "In Track",
                            "In Track"
                        }
                    }
                    Select {
                        label: "Manoeuvring Strategy",
                        label_class: "mt-3",
                        name: "manoeuvering_strategy",
                        option {
                            value: "Impulsive",
                            "Impulsive"
                        }
                        option {
                            value: "continuous electrical",
                            "continuous electrical"
                        }
                        option {
                            value: "continuous chemical",
                            "continuous chemical"
                        }
                    }
                    Input {
                        input_type: InputType::Text,
                        label_class: "mt-3",
                        label: "Remaining fuel",
                        name: "remaining_fuel",
                    }
                    Select {
                        label: "Type of RSO",
                        label_class: "mt-3",
                        name: "type_of_rso",
                        option {
                            value: "Manned",
                            "Manned"
                        }
                        option {
                            value: "Unmanned",
                            "Unmanned"
                        }
                    }
                }
            }
            DrawerFooter {
                Button {
                    button_type: ButtonType::Submit,
                    button_scheme: ButtonScheme::Primary,
                    "Add Space Object"
                }
            }
        }
    ))
}
