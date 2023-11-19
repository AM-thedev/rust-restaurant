use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct SearchTableOrders {
  pub table_number: i16,
}
