use uninews_collect::cli;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    cli::run().await;
}
