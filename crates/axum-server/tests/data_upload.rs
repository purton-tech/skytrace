pub mod common;

use thirtyfour::prelude::*;
use tokio::time::{sleep, Duration};

// let's set up the sequence of steps we want the browser to take
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn data_upload() -> WebDriverResult<()> {
    let config = common::Config::new().await;

    let driver = config.get_driver().await?;

    let result = upload(&driver, &config).await;

    driver.quit().await?;

    result?;

    Ok(())
}

async fn upload(driver: &WebDriver, config: &common::Config) -> WebDriverResult<()> {
    let delay = std::time::Duration::new(11, 0);
    driver.set_implicit_wait_timeout(delay).await?;

    driver.get(format!("{}/auth/sign_up", &config.host)).await?;

    let _email = common::register_user(driver, config).await?;

    // Upload a CDM
    upload_cdm(driver).await?;

    Ok(())
}

pub async fn upload_cdm(driver: &WebDriver) -> WebDriverResult<()> {
    let objects_link = driver.find_element(By::LinkText("Conjunctions")).await?;
    objects_link.click().await?;

    let upload_button = driver
        .find_element(By::XPath("//button[text()='Upload CCSDS Data']"))
        .await?;
    upload_button.click().await?;

    driver
        .find_element(By::XPath(".//*[@type='file']"))
        .await?
        .send_keys("/workspace/cdm.xml")
        .await?;

    driver
        .find_element(By::XPath("//footer//button[text()='Upload Data']"))
        .await?
        .click()
        .await?;

    // Make sure the drawer is gone
    sleep(Duration::from_millis(1000)).await;

    let objects_link = driver.find_element(By::LinkText("Conjunctions")).await?;
    objects_link.click().await?;

    let table_cell = driver
        .find_element(By::XPath("//table/tbody/tr[1]/td[2]"))
        .await?;

    assert_eq!(table_cell.text().await?, "SATELLITE A");

    // Make sure the drawer is gone
    sleep(Duration::from_millis(1000)).await;

    Ok(())
}
