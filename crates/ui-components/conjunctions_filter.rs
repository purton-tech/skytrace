#![allow(non_snake_case)]
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq, Eq)]
pub struct ConjunctionFilterProps<'a> {
    pub filter_action: &'a str,
}

pub fn ConjunctionFilter<'a>(cx: Scope<'a, ConjunctionFilterProps<'a>>) -> Element {
    cx.render(rsx!(
        form {
            method: "get",
            action: "{cx.props.filter_action}",
            Drawer {
                label: "Conjunctions Data Filte",
                trigger_id: "filter-conjunction-data",
                DrawerBody {
                    details {
                        summary {
                            "Probability of Collision"
                        }
                        div {
                            class: "Box d-flex flex-column p-2",
                            label {
                                "Show rows where :"
                            }
                            label {
                                "greater than or equal"
                            }
                            input {
                                class: "mb-2",
                                name: "pc_greater"
                            }
                            label {
                                "AND"
                            }
                            label {
                                "less than or equal"
                            }
                            input {
                                class: "mb-2",
                                name: "pc_less"
                            }
                        }
                    }
                    details {
                        summary {
                            "First Object"
                        }
                        div {
                            class: "Box d-flex flex-column p-2",
                            label {
                                "Show rows where :"
                            }
                            label {
                                "name contains"
                            }
                            input {
                                class: "mb-2",
                                name: "first_object"
                            }
                        }
                    }
                    details {
                        summary {
                            "Second Object"
                        }
                        div {
                            class: "Box d-flex flex-column p-2",
                            label {
                                "Show rows where :"
                            }
                            label {
                                "name contains"
                            }
                            input {
                                class: "mb-2",
                                name: "second_object"
                            }
                        }
                    }
                }
                DrawerFooter {
                    Button {
                        button_type: ButtonType::Submit,
                        button_scheme: ButtonScheme::Primary,
                        "Apply Filter"
                    }
                }
            }
        }
    ))
}
