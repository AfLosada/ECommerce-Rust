mod entities;
mod integration;
mod shared;

use async_graphql::{
    dynamic::Schema,
    http::{GraphQLPlaygroundConfig, playground_source},
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    Router,
    extract::State,
    http::StatusCode,
    response::{self, IntoResponse, Response},
    routing::get,
};
use dotenv::dotenv;
use sea_orm::Database;
use seaography::{async_graphql, lazy_static::lazy_static};
use std::env;
use tokio::net::TcpListener;

lazy_static! {
    static ref URL: String = env::var("URL").unwrap_or("localhost:8000".into());
    static ref ENDPOINT: String = env::var("ENDPOINT").unwrap_or("/".into());
    static ref DATABASE_URL: String =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
    static ref DEPTH_LIMIT: Option<usize> = env::var("DEPTH_LIMIT").map_or(None, |data| Some(
        data.parse().expect("DEPTH_LIMIT is not a number")
    ));
    static ref COMPLEXITY_LIMIT: Option<usize> = env::var("COMPLEXITY_LIMIT")
        .map_or(None, |data| {
            Some(data.parse().expect("COMPLEXITY_LIMIT is not a number"))
        });
}

async fn graphql_playground() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new(&*ENDPOINT)))
}

async fn graphql_handler(State(schema): State<Schema>, req: GraphQLRequest) -> GraphQLResponse {
    let req = req.into_inner();
    schema.execute(req).await.into()
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_test_writer()
        .init();
    let db = Database::connect(&*DATABASE_URL)
        .await
        .expect("Fail to initialize database connection");
    db.get_schema_registry("auth::entities::*")
        .sync(&db)
        .await
        .expect("Fail to generate schema registry");

    let schema = auth::query_root::schema(db, *DEPTH_LIMIT, *COMPLEXITY_LIMIT).unwrap();
    let app = Router::new()
        .route(&*ENDPOINT, get(graphql_playground).post(graphql_handler))
        .with_state(schema);
    println!("Visit GraphQL Playground at http://{}", *URL);
    axum::serve(TcpListener::bind(&*URL).await.unwrap(), app)
        .await
        .unwrap();
}
