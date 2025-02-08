use routes::common_routes::common_routes;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

mod routes {
    pub mod common_routes;
}
mod controllers {
    pub mod common_controller;
}

#[tokio::main]
async fn main() {
    let (router, api) = OpenApiRouter::new()
        .nest("/api/common", common_routes())
        .split_for_parts();

    let router = router.merge(SwaggerUi::new("/swagger").url("/swagger.json", api));

    let app = router.into_make_service();

    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(tcp_listener, app).await.unwrap();
}
