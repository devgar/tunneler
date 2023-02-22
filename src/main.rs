type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let client = reqwest::Client::new();
    let auth = format!(
        "Bearer {}",
        dotenvy::var("API_TOKEN")?
    );
    let body = client
        .get("https://api.ngrok.com/tunnels")
        .header(reqwest::header::AUTHORIZATION, auth)
        .header("ngrok-version", "2")
        .send()
        .await?
        .text()
        .await?;

    println!("{:?}", body);

    Ok(())
}
