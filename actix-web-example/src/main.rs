#[macro_use]
extern crate serde;

use actix_web::middleware::Logger;
use actix_web::{get, put, web, App, Error, HttpResponse, HttpServer, Responder};
use diesel::{prelude::*};
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::RunQueryDsl;
use once_cell::sync::Lazy;
mod product;

/// We lazily initialize the database and make it reachable globally
/// The reason we don't use actix-webs 'app_data()' is because it adds an ARC
/// the diesel_async deadpool Pool is already multiprocessing capable
static DATABASE: Lazy<Pool<diesel_async::AsyncPgConnection>> = Lazy::new(|| {
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(
        std::env::var("DATABASE_URL").expect("database config"),
    );
    let database: Pool<diesel_async::AsyncPgConnection> = Pool::builder(config)
        .max_size(60)
        .build()
        .expect("database");
    database
});

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/hello/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

#[get("/product/{product_name}")]
async fn get_product(product_name: web::Path<String>) -> Result<HttpResponse, Error> {
    let mut connection = crate::DATABASE
        .get()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let product_name = product_name.into_inner();
    let result = product::products::table
        .find(&product_name)
        .load::<product::Product>(&mut connection)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?
        .into_iter()
        .next();
    Ok(HttpResponse::Ok().json(result))
}

#[put("/product/")]
async fn put_product(my_product: web::Json<product::Product>) -> Result<HttpResponse, Error> {
    let mut connection = crate::DATABASE
        .get()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    diesel::insert_into(product::products::table)
             .values(my_product.0)
             .execute(&mut connection)
             .await.map_err(actix_web::error::ErrorInternalServerError)?;
            Ok(HttpResponse::Ok().body(""))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(index)
            .service(hello)
            .service(get_product)
            .service(put_product)
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}
