fn main() {
    let args: fnapi::Args = clap::Parser::parse();

    args.run().unwrap()
}
