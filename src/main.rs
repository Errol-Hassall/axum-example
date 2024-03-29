use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        .route("/cars", post(create_car));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Root"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

async fn create_car(
    Json(payload): Json<CreateCar>,
) ->(StatusCode, Json<Car>) {
    let car = Car {
        id: 1,
        price: payload.price,
        model: Model {
            id: 1,
            name: payload.model.name
        }
    };

    (StatusCode::CREATED, Json(car))
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

#[derive(Deserialize)]
struct CreateCar {
    price: f64,
    model: CreateModel
}

#[derive(Serialize)]
struct Car {
    id: u64,
    price: f64,
    model: Model
}

#[derive(Deserialize)]
struct CreateModel {
    name: String
}

#[derive(Serialize)]
struct Model {
    id: u64,
    name: String
}