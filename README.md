# IMAP Relayer

This project is an IMAP client that retrieves new emails and forwards them to a specified relayer endpoint.

## Features

- Connects to an IMAP server using either password or OAuth2 authentication
- Retrieves unseen emails from the inbox
- Forwards email bodies to a specified relayer endpoint
- Automatic reconnection on connection loss

## Prerequisites

- Rust (latest stable version)
- Cargo (Rust's package manager)

## Setup

1. Clone the repository:

   ```
   git clone <repository-url>
   cd relayer-imap
   ```

2. Create a `.env` file in the project root and add the following environment variables:

   ```
   RELAYER_ENDPOINT=<your-relayer-endpoint>
   IMAP_LOGIN_ID=<your-email>
   IMAP_DOMAIN_NAME=<imap-server-domain>
   IMAP_PORT=<imap-server-port>
   AUTH_TYPE=<password or oauth>
   IMAP_LOGIN_PASSWORD=<your-password> # If using password authentication

   # If using OAuth2 authentication, add these:
   IMAP_CLIENT_ID=<your-oauth-client-id>
   IMAP_CLIENT_SECRET=<your-oauth-client-secret>
   IMAP_AUTH_URL=<oauth-auth-url>
   IMAP_TOKEN_URL=<oauth-token-url>
   IMAP_REDIRECT_URL=<oauth-redirect-url>
   ```

## Usage

To run the project:

```
cargo run
```

This will start the IMAP client, which will continuously check for new emails and forward them to the specified relayer endpoint.

## Configuration

The project uses environment variables for configuration. You can modify these in the `.env` file or set them directly in your environment.

Key configuration options:

- `AUTH_TYPE`: Set to "password" for password-based authentication or "oauth" for OAuth2 authentication.
- `RELAYER_ENDPOINT`: The endpoint where email bodies will be forwarded.

For more details on configuration options, see the `config.rs` file:

```
rust:src/config.rs
startLine: 37
endLine: 57
```

# Error Handling

The project includes robust error handling and automatic reconnection in case of connection issues. If you encounter persistent problems, check the logs for more information.
