use serde::Serialize;

use super::models::Movie;

#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub movies: Vec<Movie>,
}
