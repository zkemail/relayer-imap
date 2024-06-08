use anyhow::Result;
use relayer_imap::run;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    run().await
}
