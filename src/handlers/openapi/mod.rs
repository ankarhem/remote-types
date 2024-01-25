use crate::{models::PkgJson, prelude::*};
use anyhow::{anyhow, Context, Result};
use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncReadExt};
use tracing::instrument;
use utoipa::IntoParams;

use crate::AppState;

#[derive(Debug, Serialize, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct OpenApiParams {
    url: String,
}

/// Generate from OpenAPI
///
/// Generate a npm package containing typescript types a openapi schema
#[utoipa::path(
    get,
    path = "/openapi",
    operation_id = "generate_from_openapi",
    tag = "OpenAPI",
    params(OpenApiParams)
)]
#[instrument(name = "Generate from OpenAPI", skip(state))]
pub async fn get(
    State(state): State<AppState>,
    Query(query): Query<OpenApiParams>,
) -> Result<impl IntoResponse, AppError> {
    let spec_url = query.url;
    let response = state
        .client
        .get(&spec_url)
        .send()
        .await
        .with_context(|| format!("Failed to get OpenAPI spec from: `{spec_url}`"))?;

    if response.status() != StatusCode::OK {
        return Err(anyhow!("Spec url returned non OK status: `{}`", response.status()).into());
    }

    // let spec = response
    //     .json::<OpenApi>()
    //     .await
    //     .context("Could not parse json from openapi spec")?;

    let mut file = File::open("./templates/package.json").await?;
    let mut data = String::new();
    file.read_to_string(&mut data).await?;
    let mut pkg_json: PkgJson = serde_json::from_str(&data)?;
    pkg_json.name = "Test";

    let st = serde_json::to_string(&pkg_json)?;
    dbg!(st);

    Ok(())
}
