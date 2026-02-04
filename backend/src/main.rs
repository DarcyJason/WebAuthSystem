use backend::bootstrap::bootstrap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    bootstrap().await
}
