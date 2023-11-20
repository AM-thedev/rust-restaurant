mod routes;
mod errors;
mod models {
  pub mod order;
}
mod schemas {
  pub mod order {
    pub mod create_orders;
    pub mod search_table_orders;
    pub mod search_table_pagination;
  }
}
mod handlers {
  pub mod health {
    pub mod get_health_check;
  }
  pub mod order {
    pub mod list_table_orders;
    pub mod get_order;
    pub mod create_orders;
    pub mod delete_order;
  }
}

use std::sync::Arc;

use axum::http::{
  header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
  HeaderValue,
  Method,
};
use dotenv::dotenv;
use routes::create_router;
use tower_http::cors::CorsLayer;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};


pub struct AppState {
  db: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
  dotenv().ok();

  let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let pool = match PgPoolOptions::new()
    .max_connections(10)
    .connect(&database_url)
    .await
  {
    Ok(pool) => {
      println!("Connection to database successful!");
      pool
    }
    Err(err) => {
      println!("Failed to connect to the database: {:?}", err);
      std::process::exit(1);
    }
  };

  let cors = CorsLayer::new()
    .allow_origin("http://localhost::3000".parse::<HeaderValue>().unwrap())
    .allow_methods([Method::GET, Method::POST, Method::DELETE])
    .allow_credentials(true)
    .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

  let app = create_router(Arc::new(AppState { db: pool.clone() })).layer(cors);

  println!("Server started successfully, rocket emoji.");
  axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
  .serve(app.into_make_service())
  .await
  .unwrap();
}