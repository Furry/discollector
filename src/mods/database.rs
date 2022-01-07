use mongodb::{self, options::ClientOptions, Client, bson::doc};

pub struct Database {
    pub client: Client
}

impl Database {
    pub async fn new(uri: &str, db: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client_options = ClientOptions::parse(uri).await?;
        let client = Client::with_options(client_options)?;
        
        client
            .database(db)
            .run_command(doc! {"ping": 1}, None)
            .await?;

        return Ok(Self {
            client
        })
    }
}