use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::extract::{Extension, Path, Query};
use axum::response::Html;
use db::queries::{orbit_data, organisations};
use db::{Params, Pool, Transaction};
use serde::Deserialize;

static PAGE_SIZE: usize = 20;

#[derive(Deserialize, Debug)]
pub struct Filter {
    seek_offset: Option<i32>,
}

pub async fn index(
    Path(team_id): Path<i32>,
    Query(filter): Query<Filter>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    let orbit_data = filter_orbit_data(&transaction, &filter, PAGE_SIZE as i64).await?;

    let org = organisations::organisation()
        .bind(&transaction, &team_id)
        .one()
        .await?;

    let next_page_url = if orbit_data.len() > PAGE_SIZE {
        Some(generate_url(&filter, org.id, orbit_data[PAGE_SIZE].id))
    } else {
        None
    };

    Ok(Html(ui_components::orbit_data::orbit_data(
        orbit_data,
        next_page_url,
        org.id,
    )))
}

fn generate_url(_filter: &Filter, org_id: i32, seek_offset: i32) -> String {
    let mut s = "".to_string();

    s += "?seek_offset=";
    s += &seek_offset.to_string();

    format!("/app/team/{}/orbit_data{}", org_id, s)
}

async fn filter_orbit_data(
    transaction: &Transaction<'_>,
    filter: &Filter,
    page_size: i64,
) -> Result<Vec<orbit_data::OrbitData>, CustomError> {
    let seek_offset = if let Some(seek_offset) = filter.seek_offset {
        seek_offset
    } else {
        i32::MAX
    };

    let filter = orbit_data::FilterOrbitDataParams {
        seek_pagination: seek_offset,
        page_size: page_size + 1,
    };

    Ok(orbit_data::filter_orbit_data()
        .params(transaction, &filter)
        .all()
        .await?)
}
