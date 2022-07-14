#[macro_use]
extern crate diesel;

use std::{io, sync::Arc};

use dotenv::dotenv;
use std::env;

use actix_cors::Cors;
use actix_web::{
    get, middleware, route,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use actix_web_lab::respond::Html;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};

mod db;
mod graphql_schema;
mod schema;

use crate::db::establish_connection;
use crate::graphql_schema::{create_schema, Context, Schema};

/// GraphiQL playground UI
#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

/// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(
    st: web::Data<Schema>,
    ctx: web::Data<Context>,
    data: web::Json<GraphQLRequest>,
) -> impl Responder {
    let user = data.execute(&st, &ctx).await;
    HttpResponse::Ok().json(user)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    dotenv().ok();

    let port = env::var("PORT").expect("PORT must be set");
    let port: u16 = port.parse().unwrap();

    let pool = establish_connection();

    // Create Juniper schema
    let schema_context = Context { db: pool.clone() };
    let schema = Arc::new(create_schema());

    log::info!("starting HTTP server on port {}", port);
    log::info!("GraphiQL playground: http://localhost:{}/graphiql", port);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::from(schema.clone()))
            .app_data(Data::from(schema_context.clone()))
            .service(graphql)
            .service(graphql_playground)
            // the graphiql UI requires CORS to be enabled
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
