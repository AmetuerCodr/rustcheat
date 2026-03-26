#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    open::that("/Users/gadgetguy/rustcheatsheet.jpeg")?;
    Ok(())
}
