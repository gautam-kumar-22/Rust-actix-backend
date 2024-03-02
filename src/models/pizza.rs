// Pizza models
use serde::{Serialize, Deserialize};
use validator::Validate;
// use crate::validator::Validate;

#[derive(Validate, Deserialize, Serialize)]
pub struct BuyPizzaRequest {
    #[validate(length(min=1, message="Pizza name is required."))]
    pub pizza_name: String,
}

#[derive(Validate, Deserialize, Serialize)]
pub struct UpdatePizzaURL {
    pub uuid: String,
}

#[derive(Validate, Deserialize, Serialize, Debug)]
pub struct Pizza {
    pub uuid: String,
    pub pizza_name: String,
}

impl Pizza {
    pub fn new(uuid: String, pizza_name: String) -> Pizza {
        Pizza {
            uuid,
            pizza_name
        }
    }
}

