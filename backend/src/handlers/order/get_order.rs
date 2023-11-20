use std::sync::Arc;

use axum::{
  extract::{Path, State},
  response::IntoResponse,
  Json,
};
use serde_json::json;

use crate::{
  AppState,
  models::order::OrderModel,
  errors::CustomError
};


/** Attempts to find a single order with the provided id

  # Arguments
  * `Path(id)` - The order id to be returned, extracted from the url path
  * `State(data)` - A reference to our database

*/
pub async fn get_order_handler(
  Path(id): Path<uuid::Uuid>,
  State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, CustomError> {

  let query_result = sqlx::query_as!(
    OrderModel,
    "SELECT * FROM orders WHERE id = $1",
    id
  )
  .fetch_one(&data.db)
  .await;

  match query_result {
    Ok(order) => {
      let order_response = json!({
        "status": "success",
        "data": json!({
          "order": order
        })
      });

      return Ok(Json(order_response))
    }
    Err(_) => {
      return Err(CustomError::OrderNotFound);
    }
  }
}
