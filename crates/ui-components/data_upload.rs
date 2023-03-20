#![allow(non_snake_case)]
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq, Eq)]
pub struct DataUploadProps<'a> {
    pub upload_action: &'a str,
}

pub fn DataUpload<'a>(cx: Scope<'a, DataUploadProps<'a>>) -> Element {
    cx.render(rsx!(
        form {
            method: "post",
            id: "payload-form",
            action: "{cx.props.upload_action}",
            {
                LazyNodes::new(|f| f.text(format_args!("<data-upload id='upload-ccsds-data' label='CCSDS Data Upload'>")))
            }
            template {
                "slot": "body",
                div {
                    id: "upload-fields",
                    class: "d-flex flex-column",
                    h4 {
                        "Choose a file to upload - Step 1 of 2"
                    }
                    Alert {
                        class: "mb-4 mt-4",
                        "The following CCSDS based message types shall be accepted for upload"
                        p {
                            "CDM - Conjunction Data Message according to CCSDS 508.0"
                        }
                    }
                    Select {
                        name: "public",
                        required: true,
                        label: "Data sharing policy",
                        help_text: "Choose the sharing policy for this upload",
                        option {
                            value: "false",
                            "Private - Only you can see this data"
                        }
                        option {
                            value: "true",
                            "Public - This data will be available for all users to see"
                        }
                    }
                    label {
                        "Select your CCSDS file"
                    }
                    input {
                        "type": "file",
                        name: "file",
                        id: "file-field"
                    }
                }
                div {
                    id: "verify-fields",
                    class: "d-none",
                    h4 {
                        "Summary - Step 2 of 2"
                    }
                    Alert {
                        class: "mb-4 mt-4",
                        "This is a summary of your upload. When you are ready click 'Upload Data'"
                    }
                    DataTable {
                        table {
                            class: "m_table",
                            tbody {
                                class: "body",
                                id: "verify-table"
                            }
                        }
                    }
                    Alert {
                        alert_color: AlertColor::Warn,
                        class: "mt-4",
                        p {
                            class: "m_signature",
                            id: "signature-view"
                        }
                    }
                }
                input {
                    "type": "hidden",
                    id: "payload",
                    name: "payload"
                }
                input {
                    "type": "hidden",
                    id: "payloadtype",
                    name: "payloadtype"
                }
                input {
                    "type": "hidden",
                    id: "signature",
                    name: "signature"
                }
            }
            template {
                "slot": "footer",
                Button {
                    id: "submit-button",
                    button_type: ButtonType::Submit,
                    button_scheme: ButtonScheme::Primary,
                    "Upload Data"
                }
            }
            {
                LazyNodes::new(|f| f.text(format_args!("</data-upload>")))
            }
        }
    ))
}
