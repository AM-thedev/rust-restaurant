use serde::Deserialize;

/// A struct to contain the pagination options for list table orders
#[derive(Deserialize, Debug, Default)]
pub struct SearchTablePagination {
  pub page: Option<usize>,
  pub limit: Option<usize>,
}
