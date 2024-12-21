use anyhow::Result;
use relayer_imap::run;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let config = relayer_imap::config::RelayerIMAPConfig::new();
    run(config).await
}
