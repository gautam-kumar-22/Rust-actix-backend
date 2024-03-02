use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};
use crate::models::pizza::Pizza;


#[derive(Clone)]
pub struct Database {
    pub client: Surreal<Client>,
    pub name_space: String,
    pub db_name: String
}


impl Database {
    pub async fn init() -> Result<Self, Error> {
        let client =  Surreal::new::<Ws>("127.0.0.1:8000").await?;
        client.signin(Root {
            username: "root",
            password: "root"
        })
        .await?;
        client.use_ns("surreal").use_db("pizzas").await.unwrap();
        Ok(Database {
            client,
            name_space: String::from("surreal"),
            db_name: String::from("pizzas")
        })
    }

    // pub async fn get_all_pizzas(&self) -> Option<Vec<Pizza>> {
    //     let result = self.client.select("pizza").await;
    //     match result {
    //         Ok(all_pizzas) => Some(all_pizzas),
    //         Err(_) => None
    //     }
    // }

    // pub async fn add_pizza(&self, new_pizza: Pizza) -> Option<Pizza> {
    //     let created_pizza = self
    //                             .client
    //                             .create(("pizza", new_pizza.uuid.clone()))
    //                             .content(new_pizza)
    //                             .await;
    //     match created_pizza {
    //         Ok(created) => created,
    //         Err(_) => None,
    //     }
                
    // }

    // pub async fn update_pizza(&self, uuid: String) -> Option<Pizza> {
    //     let find_pizza: Result<Option<Pizza>, Error> = self.client.select(("pizza", &uuid)).await;

    //     match find_pizza {
    //         Ok(found) => {
    //             match found {
    //                 Some(_found_pizza) => {
    //                     let updated_pizza: Result<Option<Pizza>, Error> = self
    //                         .client
    //                         .update(("pizza", &uuid))
    //                         .merge(Pizza {
    //                             uuid,
    //                             pizza_name: String::from("Sold")
    //                         })
    //                         .await;
    //                     match updated_pizza {
    //                         Ok(updated) => updated,
    //                         Err(_) => None,
    //                     }
    //                 },
    //                 None => None,
    //             }
    //         },
    //         Err(_) => None,
    //     }
    // }

}
