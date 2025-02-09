use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::AppState;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct User {
    pub id: i32,
}

#[utoipa::path(
    get,
    path = "/health",
    tags = ["common"],
    summary = "Get health",
    description = "Get health of the service",
    responses(
        (status = 200, body=[User]),
    )
    )]
pub async fn get_health() -> Json<Vec<User>> {
    Json(vec![User { id: 1 }])
}
