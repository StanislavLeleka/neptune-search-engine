use std::env;

use mongodb::{
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};

// MongoDB connection string environment variable name
const MONGO_CONN_STR: &str = "MONGO_CONN_STR";

pub async fn connect() -> mongodb::error::Result<Client> {
    let uri = env::var(MONGO_CONN_STR).expect(&*format!("${} is not set", MONGO_CONN_STR));

    log::debug!("Connecting to MongoDB with URI: {}", uri);

    let mut client_options = ClientOptions::parse(uri).await?;
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();

    client_options.server_api = Some(server_api);

    let client = Client::with_options(client_options)?;

    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    log::info!("Successfully connected to MongoDB");

    Ok(client)
}
