use axum::{
    http::{HeaderValue, Method},
    routing::{get, get_service},
    Router,
};
use dotenv::dotenv;
use objserve::{collections, errors, objs};
use sqlx::sqlite::SqlitePool;
use std::{env, net::SocketAddr};
use tower_http::{cors::CorsLayer, services::ServeDir};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv()?;
    tracing_subscriber::fmt::init();

    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
    let serve_dir =
        get_service(ServeDir::new(&env::var("OBJ_PATH")?)).handle_error(errors::handle_error);

    let app = Router::new()
        .route("/collections", get(collections::routes::collections))
        .route(
            "/collections/:id/objs",
            get(collections::routes::collection_objs),
        )
        .route("/objs/:id", get(objs::routes::obj))
        .route("/objs", get(objs::routes::objs))
        .nest_service("/files", serve_dir.clone())
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_methods(vec![Method::GET, Method::POST]),
        )
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));

    tracing::info!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
