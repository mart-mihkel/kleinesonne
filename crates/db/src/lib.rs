mod cornucopia;

use std::env;

use serde::Deserialize;
use tokio_postgres::{Config, NoTls};

pub use tokio_postgres::Client;

pub use cornucopia::queries::admin;
pub use cornucopia::queries::dog;
pub use cornucopia::queries::litter;
pub use cornucopia::queries::news;
pub use cornucopia::queries::puppy;
pub use cornucopia::types::public::{Availability, Breed, Gender};

pub async fn connect() -> Result<Client, tokio_postgres::Error> {
    let (client, connection) = config().connect(NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });

    Ok(client)
}

fn config() -> Config {
    let user = env::var("POSTGRES_USER").expect("POSTGRES_USER undefined");
    let password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD undefined");

    let mut conf = Config::new();
    conf.host("db")
        .dbname(user.as_str())
        .user(user.as_str())
        .password(password.as_str());

    conf
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
