use std::sync::Arc;

use axum::{
  extract::{State, Path},
  http::StatusCode,
  response::IntoResponse,
  Json,
};
use unicode_segmentation::UnicodeSegmentation;
use serde_json::json;

use crate::{
  AppState,
  errors::CustomError,
  models::order::OrderModel,
  schemas::order::create_orders::CreateOrdersSchema,
};


#[derive(Debug, PartialEq)]
pub struct ValidatedOrders {
  pub table_numbers: Vec<i16>,
  pub items: Vec<String>,
  pub cook_times: Vec<i16>
}


pub async fn create_orders_handler(
  Path(table_number): Path<i16>,
  State(data): State<Arc<AppState>>,
  Json(body): Json<CreateOrdersSchema>,
) -> Result<impl IntoResponse, CustomError> {
  // Validation
  let validated_body = validate(table_number, body).unwrap();

  // Database request
  let query_result = sqlx::query_as!(
    OrderModel,
    "INSERT INTO orders(table_number, item, cook_time) SELECT * FROM UNNEST($1::smallint[], $2::text[], $3::smallint[]) RETURNING *",
    &validated_body.table_numbers[..],
    &validated_body.items[..],
    &validated_body.cook_times[..]
  )
  .fetch_all(&data.db)
  .await;


  // Request result
  match query_result {
    Ok(_) => {
      let orders_result = query_result.unwrap();
      let json_response = json!({
        "status": "success",
        "results": orders_result.len(),
        "orders": orders_result
      });

      return Ok((StatusCode::CREATED, Json(json_response)));
    }
    Err(_) => {
      return Err(CustomError::InternalServerError);
    }
  }
}


pub fn validate(table_number: i16, body: CreateOrdersSchema) -> Result<ValidatedOrders, CustomError> {
  
  //  If table number is not within 1-100, return error.
  if table_number < 1 || table_number > 100 {
    return Err(CustomError::TableNotFound);
  }
  
  //  If there are no orders or more than 10 orders, return error.
  let body_iter = body.orders.into_iter();
  let order_count = body_iter.len();
  if order_count < 1 {
    return Err(CustomError::TooFewOrders);
  }
  if order_count > 10 {
    return Err(CustomError::TooManyOrders);
  }

  //  Prepare arrays to contain transformed data for PostgreSQL UNNEST request.
  let mut table_numbers: Vec<i16> = Vec::new();
  let mut items: Vec<String> = Vec::new();
  let mut cook_times: Vec<i16> = Vec::new();

  //  Iterate through the request body.
  for order in body_iter {
    //  Make an array for the table number the same length as the other arrays.
    table_numbers.push(table_number);

    //  If the string is empty or is more than 100 characters, return error.
    if order.item.is_empty() {
      return Err(CustomError::OrderItemEmpty);
    }
    if order.item.graphemes(true).count() > 100 {
      return Err(CustomError::OrderItemTooLong);
    }
    items.push(order.item);

    //  If the cooking time is not within 1-30 minutes, return error.
    if order.cook_time < 1 {
      return Err(CustomError::OrderCookTimeTooShort);
    }
    if order.cook_time > 30 {
      return Err(CustomError::OrderCookTimeTooLong);
    }
    cook_times.push(order.cook_time);
  }

  Ok(ValidatedOrders{table_numbers, items, cook_times})
}


#[cfg(test)]
mod tests {
  use super::*;
  
    #[test]
    fn error_if_table_number_is_too_small() {
      let test_json = r#"{"orders":[{"item":"food","cook_time":1}]}"#;
      let test_body = serde_json::from_str(test_json).unwrap();
      let less_than_one = validate(0, test_body).unwrap_err();
        
      match less_than_one {
        CustomError::TableNotFound => assert!(true),
        _ => assert!(false)
      }
    }

