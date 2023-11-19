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

#[cfg(test)]
mod tests {
  use super::*;
  use axum::{
    extract::{State, Path},
    Json
  };
  use sqlx::{postgres::PgPoolOptions};
  use dotenv::dotenv;
  use crate::errors::CustomError;
  
    #[sqlx::test]
    async fn error_if_table_number_is_out_of_range() {
      dotenv().ok();
      let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
      {
        Ok(pool) => {
          pool
        }
        Err(_) => {
          std::process::exit(1);
        }
      };
      let body_json = r#"{"orders":[{"item":"food","cook_time":1}]}"#;
      let data = std::sync::Arc::new(AppState { db: pool.clone() });
      let body = serde_json::from_str(body_json).unwrap();
      let result = create_orders_handler(Path(101), State(data), Json(body));

      match result.await {
        Err(CustomError::TableNotFound) => assert_eq!(1, 1),
        _ => assert_eq!(1, 2)
      }
        
      //assert_eq!(result, CustomError::TableNotFound);
    }
}