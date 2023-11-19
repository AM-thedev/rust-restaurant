use std::sync::Arc;

use axum::{
  extract::{Query, State, Path},
  response::IntoResponse,
  Json,
};
use serde_json::json;

use crate::{
  errors::CustomError,
  models::order::OrderModel,
  schemas::order::search_table_pagination::SearchTablePagination,
  AppState,
};


pub async fn table_orders_list_handler(
  Path(table_number): Path<i16>,
  opts: Option<Query<SearchTablePagination>>,
  State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, CustomError> {

  // Validation
  if table_number < 1 || table_number > 100 {
    return Err(CustomError::TableNotFound)
  }

  let Query(opts) = opts.unwrap_or_default();

  let limit = opts.limit.unwrap_or(10);
  let offset = (opts.page.unwrap_or(1) - 1) * limit;

  let query_result = sqlx::query_as!(
    OrderModel,
    "SELECT * FROM orders WHERE table_number = $1 ORDER by created_at LIMIT $2 OFFSET $3",
    table_number,
    limit as i32,
    offset as i32
  )
  .fetch_all(&data.db)
  .await;

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
