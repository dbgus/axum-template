use axum::extract::State;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn common_routes() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(crate::controllers::common_controller::get_health))
}
