use backend::run;
use snafu::{ResultExt, Whatever};

#[tokio::main]
async fn main() -> Result<(), Whatever> {
    run()
        .await
        .with_whatever_context(|e| format!("App run failed: {}", e.to_string()))
}
