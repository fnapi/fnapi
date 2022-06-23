use clap::Parser;

/// Start a development server
#[derive(Parser, Debug)]
pub(crate) struct ServeCommand {
    #[clap(long, short = 'p', default_value = "4321")]
    port: usize,
}
