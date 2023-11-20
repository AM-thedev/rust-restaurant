use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateOrdersSchema {
  pub orders: Vec<SingleOrder>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SingleOrder {
  pub item: String,
  pub cook_time: i16,
}
