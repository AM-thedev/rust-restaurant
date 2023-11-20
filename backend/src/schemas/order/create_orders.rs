use serde::{Deserialize, Serialize};

/// A vector containing the orders made for a create orders request
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateOrdersSchema {
  pub orders: Vec<SingleOrder>
}

/// A single order contained in a create orders request, the table_number is provided by the url path
#[derive(Serialize, Deserialize, Debug)]
pub struct SingleOrder {
  pub item: String,
  pub cook_time: Option<i16>,
}
