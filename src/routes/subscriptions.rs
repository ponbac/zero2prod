use axum::{extract::State, http::StatusCode, Form};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domain::{NewSubscriber, SubscriberEmail, SubscriberName},
    startup::AppState,
};

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

impl TryFrom<FormData> for NewSubscriber {
    type Error = String;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name)?;
        let email = SubscriberEmail::parse(value.email)?;

        Ok(Self { name, email })
    }
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form_data, app_state),
    fields(
        email = %form_data.email,
        subscriber_name = %form_data.name
    )
)]
pub async fn subscribe(
    State(app_state): State<AppState>,
    Form(form_data): Form<FormData>,
) -> StatusCode {
    let new_subscriber = match form_data.try_into() {
        Ok(subscriber) => subscriber,
        Err(e) => {
            tracing::error!("Failed to parse subscriber details: {:?}", e);
            return StatusCode::BAD_REQUEST;
        }
    };

    match insert_subscriber(&app_state.db_pool, &new_subscriber).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, pool)
)]
pub async fn insert_subscriber(
    pool: &PgPool,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            Err(e)
        }
    }
}
