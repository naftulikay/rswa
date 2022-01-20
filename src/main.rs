use anyhow::Error;
use rswa::cli::{ArgParser, CLI};

#[rocket::main]
async fn main() -> Result<(), Error> {
    CLI::parse().execute().await
}
