use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::extract::{Extension, Path, Query};
use axum::response::Html;
use db::queries::{conjunctions, organisations};
use db::{Params, Pool, Transaction};
use serde::Deserialize;

static PAGE_SIZE: usize = 20;

#[derive(Deserialize, Debug)]
pub struct Filter {
    seek_offset: Option<i32>,
    pc_greater: Option<String>,
    pc_less: Option<String>,
    second_object: Option<String>,
    first_object: Option<String>,
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

    let conjunctions = filter_conjuctions(&transaction, &filter, PAGE_SIZE as i64).await?;

    let org = organisations::organisation()
        .bind(&transaction, &team_id)
        .one()
        .await?;

    let next_page_url = if conjunctions.len() > PAGE_SIZE {
        Some(generate_url(&filter, org.id, conjunctions[PAGE_SIZE].id))
    } else {
        None
    };

    Ok(Html(ui_components::conjunctions::conjunctions(
        conjunctions,
        next_page_url,
        org.id,
    )))
}

fn generate_url(filter: &Filter, org_id: i32, seek_offset: i32) -> String {
    let mut s = "".to_string();

    s += "?seek_offset=";
    s += &seek_offset.to_string();

    if let Some(pc_less) = &filter.pc_less {
        s += "&pc_less=";
        s += pc_less;
    }

    if let Some(pc_greater) = &filter.pc_greater {
        s += "&pc_greater=";
        s += pc_greater;
    }

    format!("/app/team/{}/conjunctions{}", org_id, s)
}

async fn filter_conjuctions(
    transaction: &Transaction<'_>,
    filter: &Filter,
    page_size: i64,
) -> Result<Vec<conjunctions::Conjunction>, CustomError> {
    let mut pc_greater = None;
    if let Some(pc_gr) = &filter.pc_greater {
        if let Ok(pc_g) = pc_gr.parse::<f64>() {
            pc_greater = Some(pc_g);
        }
    }

    let mut pc_less = None;
    if let Some(pc_ls) = &filter.pc_less {
        if let Ok(pc_l) = pc_ls.parse::<f64>() {
            pc_less = Some(pc_l);
        }
    }

    let mut first_object = None;
    if let Some(fo) = &filter.first_object {
        if !fo.is_empty() {
            first_object = Some(fo.as_ref());
        }
    }

    let mut second_object = None;
    if let Some(so) = &filter.second_object {
        if !so.is_empty() {
            second_object = Some(so.as_ref());
        }
    }

    let seek_offset = if let Some(seek_offset) = filter.seek_offset {
        seek_offset
    } else {
        i32::MAX
    };

    let filter = conjunctions::FilterConjunctionsParams {
        pc_greater,
        pc_less,
        object1_name_like: first_object,
        object2_name_like: second_object,
        seek_pagination: seek_offset,
        page_size: page_size + 1,
    };

    Ok(conjunctions::filter_conjunctions()
        .params(transaction, &filter)
        .all()
        .await?)
}
