mod jwk;

use crate::CLI;
use anyhow::Error;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct GenerateCommand {
    #[clap(subcommand)]
    command: GenerateSubcommand,
}

#[derive(Debug, Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum GenerateSubcommand {
    Jwk(jwk::GenerateJwkCommand),
}

impl GenerateCommand {
    pub async fn execute(&self, root: &CLI) -> Result<(), Error> {
        match &self.command {
            GenerateSubcommand::Jwk(s) => s.execute(root).await?,
        }

        Ok(())
    }
}
