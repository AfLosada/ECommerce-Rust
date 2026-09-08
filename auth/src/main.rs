use async_graphql::{
    dynamic::Schema,
    http::{GraphQLPlaygroundConfig, playground_source},
};
use axum::{
    Json, Router,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
};

use sea_orm::{Database, DatabaseConnection, Schema};
use seaography::async_graphql::{
    Data,
    http::{GraphQLPlaygroundConfig, playground_source},
};
use serde::{Deserialize, Serialize};

pub mod model;

use model::user::user;

#[handler]
async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(&ENDPOINT)))
}

#[handler]
async fn graphql_handler(schema: Data, req: GraphQLRequest) -> GraphQLResponse {
    let req = req.0;
    schema.execute(req).await.into()
}

#[tokio::main]
async fn main() {
    let db = Database::connect("sqlite://auth.db?mode=rwc")
        .await
        .expect("Fail to initialize database connection");
    // synchronizes database schema with entity definitions
    db.get_schema_builder().register(user::Entity);
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await;
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
