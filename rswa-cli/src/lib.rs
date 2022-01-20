mod generate;
mod serve;

use anyhow::Error;
use clap::Subcommand;

pub use clap::Parser as ArgParser;

#[derive(Debug, ArgParser)]
#[clap(name = "rswa")]
pub struct CLI {
    #[clap(subcommand)]
    pub command: CLISubcommand,
}

#[derive(Debug, Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum CLISubcommand {
    /// Generate values.
    Generate(generate::GenerateCommand),
    /// Launch the Rocket web-service.
    Serve(serve::ServeCommand),
}

impl CLI {
    pub async fn execute(&self) -> Result<(), Error> {
        match &self.command {
            CLISubcommand::Generate(c) => c.execute(&self).await?,
            CLISubcommand::Serve(c) => c.execute(&self).await?,
        }

        Ok(())
    }
}
