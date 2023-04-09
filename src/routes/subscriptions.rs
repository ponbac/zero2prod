use axum::Form;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(Form(form_data): Form<FormData>) -> String {
    format!(
        "Welcome, {}! We saved your email: {}",
        form_data.name, form_data.email
    )
}
