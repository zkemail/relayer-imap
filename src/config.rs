use crate::*;

use std::env;

use dotenv::dotenv;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelayerIMAPConfig {
    pub imap_config: ImapConfig,
    pub relayer_endpoint: String,
}

unsafe impl Send for RelayerIMAPConfig {}
unsafe impl Sync for RelayerIMAPConfig {}

impl RelayerIMAPConfig {
    pub fn new() -> Self {
        dotenv().ok();

        RelayerIMAPConfig {
            imap_config: ImapConfig::new(),
            relayer_endpoint: env::var(RELAYER_ENDPOINT_KEY).unwrap(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ImapAuth {
    Password {
        user_id: String,
        password: String,
    },
    Oauth {
        user_id: String,
        client_id: String,
        client_secret: String,
        auth_url: String,
        token_url: String,
        redirect_url: String,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImapConfig {
    pub domain_name: String,
    pub port: u16,
    pub auth: ImapAuth,
    pub initially_checked: bool,
}

pub struct OAuth2 {
    pub user_id: String,
    pub access_token: String,
}

impl ImapConfig {
    pub fn new() -> Self {
        dotenv().ok();

        let imap_auth = env::var(IMAP_AUTH_TYPE_KEY).unwrap();
        let imap_auth = match &*imap_auth {
            "password" => ImapAuth::Password {
                user_id: env::var(IMAP_LOGIN_ID_KEY).unwrap(),
                password: env::var(IMAP_LOGIN_PASSWORD_KEY).unwrap(),
            },
            // TODO: Implement OAuth authentication
            _ => panic!("{WRONG_AUTH_METHOD}"),
        };

        ImapConfig {
            domain_name: env::var(IMAP_DOMAIN_NAME_KEY).unwrap(),
            port: env::var(IMAP_PORT_KEY).unwrap().parse().unwrap(),
            auth: imap_auth,
            initially_checked: false,
        }
    }
}
