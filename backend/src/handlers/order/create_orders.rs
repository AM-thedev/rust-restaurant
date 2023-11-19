use std::sync::Arc;

use axum::{
  extract::{State, Path},
  http::StatusCode,
  response::IntoResponse,
  Json,
};
/*
use sqlx::{
  Postgres,
  QueryBuilder,
  //PgRow
};
*/
use serde_json::json;

use crate::{
  models::order::OrderModel,
  schemas::order::create_order::CreateOrderSchema,
  AppState,
};


pub async fn create_orders_handler(
  Path(table_number): Path<i16>,
  State(data): State<Arc<AppState>>,
  Json(body): Json<Vec<CreateOrderSchema>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
  /*
  #[derive(Serialize, Deserialize)]
  #[serde(remote = "PgRow")]
  struct PgRowDef {
      secs: i64,
      nanos: i32,
  }

  //let orders: Vec<CreateOrderSchema> = serde_json::from_string(body);
  let orders = body.into_iter();
  const BIND_LIMIT: usize = 65535;
  let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("INSERT INTO orders(table_number, item, cook_time) ");

  query_builder.push_values(orders.take(BIND_LIMIT / 4), |mut b, order| {
    b.push_bind(table_number)
      .push_bind(order.item)
      .push_bind(order.cook_time);
  });

  let query_result = sqlx::query_as!(
    OrderModel,
    query_builder
  )
    .fetch_all(&data.db)
    .await;
*/
  let mut table_numbers: Vec<i16> = Vec::new();
  let mut items: Vec<String> = Vec::new();
  let mut cook_times: Vec<i16> = Vec::new();

  for order in body.into_iter() {
    table_numbers.push(table_number);
    items.push(order.item);
    cook_times.push(order.cook_time);
  }

  let query_result = sqlx::query_as!(
    OrderModel,
    "INSERT INTO orders(table_number, item, cook_time) SELECT * FROM UNNEST($1::smallint[], $2::text[], $3::smallint[]) RETURNING *",
    &table_numbers[..],
    &items[..],
    &cook_times[..]
  )
  .fetch_all(&data.db)
  .await;


  match query_result {
    Ok(_) => {
      let orders_result = query_result.unwrap();
      let json_response = serde_json::json!({
        "status": "success",
        "results": orders_result.len(),
        "orders": orders_result
      });

      return Ok((StatusCode::CREATED, Json(json_response)));
    }
    Err(err) => {
      return Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({"status": "error", "message": format!("{:?}", err)})),
      ));
    }
  }
}
