use crate::ccsds::schema;
use crate::trace;
use serde::{Deserialize, Serialize};
use serde_xml_rs::{self, EventReader, ParserConfig};

use crate::errors::CustomError;

pub async fn convert_to_proto(xml: &str) -> Result<trace::UploadDataRequest, CustomError> {
    let config = ParserConfig::new()
        .trim_whitespace(false)
        .whitespace_to_characters(false);
    let mut deserializer =
        serde_xml_rs::Deserializer::new(EventReader::new_with_config(xml.as_bytes(), config));
    let oem = Oem::deserialize(&mut deserializer).unwrap();

    tracing::debug!("OEM: {:?}", oem);

    let segments: Vec<schema::OemSegment> = oem
        .body
        .segment
        .iter()
        .map(|segment| {
            schema::OemSegment {
                metadata: Some(schema::OemMetadata {
                    object_name: segment.metadata.object_name.clone(),
                    object_id: segment.metadata.object_id.clone(),
                    center_name: segment.metadata.center_name.clone(),
                    ref_frame: segment.metadata.ref_frame.clone(),
                    time_system: segment.metadata.time_system.clone(),
                    start_time: segment.metadata.start_time.clone(),
                    useable_start_time: segment.metadata.useable_start_time.clone(),
                    useable_stop_time: segment.metadata.useable_stop_time.clone(),
                    stop_time: segment.metadata.stop_time.clone(),
                    interpolation: segment.metadata.interpolation.clone(),
                    interpolation_degree: segment.metadata.interpolation_degree as u32,
                    ..Default::default()
                }),
                data: Some(schema::OemData {
                    comment: segment.data.comments.clone(),
                    state_vector: segment
                        .data
                        .state_vectors
                        .iter()
                        .map(|state_vector| schema::StateVectorAccType {
                            epoch: state_vector.epoch.clone(),
                            x: convert_to_position_units(state_vector.x),
                            y: convert_to_position_units(state_vector.y),
                            z: convert_to_position_units(state_vector.z),
                            x_dot: convert_to_velocity_units(state_vector.x_dot),
                            y_dot: convert_to_velocity_units(state_vector.y_dot),
                            z_dot: convert_to_velocity_units(state_vector.z_dot),
                            x_ddot: convert_to_acc_type(state_vector.x_ddot),
                            y_ddot: convert_to_acc_type(state_vector.y_ddot),
                            z_ddot: convert_to_acc_type(state_vector.z_ddot),
                        })
                        .collect(),
                    covariance_matrix: segment
                        .data
                        .covariance_matrices
                        .iter()
                        .map(|covariance_matrix| schema::OemCovarianceMatrixType {
                            //epoch: covariance_matrix.epoch.clone(),
                            //cov_ref_frame: covariance_matrix.cov_ref_frame.clone(),
                            cx_x: convert_to_position_covariance_units(covariance_matrix.cx_x),
                            cy_x: convert_to_position_covariance_units(covariance_matrix.cy_x),
                            cy_y: convert_to_position_covariance_units(covariance_matrix.cy_y),
                            cz_x: convert_to_position_covariance_units(covariance_matrix.cz_x),
                            cz_y: convert_to_position_covariance_units(covariance_matrix.cz_y),
                            cz_z: convert_to_position_covariance_units(covariance_matrix.cz_z),
                            cx_dot_x: convert_to_position_velocity_covariance_units(
                                covariance_matrix.cx_dot_x,
                            ),
                            cx_dot_y: convert_to_position_velocity_covariance_units(
                                covariance_matrix.cx_dot_y,
                            ),
                            cx_dot_z: convert_to_position_velocity_covariance_units(
                                covariance_matrix.cx_dot_z,
                            ),
                            cx_dot_x_dot: convert_to_velocity_covariance_units(
                                covariance_matrix.cx_dot_x_dot,
                            ),
                            ..Default::default()
                        })
                        .collect(),
                }),
            }
        })
        .collect();

    let upload_data_request = trace::UploadDataRequest {
        msg: Some(crate::trace::DataMessage {
            data: Some(trace::data_message::Data::Oem(schema::OemType {
                header: Some(schema::NdmHeader {
                    comment: oem.header.comments,
                    creation_date: oem.header.creation_date,
                    originator: oem.header.originator,
                }),
                body: Some(schema::OemBody { segment: segments }),
                ..Default::default()
            })),
        }),
        signature: "".to_string(),
    };

    Ok(upload_data_request)
}

fn convert_to_velocity_covariance_units(value: f64) -> Option<schema::VelocityCovarianceType> {
    Some(schema::VelocityCovarianceType {
        value,
        units: schema::VelocityCovarianceUnits::Unspecified.into(),
    })
}

fn convert_to_position_velocity_covariance_units(
    value: f64,
) -> Option<schema::PositionVelocityCovarianceType> {
    Some(schema::PositionVelocityCovarianceType {
        value,
        units: schema::PositionVelocityCovarianceUnits::Unspecified.into(),
    })
}

