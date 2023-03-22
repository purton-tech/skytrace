mod config;

use grpc_api::ccsds::schema::{
    CdmBody, CdmHeader, CdmMetadata, CdmSegment, CdmType, ObjectDescriptionType,
    RelativeMetadataData,
};
use grpc_api::trace::trace_client::TraceClient;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConjunctionData {
    #[serde(rename(deserialize = "CDM_ID"))]
    pub cdm_id: String,
    #[serde(rename(deserialize = "CREATED"))]
    pub created: Option<String>,
    #[serde(rename(deserialize = "EMERGENCY_REPORTABLE"))]
    pub emergency_reportable: Option<String>,
    #[serde(rename(deserialize = "TCA"))]
    pub time_of_closest_approach: Option<String>,
    #[serde(rename(deserialize = "PC"))]
    pub probability_of_collision: Option<String>,
    #[serde(rename(deserialize = "SAT_1_ID"))]
    pub sat_1_id: Option<String>,
    #[serde(rename(deserialize = "SAT_1_NAME"))]
    pub sat_1_name: Option<String>,
    #[serde(rename(deserialize = "SAT1_OBJECT_TYPE"))]
    pub sat1_object_type: Option<String>,
    #[serde(rename(deserialize = "SAT1_RCS"))]
    pub sat1_rcs: Option<String>,
    #[serde(rename(deserialize = "SAT_1_EXCL_VOL"))]
    pub sat_1_excl_vol: Option<String>,
    #[serde(rename(deserialize = "SAT_2_ID"))]
    pub sat_2_id: Option<String>,
    #[serde(rename(deserialize = "SAT_2_NAME"))]
    pub sat_2_name: Option<String>,
    #[serde(rename(deserialize = "SAT2_OBJECT_TYPE"))]
    pub sat2_object_type: Option<String>,
    #[serde(rename(deserialize = "SAT2_RCS"))]
    pub sat2_rcs: Option<String>,
    #[serde(rename(deserialize = "SAT_2_EXCL_VOL"))]
    pub sat_2_excl_vol: Option<String>,
}

#[tokio::main]
async fn main() {
    let config = config::Config::new();

    let response = ureq::post("https://www.space-track.org/ajaxauth/login").send_form(&[
        ("identity", &config.space_track_identity),
        ("password", &config.space_track_password),
        ("query", "https://www.space-track.org/basicspacedata/query/class/cdm_public/SAT1_OBJECT_TYPE/payload/SAT2_OBJECT_TYPE/payload/orderby/CDM_ID%20asc/emptyresult/show"),
    ]).unwrap();

    let cdms: Vec<ConjunctionData> = response.into_json().unwrap();

    let cdms = json_to_proto(cdms);

    let mut client = TraceClient::connect(config.base_url).await.unwrap();

    for cdm in cdms {
        let dm = grpc_api::trace::DataMessage {
            data: Some(grpc_api::trace::data_message::Data::Cdm(cdm)),
        };
        let ur = grpc_api::trace::UploadDataRequest {
            msg: Some(dm),
            signature: "None".to_string(),
        };

        let response = client.upload_data(ur).await.unwrap();

        println!("gRPC Response: {:?}", response);
    }
}

fn json_to_proto(cdms: Vec<ConjunctionData>) -> Vec<CdmType> {
    let cdms: Vec<CdmType> = cdms
        .into_iter()
        .filter_map(|cdm| {
            if let (
                Some(created),
                Some(tca),
                Some(probability_of_collision),
                Some(sat1_id),
                Some(sat1_name),
                Some(sat1_object_type),
                Some(_sat1_rcs),
                Some(_sat1_excl_vol),
                Some(sat2_id),
                Some(sat2_name),
                Some(sat2_object_type),
                Some(_sat2_rcs),
                Some(_sat2_excl_vol),
            ) = (
                cdm.created,
                cdm.time_of_closest_approach,
                cdm.probability_of_collision,
                cdm.sat_1_id,
                cdm.sat_1_name,
                cdm.sat1_object_type,
                cdm.sat1_rcs,
                cdm.sat_1_excl_vol,
                cdm.sat_2_id,
                cdm.sat_2_name,
                cdm.sat2_object_type,
                cdm.sat2_rcs,
                cdm.sat_2_excl_vol,
            ) {
                let header = CdmHeader {
                    comment: vec!["Space-Track".to_string()],
                    creation_date: created,
                    originator: "Trace Space-Track Feed".to_string(),
                    message_for: "Trace".to_string(),
                    message_id: cdm.cdm_id.clone(),
                };
                let poc = probability_of_collision.parse::<f64>();
                let mut poc_f64 = 0.;
                if let Ok(poc) = poc {
                    poc_f64 = poc;
                }
                let relative_meta = RelativeMetadataData {
                    tca,
                    collision_probability: poc_f64,
                    ..Default::default()
                };

                let object1_meta = create_object_meta(1, sat1_id, sat1_name, &sat1_object_type);
                let object2_meta = create_object_meta(2, sat2_id, sat2_name, &sat2_object_type);

                let msg = CdmType {
                    id: cdm.cdm_id,
                    version: "1.0".to_string(),
                    header: Some(header),
                    body: Some(CdmBody {
                        relative_metadata_data: Some(relative_meta),
                        segment: vec![
                            CdmSegment {
                                data: None,
                                metadata: Some(object1_meta),
                            },
                            CdmSegment {
                                data: None,
                                metadata: Some(object2_meta),
                            },
                        ],
                    }),
                };
                Some(msg)
            } else {
                None
            }
        })
        .collect();
    cdms
}

fn create_object_meta(number: i32, id: String, name: String, obj_type: &str) -> CdmMetadata {
    CdmMetadata {
        object: number,
        object_designator: id,
        catalog_name: name.clone(),
        object_name: name,
        object_type: object_type_from_string(obj_type).into(),
        ..Default::default()
    }
}

fn object_type_from_string(object_type: &str) -> ObjectDescriptionType {
    match object_type {
        "payload" => ObjectDescriptionType::Payload,
        "rocket_body" => ObjectDescriptionType::RocketBody,
        "debris" => ObjectDescriptionType::Debris,
        "upper_stage" => ObjectDescriptionType::UpperStage,
        "other" => ObjectDescriptionType::Other,
        _ => ObjectDescriptionType::Unspecified,
    }
}
