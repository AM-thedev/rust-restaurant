use axum::{
  response::IntoResponse,
  Json,
};


pub async fn health_check_handler() -> impl IntoResponse {
  const MESSAGE: &str = "Get a table's orders at api/tables/TABLE_NUMBER";

  let json_response = serde_json::json!({
    "status": "success",
    "message": MESSAGE
  });

  Json(json_response)
}