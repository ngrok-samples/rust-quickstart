# ngrok Rust SDK Quickstart

A minimal Rust app demonstrating the ngrok Rust SDK.

## What you'll need

- An [ngrok account](https://dashboard.ngrok.com/signup).
- Your [ngrok auth token](https://dashboard.ngrok.com/get-started/your-authtoken).
- [Rust installed](https://www.rust-lang.org/tools/install) on your machine.

## Setup

1. Build the project (this will download dependencies):
   ```bash
   cargo build
   ```

2. Create a `.env` file from the example:
   ```bash
   cp .env.example .env
   ```

3. Add your ngrok auth token to the `.env` file:
   ```txt
   NGROK_AUTHTOKEN=your_actual_authtoken_here
   ```

4. (Optional) Reserve a domain in the [ngrok dashboard](https://dashboard.ngrok.com/domains) and update the `domain` variable in `src/example.rs`.

## Running the app

1. Start the Rust service:
   ```bash
   cargo run --bin service
   ```

2. In another terminal, start the ngrok agent endpoint:
   ```bash
   NGROK_AUTHTOKEN=your_token_here cargo run --bin example
   ```
   
   Or if you created a `.env` file:
   ```bash
   cargo run --bin example
   ```

The ngrok agent endpoint will output a URL that forwards traffic to your local app. If you configured OAuth, visitors will need to log in with Google to access it.

## Files

- `src/service.rs` - Basic Rust HTTP server
- `src/example.rs` - ngrok agent endpoint configuration with OAuth
- `.env.example` - Environment variable template
- `Cargo.toml` - Rust dependencies and binary configuration
