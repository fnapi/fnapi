#[napi]
async fn run_cli(args: Vec<String>) {
    let args: fnapi::Args = clap::Parser::parse_from(args);

    args.run().await.unwrap()
}
