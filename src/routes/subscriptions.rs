use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize, Debug)]
pub struct SubscriptionData {
    email: String,
    name: String,
}

// Let's start simple: we always return a 200 OK
pub async fn subscribe(data: web::Json<SubscriptionData>) -> HttpResponse {
    println!("{:?}", data);

    HttpResponse::Ok().finish()
}
