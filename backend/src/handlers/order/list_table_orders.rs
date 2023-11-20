use std::sync::Arc;

use axum::{
  extract::{State, Path},
  response::IntoResponse,
  Json,
};
use serde_json::json;

use crate::{
  errors::CustomError,
  models::order::OrderModel,
  AppState,
};


/** If successful it will return all orders with the specified table number, or a CustomError if it fails.

  The UNNEST function is unique to PostgreSQL and is the more optimized choice for multi-insert scenarios.
  
  # Arguments
  * `Path(table_number)` - The table number extracted from the url path
  * `State(data)` - A reference to our database

*/
pub async fn table_orders_list_handler(
  Path(table_number): Path<i16>,
  State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, CustomError> {

  // Validation
  let validated_table_number = validate(table_number).unwrap();

  // Database request
  let query_result = sqlx::query_as!(
      OrderModel,
      "SELECT * FROM orders WHERE table_number = $1 ORDER by created_at",
      validated_table_number
    )
    .fetch_all(&data.db)
    .await;

  // Request result
  if query_result.is_err() {
    return Err(CustomError::InternalServerError);
  }

  let orders = query_result.unwrap();

  let json_response = json!({
    "status": "success",
    "results": orders.len(),
    "orders": orders
  });
  return Ok(Json(json_response))
}


/** Returns the table number if validation passes or returns a CustomError if the requested table number is not within 1-100

  # Arguments
  * `table_number` - The table number extracted from the url path

*/
pub fn validate(table_number: i16) -> Result<i16, CustomError> {

  if table_number < 1 || table_number > 100 {
    return Err(CustomError::TableNotFound);
  }
  Ok(table_number)
}


#[cfg(test)]
mod tests {
  use super::*;
  
    #[test]
    fn error_if_table_number_is_too_small() {
      let less_than_one = validate(0).unwrap_err();
        
      match less_than_one {
        CustomError::TableNotFound => assert!(true),
        _ => assert!(false)
      }
    }

    #[test]
    fn error_if_table_number_is_too_big() {
      let more_than_one_hundred = validate(101).unwrap_err();
        
      match more_than_one_hundred {
        CustomError::TableNotFound => assert!(true),
        _ => assert!(false)
      }
    }

    #[test]
    fn success_if_table_number_is_between_one_and_a_hundred() {
      let table_number: i16 = 50;
      let just_right = validate(50).unwrap();
      
      assert_eq!(table_number, just_right)
    }
}
