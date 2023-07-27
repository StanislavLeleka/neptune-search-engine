use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub page: u64,
    pub page_size: u64,
}
