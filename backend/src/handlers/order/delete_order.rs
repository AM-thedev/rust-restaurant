use std::sync::Arc;

use axum::{
  extract::{Path, State},
  http::StatusCode,
  response::IntoResponse,
  Json,
};

use crate::AppState;


pub async fn delete_order_handler(
  Path(id): Path<uuid::Uuid>,
  State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
  let rows_affected = sqlx::query!("DELETE FROM orders WHERE id = $1", id)
    .execute(&data.db)
    .await
    .unwrap()
    .rows_affected();
  
    if rows_affected == 0 {
      let error_response = serde_json::json!({
        "status": "fail",
        "message": format!("Note with ID: {} not found", id)
      });
      return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }
    
    return Ok(StatusCode::NO_CONTENT)
}
