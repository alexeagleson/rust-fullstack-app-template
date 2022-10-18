use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

mod types;
use types::Person;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/people", get(get_people))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn get_people() -> impl IntoResponse {
    let people = vec![
        Person {
            name: String::from("Person A"),
            age: 36,
            favourite_food: Some(String::from("Pizza")),
        },
        Person {
            name: String::from("Person B"),
            age: 5,
            favourite_food: Some(String::from("Broccoli")),
        },
        Person {
            name: String::from("Person C"),
            age: 100,
            favourite_food: None,
        },
    ];

    (StatusCode::OK, Json(people))
}
