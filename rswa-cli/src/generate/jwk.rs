use crate::CLI;
use anyhow::Error;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct GenerateJwkCommand {}

impl GenerateJwkCommand {
    pub async fn execute(&self, _root: &CLI) -> Result<(), Error> {
        todo!()
    }
}
