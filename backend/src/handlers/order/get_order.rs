use std::sync::Arc;

use axum::{
  extract::{Path, State},
  http::StatusCode,
  response::IntoResponse,
  Json,
};

use crate::{
  models::order::OrderModel,
  AppState,
};


pub async fn get_order_handler(
  Path(id): Path<uuid::Uuid>,
  State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
  let query_result = sqlx::query_as!(
    OrderModel,
    "SELECT * FROM orders WHERE id = $1",
    id
  )
  .fetch_one(&data.db)
  .await;

  match query_result {
    Ok(order) => {
      let order_response = serde_json::json!({
        "status": "success",
        "data": serde_json::json!({
          "order": order
        })
      });

      return Ok(Json(order_response))
    }
    Err(_) => {
      let error_response = serde_json::json!({
        "status": "fail",
        "message": format!("Order with ID: {} not found", id)
      });
      return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }
  }
}
