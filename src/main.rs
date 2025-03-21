use std::num::ParseIntError;

use axum::{Json, Router, extract::Path, http::StatusCode, response::IntoResponse, routing::get};
use serde::Serialize;

//TODO: respond with a message as `text/plain`
async fn ex1() -> ??? {
    todo!();
}

#[derive(Serialize)]
struct Ex2Response {
    param: String,
}

//TODO: extract an argument, `param`, from the path and return
//it with the above struct as a json response
async fn ex2(???) -> ??? {
    todo!()
}

//TODO: extract an argument, `tea_time`, from the path and try
//to parse it into an i32
// - if you can convert it into a number AND the value is between 1 and 12, return `HTTP 200`.
// - if it's not a valid number, return an `HTTP 400` with a text message,
//   describing the issue UNLESS the argument is "coffee", then return `HTTP 418`
// - if the number is negative, return an `HTTP 400` with a text message,
//   describing the issue.
async fn ex3(Path(tea_time): Path<String>) -> impl IntoResponse {
    let tea_time_as_num: Result<i32, ParseIntError> = tea_time.parse::<i32>();

    todo!()
}

async fn ex4() -> StatusCode {
    let client = reqwest::Client::new();
    //TODO: Make an HTTP request
    todo!();

    //TODO: Return `HTTP 200` if the status code form your request was `200`,
    //otherwise, return `HTTP 500`. No response body necessary, just the status code.
    todo!()

    //BONUS: Creating a new HTTP client every request is kind of wasteful. Anyway we can avoid that?
    //Will require changes to this handler, as well as the Router in `main()`
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/ex1", get(ex1))
        .route("/ex2/{param}", get(ex2))
        .route("/ex3/{tea_time}", get(ex3))
        .route("/ex4", get(ex4));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
            .await
            .unwrap();
        axum::serve(listener, app).await.unwrap();
}
