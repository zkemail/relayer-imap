pub mod config;
pub mod imap_client;
pub mod strings;
pub use crate::config::RelayerIMAPConfig;
use config::*;
use imap_client::*;
use strings::*;

use anyhow::Result;
use relayer_utils::LOG;
use slog::{error, info, trace};

pub async fn run(config: RelayerIMAPConfig) -> Result<()> {
    let mut imap_client = ImapClient::new(config.imap_config).await?;
    let client = reqwest::Client::new();
    let relayer_endpoint = config.relayer_endpoint;

    loop {
        match imap_client.retrieve_new_emails().await {
            Ok(emails) => {
                // Process emails
                for email in emails {
                    for fetch in email.iter() {
                        if let Some(body) = fetch.body() {
                            let body = String::from_utf8(body.to_vec())?;
                            info!(LOG, "Email body: {}", body);
                            // Send a POST request to the relayer endpoint with the email body
                            let res = client.post(&relayer_endpoint).body(body).send().await;
                            match res {
                                Ok(response) => {
                                    if response.status().is_success() {
                                        trace!(
                                            LOG,
                                            "Email body successfully sent to the relayer endpoint"
                                        );
                                    } else {
                                        error!(LOG, "Failed to send email body to the relayer endpoint; HTTP status: {}", response.status());
                                    }
                                }
                                Err(e) => error!(
                                    LOG,
                                    "Failed to send email body to the relayer endpoint; Error: {}",
                                    e
                                ),
                            }
                        }
                    }
                }
            }
            Err(e) => {
                error!(LOG, "Error retrieving emails: {}", e);
                trace!(LOG, "Reconnecting...");
                imap_client.reconnect().await?;
                trace!(LOG, "Reconnected!");
            }
        }

        // Wait before the next iteration to avoid overwhelming the server and to handle errors gracefully
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
