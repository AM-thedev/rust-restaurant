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
  Json(body): Json<Vec<CreateOrderSchema>>,
) -> Result<impl IntoResponse, CustomError> {

  // Validation
  if table_number < 1 || table_number > 100 {
    return Err(CustomError::TableNotFound)
  }
  
  let order_count = body.len();
  if order_count < 1 {
    return Err(CustomError::AtLastOneOrder)
  }
  if order_count > 10 {
    return Err(CustomError::TooManyOrders)
  }

  let mut table_numbers: Vec<i16> = Vec::new();
  let mut items: Vec<String> = Vec::new();
  let mut cook_times: Vec<i16> = Vec::new();

  for order in body.into_iter() {
    table_numbers.push(table_number);

    if order.item.is_empty() {
      return Err(CustomError::OrderMissingItem);
    }
    if order.item.graphemes(true).count() > 255 {
      return Err(CustomError::OrderItemTooLong);
    }
    items.push(order.item);

    if order.cook_time < 1 {
      return Err(CustomError::OrderCookTimeTooShort);
    }
    if order.cook_time > 30 {
      return Err(CustomError::OrderCookTimeTooLong);
    }
    cook_times.push(order.cook_time);
  }



  let query_result = sqlx::query_as!(
    OrderModel,
    "INSERT INTO orders(table_number, item, cook_time) SELECT * FROM UNNEST($1::smallint[], $2::text[], $3::smallint[]) RETURNING *",
    &table_numbers[..],
    &items[..],
    &cook_times[..]
  )
  .fetch_all(&data.db)
  .await;


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
