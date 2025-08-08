#![deny(warnings)]
use ngrok::config::ForwarderBuilder;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Set up ngrok session
    let session = ngrok::Session::builder()
        .authtoken_from_env()
        .connect()
        .await?;

    let domain = "your-reserved-domain";

    // Forward HTTP traffic from ngrok to the local server
    let _listener = session
        .http_endpoint()
        .domain(domain)
        .traffic_policy(r#"{"on_http_request": [{"actions": [{"type": "oauth","config": {"provider": "google"}}]}]}"#)
        .listen_and_forward(Url::parse("http://localhost:8080").unwrap())
        .await?;

    println!("Ngrok agent endpoint established at {}", domain);

    // Wait indefinitely
    tokio::signal::ctrl_c().await?;
    Ok(())
}
