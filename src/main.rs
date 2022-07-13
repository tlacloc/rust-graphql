#[macro_use]
extern crate diesel;
extern crate juniper;

use std::io;
use std::sync::Arc;

use dotenv::dotenv;
use std::env;

use actix_web::{web, App, Error, HttpResponse, HttpServer};
use futures::future::Future;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

mod graphql_schema;
mod schema;

use crate::graphql_schema::{create_schema, Schema};

fn main() -> io::Result<()> {
    dotenv().ok();


    let port = env::var("PORT").expect("PORT must be set");
    let port : u16 = port.parse().unwrap();


    let schema = std::sync::Arc::new(create_schema());
    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(web::resource("/graphql").route(web::post().to_async(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .bind(("0.0.0.0", port))?
    .run()
}

fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let res = data.execute(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .map_err(Error::from)
    .and_then(|user| {
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(user))
    })
}

fn graphiql() -> HttpResponse {
    dotenv().ok();

    let port = env::var("PORT").expect("PORT must be set");
    let port : u16 = port.parse().unwrap();

    let html = graphiql_source(&format!("http://localhost:{}/graphql", port));
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
