mod commands;
mod runner;

use crate::cli::runner::run;

pub async fn main() -> anyhow::Result<()> {
    run().await?;
    Ok(())
}
