//! alt-bn128-bench command line interface definition

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "ALT BN128 benchmark")]
pub struct Application {
    #[structopt(short, long, help = "Number of rounds", default_value = "10000")]
    pub count: usize,

    #[structopt(
        short,
        long,
        help = "Size of random input buffer in bytes",
        default_value = "10000"
    )]
    pub size: usize,

    #[structopt(subcommand)]
    pub bench: Option<Bench>,
}

#[derive(StructOpt, Eq, PartialEq)]
pub enum Bench {
    #[structopt(name = "add")]
    Addition,
    #[structopt(name = "mul")]
    Multiplication,
    #[structopt(name = "pair")]
    Pairing,
}

/// Constructs an instance of the Application.
pub fn application() -> Application {
    Application::from_args()
}
