use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    axum2::run().await?;
    Ok(())
}
