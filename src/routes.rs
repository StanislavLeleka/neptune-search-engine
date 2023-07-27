use actix_web::web::{self, post, ServiceConfig};

use crate::app;

pub fn api(config: &mut ServiceConfig) {
    config.service(
        web::scope("/api").service(web::scope("/search").route("", post().to(app::api::search))),
    );
}
