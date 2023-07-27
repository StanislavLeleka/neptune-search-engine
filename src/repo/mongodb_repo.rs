use mongodb::{bson::doc, options::FindOptions, Collection};

use crate::{
    config,
    models::{models::Movie, response::SearchResult},
};

pub struct MongoRepo {
    movies: Collection<Movie>,
}

impl MongoRepo {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        log::info!("Initializing MongoDB repository");

        let client = config::database::connect().await?;
        log::info!("Connected to the MongoDB database");

        let db = client.database("metadata");
        log::info!("Selected 'metadata' database");

        let col: Collection<Movie> = db.collection("movies_metadata");
        log::info!("Selected 'movies_metadata' collection");

        Ok(MongoRepo { movies: col })
    }

    pub async fn search_movie(
        &self,
        query: String,
        page: u64,
        page_size: u64,
    ) -> Result<SearchResult, Box<dyn std::error::Error>> {
        let query_filter = doc! {"original_title": {"$regex": &query, "$options": "i"}};
        let find_options = FindOptions::builder()
            .skip(page * page_size)
            .limit(page_size as i64)
            .sort(doc! {"vote_count": -1})
            .build();
        let mut cursor = self.movies.find(query_filter, find_options).await?;
        let mut movies = vec![];

        while cursor.advance().await? {
            let movie = cursor.deserialize_current()?;
            movies.push(movie);
        }

        Ok(SearchResult { movies })
    }
}
