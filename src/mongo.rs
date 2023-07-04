use dotenvy::var;
use mongodb::{options::ClientOptions, Client, Database};

pub async fn connect_mongo() -> Database {
    let mongodb_uri = var("MONGODB_URI").expect("MONGODB_URI in .env is required");
    let mut client_options = ClientOptions::parse(mongodb_uri)
        .await
        .expect("Cannot create mongodb options");
    client_options.app_name = Some("sync-module".into());
    let client = Client::with_options(client_options).expect("Cannot connect to MongoDB");
    client.database("sync-module-db")
}
