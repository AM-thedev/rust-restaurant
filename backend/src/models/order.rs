use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct OrderModel {
  pub id: Uuid,
  #[serde(rename = "tableNumber")]
  pub table_number: i16,
  pub item: String,
  #[serde(rename = "cookTime")]
  pub cook_time: i16,
  #[serde(rename = "createdAt")]
  pub created_at: Option<chrono::DateTime<chrono::Utc>>
}
