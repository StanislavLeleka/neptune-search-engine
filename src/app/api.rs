use actix_web::{web, HttpResponse};

use crate::{models::request::SearchQuery, repo::mongodb_repo::MongoRepo};

pub async fn search(
    db: web::Data<MongoRepo>,
    search_query: web::Json<SearchQuery>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    log::info!(
        "search_movie called with query: {}, page: {}, page_size: {}",
        search_query.q.clone(),
        search_query.page,
        search_query.page_size
    );

    let search_result = db
        .search_movie(
            search_query.q.clone(),
            search_query.page,
            search_query.page_size,
        )
        .await?;

    log::info!("search_movie completed successfully");

    Ok(HttpResponse::Ok().json(search_result))
}
