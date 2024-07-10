mod cornucopia;

pub use cornucopia::queries::admin;
pub use cornucopia::queries::dog;
pub use cornucopia::queries::litter;
pub use cornucopia::queries::news;
pub use cornucopia::queries::puppy;
pub use cornucopia::types::public::{Availability, Breed, Gender};

use serde::Deserialize;
pub use tokio_postgres::Client;
use tokio_postgres::NoTls;

pub async fn connect() -> Result<Client, tokio_postgres::Error> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });

    Ok(client)
}

#[derive(Deserialize)]
#[serde(remote = "Gender")]
pub enum GenderDef {
    Male,
    Female,
}

#[derive(Deserialize)]
#[serde(remote = "Breed")]
pub enum BreedDef {
    Australian,
    Entlebuch,
    Bernese,
}

#[derive(Deserialize)]
#[serde(remote = "Availability")]
pub enum AvailabilityDef {
    Available,
    Unavailable,
    Co,
}
