use actix_web::{web::Path, get, post, patch, Responder, App, HttpServer, HttpResponse, web::Json};
mod models;
mod db;
use actix_web::web::Data;
use crate::models::pizza::{BuyPizzaRequest, UpdatePizzaURL, Pizza};
// use crate::db::Database;
use crate::db::{pizza_data_trait::PizzaDataTrait, Database};
use validator::Validate;
use uuid::Uuid;
mod error;
use error::PizzaError;


#[get("/pizzas")]
async fn get_pizzas(db: Data<Database>) -> Result<Json<Vec<Pizza>>, PizzaError> {
    let pizzas = Database::get_all_pizzas(&db).await;
    match pizzas {
        // Some(found_pizzas) => HttpResponse::Ok().body(format!("{:?}", found_pizzas)),
        // None => HttpResponse::Ok().body("Error"),
        Some(found_pizzas) => Ok(Json(found_pizzas)),
        None => Err(PizzaError::NoPizzasFound),
    }
}

#[post("/buypizza")]
async fn buy_pizza(body: Json<BuyPizzaRequest>, db: Data<Database>) -> Result<Json<Pizza>, PizzaError> {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let pizza_name = body.pizza_name.clone();
            let mut buffer = Uuid::encode_buffer();
            let new_uuid = Uuid::new_v4().simple().encode_lower(&mut buffer);
            let new_pizza = Database::add_pizza(&db,
                Pizza::new(
                    String::from(new_uuid),
                    pizza_name
                )
            ).await;
            match new_pizza {
                Some(created) => {
                    Ok(Json(created))
                },
                None => Err(PizzaError::PizzaCreationFailure,)
            }
            // return HttpResponse::Ok().body(format!("Entered pizza name is: {pizza_name}"));
        },
        Err(_) => {
            Err(PizzaError::PizzaCreationFailure)
        }
    }
}

#[patch("/updatepizza/{uuid}")]
async fn update_pizza(update_pizza_url: Path<UpdatePizzaURL>, db: Data<Database>) -> Result<Json<Pizza>, PizzaError> {
    let uuid = update_pizza_url.into_inner().uuid;
    let update_result = Database::update_pizza(&db, uuid).await;
    match update_result {
        Some(updated_pizza) => Ok(Json(updated_pizza)),
        None => Err(PizzaError::NoSuchPizzaFound),

    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    println!("Server running at 127.0.0.1:8080");

    let db = Database::init()
        .await
        .expect("Error connecting to database");
    let db_data = Data::new(db);

    
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(get_pizzas)
            .service(buy_pizza)
            .service(update_pizza)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
