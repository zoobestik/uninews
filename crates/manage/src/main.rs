use uninews_manage::cli::run;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    run().await;
}
