use axum::{
    extract::Path,
    http::StatusCode,
    routing::{get, post},
    Form, Router,
};

pub fn app() -> Router {
    Router::new()
        .route("/", get(handler))
        .route("/health_check", get(health_check))
        .route("/hello/:name", get(hello))
        .route("/subscriptions", post(subscribe))
}

async fn handler() -> &'static str {
    "Hello, world!"
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

async fn hello(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}

#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String,
}

async fn subscribe(Form(form_data): Form<FormData>) -> StatusCode {
    match (form_data.name.is_empty(), form_data.email.is_empty()) {
        (true, true) => return StatusCode::BAD_REQUEST,
        (true, false) => {
            println!("Missing name");
            return StatusCode::BAD_REQUEST;
        }
        (false, true) => {
            println!("Missing email");
            return StatusCode::BAD_REQUEST;
        }
        (false, false) => (),
    }

    print!(
        "Welcome, {}! We saved your email: {}",
        form_data.name, form_data.email
    );

    StatusCode::OK
}
