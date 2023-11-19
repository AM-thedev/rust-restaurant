use axum::{
  http::StatusCode,
  response::{
    Response,
    IntoResponse
  },
  Json
};
use serde_json::json;

#[derive(Debug)]
pub enum CustomError {
  // General
  InternalServerError,

  // Order related
  OrderNotFound,
  OrderMissingItem,
  OrderItemTooLong,
  OrderCookTimeTooLong,
  OrderCookTimeTooShort,
  TooManyOrders,
  AtLastOneOrder,

  // Table related
  TableNotFound
}

impl IntoResponse for CustomError {
  fn into_response(self) -> Response {
    let (status, error_message) = match self {
      // General
      Self::InternalServerError => (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Internal Server Error"
      ),

      // Order related
      Self::OrderNotFound => (
        StatusCode::NOT_FOUND,
        "The order with the provided ID could not be found."
      ),
      Self::OrderMissingItem => (
        StatusCode::BAD_REQUEST,
        "One or more orders are missing an item."
      ),
      Self::OrderItemTooLong => (
        StatusCode::BAD_REQUEST,
        "One or more order items are longer than 255 characters long."
      ),
      Self::OrderCookTimeTooLong => (
        StatusCode::BAD_REQUEST,
        "One or more order cook times are longer than 30 minutes."
      ),
      Self::OrderCookTimeTooShort => (
        StatusCode::BAD_REQUEST,
        "One or more order cook times are shorter than 1 minute."
      ),
      Self::TooManyOrders => (
        StatusCode::BAD_REQUEST,
        "Only up to 10 orders can be submitted with a single request."
      ),
      Self::AtLastOneOrder => (
        StatusCode::BAD_REQUEST,
        "At least 1 order must be submitted with a request."
      ),

      // Table related
      Self::TableNotFound => (
        StatusCode::NOT_FOUND,
        "There is no such table number, please select a table from 1 to 100."
      )
    };
    (status, Json(json!({"error": error_message}))).into_response()
  }
}