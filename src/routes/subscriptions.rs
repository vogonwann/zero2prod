use chrono::Utc;
use uuid::Uuid;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
// use sqlx::types::{uuid, chrono};

#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let uuid_v4 = Uuid::new_v4().to_owned();
    let now_utc = Utc::now();
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        uuid_v4.clone(),
        form.email,
        form.name,
        now_utc
    )
        .execute(pool.as_ref())
        .await
        {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(e) => {
                println!("Failed to execute query: {}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
    }
