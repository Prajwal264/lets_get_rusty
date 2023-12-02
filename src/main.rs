use std::sync::Mutex;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};

// Create a GET request
#[get("/hello")]
// Create a function that will be involked when the above mentioned route is hit.
async fn hello() -> impl Responder {
    // Response with Ok and a message body
    HttpResponse::Ok().body("Hello World")
}

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

async fn counter_route_handler(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard
    format!("Request number: {counter}") // <- response with count
}

/// extract path info from "/users/{user_id}/{friend}" url
/// {user_id} - deserializes to a u32
/// {friend} - deserializes to a String
#[get("/users/{user_id}/{friend}")] // <- define path parameters
async fn path_extraction_handler(path: web::Path<(u32, String)>) -> Result<String> {
    let (user_id, friend) = path.into_inner();
    Ok(format!("Welcome {}, user_id {}!", friend, user_id))
}

// Called first
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Note: web::Data created _outside_ HttpServer::new closure
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });
    // Create a new Http Server
    HttpServer::new(move || {
        // Create an Application
        App::new()
            .app_data(counter.clone())
            .route("/counter", web::get().to(counter_route_handler))
            .service(hello)
            .service(path_extraction_handler)
    })
    // Bind the server to a port
    .bind(("127.0.0.1", 8080))?
    // run the application
    .run()
    // Suspend the execution until the future is ready
    .await
}
