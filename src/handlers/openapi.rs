use crate::prelude::*;
use anyhow::{Context, Result};
use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::instrument;
use utoipa::{openapi::OpenApi, IntoParams};

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
    let response = state
        .client
        .get(query.url)
        .send()
        .await
        .with_context(|| "Failed to get OpenAPI spec from {query.url}")?;

    let spec = response
        .json::<OpenApi>()
        .await
        .context("Could not parse json from openapi spec")?;

    Ok(())
}
