use std::sync::Arc;

use axum::{
  routing::{get},
  Router,
};

use crate::{
  handlers::{
    health::get_health_check::health_check_handler,
    order::{
      list_table_orders::table_orders_list_handler,
      get_order::get_order_handler,
      create_orders::create_orders_handler,
      delete_order::delete_order_handler,
    }
  },
  AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
  Router::new()
    .route("/api/healthcheck", get(health_check_handler))
    .route(
      "/api/tables/:table_number",
      get(table_orders_list_handler)
        .post(create_orders_handler)
    )
    .route(
      "/api/orders/:id",
      get(get_order_handler)
        .delete(delete_order_handler)
    )
    .with_state(app_state)
}
