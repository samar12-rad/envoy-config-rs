use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

use crate::domain::{DomainError, Environment, FeatureFlag};
use crate::repository::InMemoryFlagRepository;
use crate::service::FlagService;

/// Shared state type used by axum handlers.
pub type AppState = Arc<FlagService<InMemoryFlagRepository>>;

#[derive(Debug, Deserialize)]
pub struct CreateFlagRequest {
    pub key: String,
    pub enabled: bool,
    pub environment: Environment,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/flags", post(create_flag))
        .route("/flags/:environment", get(list_flags))
        .with_state(state)
}

async fn create_flag(
    State(service): State<AppState>,
    Json(payload): Json<CreateFlagRequest>,
) -> Result<Json<FeatureFlag>, (StatusCode, Json<ErrorResponse>)> {
    service
        .set_flag(payload.key, payload.enabled, payload.environment)
        .map(Json)
        .map_err(map_domain_error)
}

async fn list_flags(
    Path(env): Path<Environment>,
    State(service): State<AppState>,
) -> Result<Json<Vec<FeatureFlag>>, (StatusCode, Json<ErrorResponse>)> {
    Ok(Json(service.get_flags(env)))
}

fn map_domain_error(err: DomainError) -> (StatusCode, Json<ErrorResponse>) {
    let status = match err {
        DomainError::EmptyKey => StatusCode::BAD_REQUEST,
        DomainError::DuplicateKey { .. } => StatusCode::CONFLICT,
    };

    (status, Json(ErrorResponse { message: err.to_string() }))
}
