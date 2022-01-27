use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

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

#[derive(Serialize, Deserialize)]
struct Subscription {
    id: Uuid,
    email: String,
    name: String,
    subscribed_at: chrono::DateTime<Utc>,
}

pub async fn list_subscriptions(db_pool: web::Data<PgPool>) -> impl Responder {
    let res = sqlx::query_as!(
        Subscription,
        r#"
            select s.*
            from subscriptions as s
        "#
    )
    .fetch_all(db_pool.as_ref())
    .await;

    match res {
        Ok(rows) => web::Json(rows),
        Err(e) => {
            panic!("Failed to execute query: {}", e);
            // HttpResponse::InternalServerError().finish()
        }
    }
    // Ok(web::Json(rows))
}

#[derive(Serialize, Deserialize)]
struct SubscriptionList {
    list: Vec<Subscription>,
}

pub async fn list_subscriptions_fancy(db_pool: web::Data<PgPool>) -> impl Responder {
    let res = sqlx::query!(
        r#"
            select coalesce(json_agg(s.*), '[]'::json) as list
            from subscriptions as s
        "#
    )
    .fetch_all(db_pool.as_ref())
    .await
    .map(|rows| {
        rows.into_iter()
            .map(|row| SubscriptionList {
                list: serde_json::from_value::<Vec<Subscription>>(row.list.unwrap()).unwrap(),
            })
            .collect::<Vec<SubscriptionList>>()
    });

    match res {
        Ok(rows) => web::Json(rows),
        Err(e) => {
            panic!("Failed to execute query: {}", e);
            // HttpResponse::InternalServerError().finish()
        }
    }
    // Ok(web::Json(rows))
}
