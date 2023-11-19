use serde::Deserialize;


#[derive(Deserialize, Debug, Default)]
pub struct SearchTablePagination {
  pub page: Option<usize>,
  pub limit: Option<usize>,
}
