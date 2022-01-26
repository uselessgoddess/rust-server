#![feature(entry_insert)]

mod model;

use actix_web::web::Data;
use actix_web::{guard, web, App, HttpResponse, HttpServer, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{Context, EmptyMutation, EmptySubscription};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use async_std::sync::{Arc, RwLock};
use std::collections::HashMap;
use std::error::Error;

type Schema = async_graphql::Schema<model::Query, model::Mutation, EmptySubscription>;
pub type DbCtx = Arc<RwLock<HashMap<String, Vec<String>>>>;

async fn index(schema: web::Data<Schema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn index_playground() -> Result<HttpResponse> {
    let source =
        playground_source(GraphQLPlaygroundConfig::new("/query").subscription_endpoint("/query"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(model::Query, model::Mutation, EmptySubscription)
        .data::<DbCtx>(Arc::new(RwLock::new(HashMap::new())))
        .finish();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/query").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .bind("127.0.0.1:1234")?
    .run()
    .await
}
