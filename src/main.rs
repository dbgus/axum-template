use std::{env, sync::Arc};

use dotenvy;
use routes::common_routes::common_routes;
use sea_orm::{Database, DatabaseConnection};
use tower_http::add_extension::AddExtensionLayer;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

mod routes {
    pub mod common_routes;
}
mod controllers {
    pub mod common_controller;
}

#[derive(Clone)]
struct AppState {
    database: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv()
        .ok()
        .expect("env file not found. make sure your env");

    let database: DatabaseConnection =
        Database::connect(env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
            .await
            .expect("Database connection failed");

    let state = AppState { database };

    let (router, api) = OpenApiRouter::new()
        .nest("/api/common", common_routes())
        .split_for_parts();

    let router = router
        .merge(SwaggerUi::new("/swagger").url("/swagger.json", api))
        .layer(AddExtensionLayer::new(state));

    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(tcp_listener, router.into_make_service())
        .await
        .unwrap();
}
