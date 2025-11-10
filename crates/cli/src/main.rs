use news_cli::cli;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    cli::run().await;
}
