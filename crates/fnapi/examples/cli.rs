#[tokio::main]
async fn main() {
    let args: fnapi::Args = clap::Parser::parse();

    args.run().await.unwrap()
}
