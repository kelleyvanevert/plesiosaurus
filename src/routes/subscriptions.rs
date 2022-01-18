use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;

#[derive(serde::Deserialize, Debug)]
pub struct SubscriptionData {
    email: String,
    name: String,
}

// Let's start simple: we always return a 200 OK
pub async fn subscribe(
    data: web::Json<SubscriptionData>,
    db_pool: web::Data<PgPool>,
) -> HttpResponse {
    let res = sqlx::query!(
        r#"
            insert into subscriptions(email, name, subscribed_at)
            values ($1, $2, $3)
        "#,
        data.email,
        data.name,
        Utc::now()
    )
    .execute(db_pool.as_ref())
    .await;

    match res {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
