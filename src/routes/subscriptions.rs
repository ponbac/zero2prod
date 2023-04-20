use axum::{extract::State, Form};
use chrono::Utc;
use reqwest::StatusCode;
use uuid::Uuid;

use crate::startup::AppState;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(
    State(app_state): State<AppState>,
    Form(form_data): Form<FormData>,
) -> StatusCode {
    tracing::info!(
        "Adding new subscriber, {} <{}>",
        form_data.name,
        form_data.email
    );

    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form_data.email,
        form_data.name,
        Utc::now()
    )
    .execute(app_state.db_pool.as_ref())
    .await
    {
        Ok(_) => {
            tracing::info!("Successfully added new subscriber");
            StatusCode::OK
        }
        Err(e) => {
            tracing::error!("Failed to execute query. Error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
