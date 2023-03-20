pub mod common;

use grpc_api::ccsds::schema::*;
use grpc_api::trace::data_message::Data;
use grpc_api::trace::trace_client::TraceClient;
use grpc_api::trace::{DataMessage, EmptyRequest, UploadDataRequest};
use thirtyfour::prelude::*;

// let's set up the sequence of steps we want the browser to take
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn negotiations() -> WebDriverResult<()> {
    let config = common::Config::new().await;

    let driver = config.get_driver().await?;

    let result = negotiation(&driver, &config).await;

    driver.quit().await?;

    result?;

    Ok(())
}

async fn negotiation(driver: &WebDriver, config: &common::Config) -> WebDriverResult<()> {
    let delay = std::time::Duration::new(11, 0);
    driver.set_implicit_wait_timeout(delay).await?;

    driver.get(format!("{}/auth/sign_up", &config.host)).await?;

    let _email = common::register_user(driver, config).await?;

    // Ceate some space objects
    let norad_id1 = common::register_space_object(driver, "ISS".to_string()).await?;

    // Click away
    driver
        .find_element(By::LinkText("Negotiations"))
        .await?
        .click()
        .await?;

    driver
        .query(By::XPath("//turbo-frame//h3[text()='Negotiations']"))
        .first()
        .await?
        .wait_until()
        .displayed()
        .await?;

    let norad_id2 = common::register_space_object(driver, "SATCOM2".to_string()).await?;

    // Create a relevant CDM and send it.
    let cdm = CdmType {
        id: "".to_string(),
        version: "1.0".to_string(),
        header: Some(CdmHeader {
            message_id: common::random_number(),
            ..Default::default()
        }),
        body: Some(CdmBody {
            relative_metadata_data: Some(RelativeMetadataData {
                tca: "1.0".to_string(),
                miss_distance: Some(LengthType {
                    units: LengthUnits::M.into(),
                    value: 0.,
                }),
                collision_probability: 0.006,
                collision_probability_method: "1.0".to_string(),
                ..Default::default()
            }),
            segment: vec![
                CdmSegment {
                    metadata: Some(CdmMetadata {
                        object: 1,
                        object_designator: norad_id1.clone(),
                        catalog_name: "SATELLITE gRPC".to_string(),
                        object_name: "SATELLITE gRPC".to_string(),
                        international_designator: norad_id1,
                        ..Default::default()
                    }),
                    data: Some(CdmData {
                        ..Default::default()
                    }),
                },
                CdmSegment {
                    metadata: Some(CdmMetadata {
                        object: 2,
                        object_designator: norad_id2.clone(),
                        catalog_name: "SATCOM2".to_string(),
                        object_name: "SATCOM2".to_string(),
                        international_designator: norad_id2,
                        ..Default::default()
                    }),
                    data: Some(CdmData {
                        ..Default::default()
                    }),
                },
            ],
        }),
    };

    let request = UploadDataRequest {
        msg: Some(DataMessage {
            data: Some(Data::Cdm(cdm)),
        }),
        signature: Default::default(),
    };

    // Upload the data
    let mut client = TraceClient::connect("http://envoy:7400").await.unwrap();
    client.upload_data(request).await.unwrap();

    driver
        .find_element(By::LinkText("Conjunctions"))
        .await?
        .click()
        .await?;

    let elem = driver
        .query(By::XPath("//turbo-frame//h3[text()='Conjunctions']"))
        .first()
        .await?;
    // Wait until the element is displayed.
    elem.wait_until().displayed().await?;

    let table_cell = driver
        .find_element(By::XPath("//table/tbody/tr[1]/td[2]"))
        .await?;

    assert_eq!(table_cell.text().await?, "SATELLITE gRPC");

    driver
        .find_element(By::LinkText("Negotiations"))
        .await?
        .click()
        .await?;

    driver
        .query(By::XPath("//turbo-frame//h3[text()='Negotiations']"))
        .first()
        .await?
        .wait_until()
        .displayed()
        .await?;

    let request = EmptyRequest {};
    client.process_negotiations(request).await.unwrap();

    // Click away and click back

    driver
        .find_element(By::LinkText("Space Objects"))
        .await?
        .click()
        .await?;

    driver
        .query(By::XPath("//turbo-frame//h3[text()='Space Objects']"))
        .first()
        .await?
        .wait_until()
        .displayed()
        .await?;

    driver
        .find_element(By::LinkText("Negotiations"))
        .await?
        .click()
        .await?;

    driver
        .query(By::XPath("//turbo-frame//h3[text()='Negotiations']"))
        .first()
        .await?
        .wait_until()
        .displayed()
        .await?;

    Ok(())
}
