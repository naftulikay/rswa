use anyhow::Error;
use clap::Parser;

use crate::CLI;

#[derive(Debug, Parser)]
pub struct ServeCommand {}

impl ServeCommand {
    pub async fn execute(&self, _root: &CLI) -> Result<(), Error> {
        todo!()
    }
}
