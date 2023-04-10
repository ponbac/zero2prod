use axum::{extract::State, response::Html, Form};

use crate::startup::AppState;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(
    State(app_state): State<AppState>,
    Form(form_data): Form<FormData>,
) -> Html<String> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        uuid::Uuid::new_v4(),
        form_data.email,
        form_data.name,
        chrono::Utc::now()
    )
    .execute(app_state.db_pool.as_ref())
    .await
    .expect("Failed to execute query.");

    let response = format!(
        "<h1>Welcome, {}! We saved your email: {}</h1>",
        form_data.name, form_data.email
    );

    Html(response)
}
