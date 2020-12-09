use dnsl::{entry, Result};

#[tokio::main]
async fn main() -> Result<()> {
    entry().await
}
