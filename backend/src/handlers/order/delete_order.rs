use std::sync::Arc;

use axum::{
  extract::{Path, State},
  http::StatusCode,
  response::IntoResponse
};

use crate::{
  AppState,
  errors::CustomError
};


pub async fn delete_order_handler(
  Path(id): Path<uuid::Uuid>,
  State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, CustomError> {
  let rows_affected = sqlx::query!("DELETE FROM orders WHERE id = $1", id)
    .execute(&data.db)
    .await
    .unwrap()
    .rows_affected();
  
    if rows_affected == 0 {
      return Err(CustomError::OrderNotFound);
    }
    
    return Ok(StatusCode::NO_CONTENT)
}
