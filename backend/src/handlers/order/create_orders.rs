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
  schemas::order::create_order::CreateOrderSchema,
};


pub async fn create_orders_handler(
  Path(table_number): Path<i16>,
  State(data): State<Arc<AppState>>,
  Json(body): Json<CreateOrderSchema>,
) -> Result<impl IntoResponse, CustomError> {
  // Validation

  //  If table number is not within 1-100, return error.
  if table_number < 1 || table_number > 100 {
    return Err(CustomError::TableNotFound);
  }
  
  //  If there are no orders or more than 10 orders, return error.
  let body_iter = body.orders.into_iter();
  let order_count = body_iter.len();
  if order_count < 1 {
    return Err(CustomError::AtLastOneOrder);
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

    //  If the string is empty or is more than 255 characters, return error.
    if order.item.is_empty() {
      return Err(CustomError::OrderMissingItem);
    }
    if order.item.graphemes(true).count() > 255 {
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


  // Database request
  let query_result = sqlx::query_as!(
    OrderModel,
    "INSERT INTO orders(table_number, item, cook_time) SELECT * FROM UNNEST($1::smallint[], $2::text[], $3::smallint[]) RETURNING *",
    &table_numbers[..],
    &items[..],
    &cook_times[..]
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