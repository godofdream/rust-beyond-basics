#[macro_use]
extern crate serde;

use actix_web::middleware::Logger;
use actix_web::{get, put, web, App, Error, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::RunQueryDsl;
use once_cell::sync::Lazy;
mod offer;
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
async fn get_index() -> impl Responder {
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

/// Handler for the `PUT /product/` route.
///
/// This route expects a JSON payload containing a product object.
/// It inserts the product into the database and returns an empty response body.
///
/// # Arguments
///
/// * `my_product` - JSON payload containing the product object wrapped in `web::Json`.
///
/// # Returns
///
/// Returns an `HttpResponse` indicating the success or failure of the operation.
/// If successful, returns an HTTP 200 OK response with an empty body.
/// If an error occurs, returns an appropriate `Error` response.
#[put("/product/")]
async fn put_product(my_product: web::Json<product::Product>) -> Result<HttpResponse, Error> {
    let mut connection = crate::DATABASE
        .get()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    diesel::insert_into(product::products::table)
        .values(my_product.0)
        .execute(&mut connection)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(""))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(get_index)
            .service(hello)
            .service(get_product)
            .service(put_product)
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}

#[cfg(test)]
mod tests {


    use crate::product::Product;
    use actix_web::App;
    use actix_web::{
        http::{self, header::ContentType},
        test,
    };
    use rand::Rng;

    #[actix_web::test]
    async fn test_index_ok() {
        dotenv::dotenv().ok();
        // Create a test App with the `put_product` handler
        let mut app = test::init_service(App::new().service(crate::get_index)).await;
        let req = test::TestRequest::get().uri("/")
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_put_get_product() {
        dotenv::dotenv().ok();
        // Create a test App with the `put_product` handler
        let mut app = test::init_service(
            App::new()
                .service(crate::put_product)
                .service(crate::get_product),
        )
        .await;
        // create mock product name
        let mut rng = rand::thread_rng();
        let random_id: u32 = rng.gen();
        let product_name = format!("testproduct{random_id}");
        // Create a mock product
        let product = Product {
            product_name: product_name.clone(),
            title: "testtitle".to_string(),
            gtin: Some("3245345345".to_string()),
        };

        // Create a test request with the product as JSON payload
        let req = test::TestRequest::put()
            .uri("/product/")
            .set_json(&product)
            .to_request();

        // Send the request to the app and get the response
        let resp = test::call_service(&mut app, req).await;

        // Assert that the response is successful (HTTP 200 OK)
        assert_eq!(resp.status(), http::StatusCode::OK);

        // Create a test request getting the product
        let req = test::TestRequest::get()
            .uri(&format!("/product/{product_name}"))
            .to_request();

        // Send the request to the app and get the response as json
        let resp: Product = test::call_and_read_body_json(&app, req).await;

        // Assert that the response is the same product
        assert_eq!(resp.product_name, product.product_name);

        // Optionally, you can assert the response body or perform further checks on the database
        // For example:
        //let body = test::read_body(resp).await;
        //assert_eq!(body, actix_web::web::Bytes::from_static(b""));

        // You can also assert any side effects or changes in the database
    }
}
