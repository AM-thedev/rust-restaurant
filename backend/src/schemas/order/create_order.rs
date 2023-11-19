use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateOrderSchema {
  //pub table_number: i16,
  pub item: String,
  pub cook_time: i16,
}
