use swc_nodejs_common::MapErr;

#[napi]
async fn run_cli(args: Vec<String>) -> napi::Result<()> {
    let args: fnapi::Args = clap::Parser::parse_from(args);

    args.run().await.convert_err()
}
