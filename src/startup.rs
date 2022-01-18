use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Responder};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use sqlx::PgPool;
use std::net::TcpListener;

use crate::{
    model::{PlesioSchema, Query, RootValue},
    routes::{health_check, subscribe},
};

async fn index(schema: web::Data<PlesioSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn index_playground() -> impl Responder {
    let source = playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"));
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source)
}

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);

    let graphql_schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(RootValue::new())
        .finish();

    let server = HttpServer::new(move || {
        App::new()
            .route("/", web::post().to(index))
            .route("/", web::get().to(index_playground))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
            .app_data(web::Data::new(graphql_schema.clone()))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