fn convert_to_position_covariance_units(value: f64) -> Option<schema::PositionCovarianceType> {
    Some(schema::PositionCovarianceType {
        value,
        units: schema::PositionCovarianceUnits::Unspecified.into(),
    })
}

fn convert_to_position_units(value: f64) -> Option<schema::PositionType> {
    Some(schema::PositionType {
        value,
        units: schema::PositionUnits::Unspecified.into(),
    })
}

fn convert_to_velocity_units(value: f64) -> Option<schema::VelocityType> {
    Some(schema::VelocityType {
        value,
        units: schema::VelocityUnits::Unspecified.into(),
    })
}

fn convert_to_acc_type(value: f64) -> Option<schema::AccType> {
    Some(schema::AccType {
        value,
        units: schema::AccUnits::Unspecified.into(),
    })
}

#[derive(Debug, Deserialize, Serialize)]
struct Oem {
    header: Header,
    body: Body,
    id: String,
    version: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Header {
    #[serde(rename = "COMMENT", default)]
    comments: Vec<String>,
    #[serde(rename = "CREATION_DATE")]
    creation_date: String,
    #[serde(rename = "ORIGINATOR")]
    originator: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Body {
    segment: Vec<Segment>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Segment {
    metadata: Metadata,
    data: Data,
}

#[derive(Debug, Deserialize, Serialize)]
struct Metadata {
    #[serde(rename = "OBJECT_NAME")]
    object_name: String,
    #[serde(rename = "OBJECT_ID")]
    object_id: String,
    #[serde(rename = "CENTER_NAME")]
    center_name: String,
    #[serde(rename = "REF_FRAME")]
    ref_frame: String,
    #[serde(rename = "TIME_SYSTEM")]
    time_system: String,
    #[serde(rename = "START_TIME")]
    start_time: String,
    #[serde(rename = "USEABLE_START_TIME")]
    useable_start_time: String,
    #[serde(rename = "USEABLE_STOP_TIME")]
    useable_stop_time: String,
    #[serde(rename = "STOP_TIME")]
    stop_time: String,
    #[serde(rename = "INTERPOLATION")]
    interpolation: String,
    #[serde(rename = "INTERPOLATION_DEGREE")]
    interpolation_degree: i32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Data {
    #[serde(rename = "COMMENT", default)]
    comments: Vec<String>,
    #[serde(rename = "stateVector")]
    state_vectors: Vec<StateVector>,
    #[serde(rename = "covarianceMatrix")]
    covariance_matrices: Vec<CovarianceMatrix>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
struct StateVector {
    epoch: String,
    x: f64,
    y: f64,
    z: f64,
    x_dot: f64,
    y_dot: f64,
    z_dot: f64,
    x_ddot: f64,
    y_ddot: f64,
    z_ddot: f64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
struct CovarianceMatrix {
    epoch: String,
    cov_ref_frame: String,
    cx_x: f64,
    cy_x: f64,
    cy_y: f64,
    cz_x: f64,
    cz_y: f64,
    cz_z: f64,
    cx_dot_x: f64,
    cx_dot_y: f64,
    cx_dot_z: f64,
    cx_dot_x_dot: f64,
    cy_dot_x: f64,
    cy_dot_y: f64,
    cy_dot_z: f64,
    cy_dot_x_dot: f64,
    cy_dot_y_dot: f64,
    cz_dot_x: f64,
    cz_dot_y: f64,
    cz_dot_z: f64,
    cz_dot_x_dot: f64,
    cz_dot_y_dot: f64,
    cz_dot_z_dot: f64,
}

#[cfg(test)]
mod tests {

    const OEM_XML: &str = "<?xml version='1.0' encoding='UTF-8'?>
    <oem xmlns:xsi='http://www.w3.org/2001/XMLSchema-instance'
         xsi:noNamespaceSchemaLocation='http://sanaregistry.org/r/ndmxml/ndmxml-1.0-master.xsd'
         id='CCSDS_OEM_VERS'
         version='2.0'>
        <header>
            <COMMENT>THIS EXAMPLE CONFORMS TO FIGURE 5-2 IN 502.0-B-2</COMMENT>
            <COMMENT>OEM WITH OPTIONAL ACCELERATIONS CAN ONLY BE OEM VERSION 2.0</COMMENT>
            <CREATION_DATE>$CREATION_EPOCH$</CREATION_DATE>
            <ORIGINATOR>NASA/JPL</ORIGINATOR>
        </header>
        <body>
            <segment>
                <metadata>
                    <OBJECT_NAME>MARS GLOBAL SURVEYOR</OBJECT_NAME>
                    <OBJECT_ID>$OBJECT_ID$</OBJECT_ID>
                    <CENTER_NAME>MARS BARYCENTER</CENTER_NAME>
                    <REF_FRAME>EME2000</REF_FRAME>
                    <TIME_SYSTEM>UTC</TIME_SYSTEM>
                    <START_TIME>$START_EPOCH$</START_TIME>
                    <USEABLE_START_TIME>1996-12-18T12:10:00.331</USEABLE_START_TIME>
                    <USEABLE_STOP_TIME>1996-12-28T21:23:00.331</USEABLE_STOP_TIME>
                    <STOP_TIME>$STOP_EPOCH$</STOP_TIME>
                    <INTERPOLATION>HERMITE</INTERPOLATION>
                    <INTERPOLATION_DEGREE>7</INTERPOLATION_DEGREE>
                </metadata>
                <data>
                    <COMMENT>Produced by M.R. Sombedody, MSOO NAV/JPL, 1996 OCT 11. It is</COMMENT>
                    <COMMENT>to be used for DSN scheduling purposes only.</COMMENT>
                    <COMMENT>$RANDOM_COMMENT$</COMMENT>
                    <stateVector>
                        <EPOCH>1996-12-18T12:00:00.331</EPOCH>
                        <X>2789.6</X>
                        <Y>-280.0</Y>
                        <Z>-1746.8</Z>
                        <X_DOT>4.73</X_DOT>
                        <Y_DOT>-2.50</Y_DOT>
                        <Z_DOT>-1.04</Z_DOT>
                        <X_DDOT>0.008</X_DDOT>
                        <Y_DDOT>0.001</Y_DDOT>
                        <Z_DDOT>-0.159</Z_DDOT>
                    </stateVector>
                    <stateVector>
                        <EPOCH>1996-12-18T12:01:00.331</EPOCH>
                        <X>2783.4</X>
                        <Y>-308.1</Y>
                        <Z>-1877.1</Z>
                        <X_DOT>5.19</X_DOT>
                        <Y_DOT>-2.42</Y_DOT>
                        <Z_DOT>-2.00</Z_DOT>
                        <X_DDOT>0.008</X_DDOT>
                        <Y_DDOT>0.001</Y_DDOT>
                        <Z_DDOT>0.001</Z_DDOT>
                    </stateVector>
                    <stateVector>
                        <EPOCH>1996-12-18T12:02:00.331</EPOCH>
                        <X>2776.0</X>
                        <Y>-336.9</Y>
                        <Z>-2008.7</Z>
                        <X_DOT>5.64</X_DOT>
                        <Y_DOT>-2.34</Y_DOT>
                        <Z_DOT>-1.95</Z_DOT>
                        <X_DDOT>0.008</X_DDOT>
                        <Y_DDOT>0.001</Y_DDOT>
                        <Z_DDOT>0.159</Z_DDOT>
                    </stateVector>
                    <stateVector>
                        <EPOCH>1996-12-28T21:28:00.331</EPOCH>
                        <X>-3881.0</X>
                        <Y>564.0</Y>
                        <Z>-682.8</Z>
                        <X_DOT>-3.29</X_DOT>
                        <Y_DOT>-3.67</Y_DOT>
                        <Z_DOT>1.64</Z_DOT>
                        <X_DDOT>-0.003</X_DDOT>
                        <Y_DDOT>0.000</Y_DDOT>
                        <Z_DDOT>0.000</Z_DDOT>
                    </stateVector>
                    <covarianceMatrix>
                        <EPOCH>1996-12-28T21:29:07.267</EPOCH>
                        <COV_REF_FRAME>ITRF-97</COV_REF_FRAME>
                        <CX_X>0.316</CX_X>
                        <CY_X>0.722</CY_X>
                        <CY_Y>0.518</CY_Y>
                        <CZ_X>0.202</CZ_X>
                        <CZ_Y>0.715</CZ_Y>
                        <CZ_Z>0.002</CZ_Z>
                        <CX_DOT_X>0.912</CX_DOT_X>
                        <CX_DOT_Y>0.306</CX_DOT_Y>
                        <CX_DOT_Z>0.276</CX_DOT_Z>
                        <CX_DOT_X_DOT>0.797</CX_DOT_X_DOT>
                        <CY_DOT_X>0.562</CY_DOT_X>
                        <CY_DOT_Y>0.899</CY_DOT_Y>
                        <CY_DOT_Z>0.022</CY_DOT_Z>
                        <CY_DOT_X_DOT>0.079</CY_DOT_X_DOT>
                        <CY_DOT_Y_DOT>0.415</CY_DOT_Y_DOT>
                        <CZ_DOT_X>0.245</CZ_DOT_X>
                        <CZ_DOT_Y>0.965</CZ_DOT_Y>
                        <CZ_DOT_Z>0.950</CZ_DOT_Z>
                        <CZ_DOT_X_DOT>0.435</CZ_DOT_X_DOT>
                        <CZ_DOT_Y_DOT>0.621</CZ_DOT_Y_DOT>
                        <CZ_DOT_Z_DOT>0.991</CZ_DOT_Z_DOT>
                    </covarianceMatrix>
                </data>
            </segment>
        </body>
    </oem>";

    use super::*;

    #[tokio::test]
    pub async fn load_xml() {
        let oem = convert_to_proto(OEM_XML).await.unwrap();
        println!("{:#?}", oem);
    }
}
