use actix_web::{web, App, HttpServer, HttpResponse};
use serde::Deserialize;
use diesel::prelude::*;
use crate::db::establish_connection;
use crate::models::User;

mod db;
mod models;
mod schema;

#[derive(Deserialize)]
struct RegistrationData {
    username: String,
    password: String,
    full_name: String,
    email: String,
    phone_number: i64,
    address: String,
}

aasync fn register_user(data: web::Json<RegistrationData>, pool: web::Data<SqliteConnection>) -> HttpResponse {
    let mut transaction = pool.begin().await.expect("Failed to start transaction");

    let new_user = User {
        id: 0,
        username: data.username.clone(),
        password: data.password.clone(),
        full_name: data.full_name.clone(),
        email: data.email.clone(),
        phone_number: data.phone_number,
        address: data.address.clone(),
    };

    if let Err(e) = diesel::insert_into(schema::users::table)
        .values(&new_user)
        .execute(&transaction)
    {
        eprintln!("Error inserting user: {}", e);
        transaction.rollback().await.expect("Failed to roll back transaction");
        return HttpResponse::InternalServerError().body("Internal Server Error");
    }

    transaction.commit().await.expect("Failed to commit transaction");

    HttpResponse::Ok().body("User registered successfully")
}


// Main Function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(establish_connection())) // Pass the connection pool as app data
            .wrap(actix_web::middleware::Logger::default())
            .service(web::resource("/register").route(web::post().to(register_user)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

