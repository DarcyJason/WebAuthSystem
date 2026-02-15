use backend::bootstrap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    bootstrap().await
}
