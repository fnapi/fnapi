mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Compile(commands::BuildCommand),
    Serve(commands::ServeCommand),
}

fn main() {
    let _args = Args::parse();
}