    #[test]
    fn error_if_table_number_is_too_big() {
      let test_json = r#"{"orders":[{"item":"food","cook_time":1}]}"#;
      let test_body = serde_json::from_str(test_json).unwrap();
      let more_than_one_hundred = validate(101, test_body).unwrap_err();
        
      match more_than_one_hundred {
        CustomError::TableNotFound => assert!(true),
        _ => assert!(false)
      }
    }

    #[test]
    fn error_if_no_orders() {
      let test_json = r#"{"orders":[]}"#;
      let test_body = serde_json::from_str(test_json).unwrap();
      let table_number: i16 = 50;
      let result = validate(table_number, test_body).unwrap_err();

      match result {
        CustomError::TooFewOrders => assert!(true),
        _ => assert!(false)
      }
    }

    #[test]
    fn error_if_too_many_orders() {
      let test_json = r#"{"orders":[
        {"item":"food","cook_time":1}, {"item":"food","cook_time":1}, {"item":"food","cook_time":1},
        {"item":"food","cook_time":1}, {"item":"food","cook_time":1}, {"item":"food","cook_time":1},
        {"item":"food","cook_time":1}, {"item":"food","cook_time":1}, {"item":"food","cook_time":1},
        {"item":"food","cook_time":1}, {"item":"food","cook_time":1}
      ]}"#;
      let test_body = serde_json::from_str(test_json).unwrap();
      let table_number: i16 = 50;
      let result = validate(table_number, test_body).unwrap_err();

      match result {
        CustomError::TooManyOrders => assert!(true),
        _ => assert!(false)
      }
    }

    #[test]
    fn error_if_order_item_empty() {
      let test_json = r#"{"orders":[{"item":"","cook_time":1}]}"#;
      let test_body = serde_json::from_str(test_json).unwrap();
      let table_number: i16 = 50;
      let result = validate(table_number, test_body).unwrap_err();

      match result {
        CustomError::OrderItemEmpty => assert!(true),
        _ => assert!(false)
      }
    }

    #[test]
    fn error_if_order_item_too_long() {
      // The following "item" is 104 characters long.
      let test_json = r#"{"orders":[{
        "item":"ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ",
        "cook_time":1}]}"#;
      let test_body = serde_json::from_str(test_json).unwrap();
      let table_number: i16 = 50;
      let result = validate(table_number, test_body).unwrap_err();

      match result {
        CustomError::OrderItemTooLong => assert!(true),
        _ => assert!(false)
      }
    }

    #[test]
    fn error_if_order_cook_time_too_short() {
      let test_json = r#"{"orders":[{"item":"food","cook_time":0}]}"#;
      let test_body = serde_json::from_str(test_json).unwrap();
      let table_number: i16 = 50;
      let result = validate(table_number, test_body).unwrap_err();

      match result {
        CustomError::OrderCookTimeTooShort => assert!(true),
        _ => assert!(false)
      }
    }

    #[test]
    fn error_if_order_cook_time_too_long() {
      let test_json = r#"{"orders":[{"item":"food","cook_time":31}]}"#;
      let test_body = serde_json::from_str(test_json).unwrap();
      let table_number: i16 = 50;
      let result = validate(table_number, test_body).unwrap_err();

      match result {
        CustomError::OrderCookTimeTooLong => assert!(true),
        _ => assert!(false)
      }
    }

    #[test]
    fn success_return_transformed_data_if_all_validations_pass() {
      let test_json = r#"{"orders":[
        {"item":"food","cook_time":1},
          {
            "item":"ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUV",
            "cook_time":30
          }
        ]}"#;
      let test_body = serde_json::from_str(test_json).unwrap();
      let table_number: i16 = 50;
      let just_right = validate(table_number, test_body).unwrap();

      let table_numbers: Vec<i16> = [table_number, table_number].to_vec();
      let items: Vec<String> = [
        String::from("food"),
        String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUV"),
        ].to_vec();
      let cook_times: Vec<i16> = [1, 30].to_vec();
      let validated_data = ValidatedOrders{table_numbers, items, cook_times};

      assert_eq!(validated_data, just_right)
    }
}
