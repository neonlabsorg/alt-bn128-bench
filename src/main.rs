//! alt-bn128-bench main module

#![deny(warnings)]
#![deny(missing_docs)]

mod alt_bn128;
mod benchmark;
mod cli;
mod significant;

fn main() {
    init_logger();
    execute(cli::application());
}

/// Initializes the logger
fn init_logger() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    tracing_subscriber::fmt::init();
}

/// Dispatches CLI commands
fn execute(app: cli::Application) {
    benchmark::run(app.count, app.size, app.bench);
}
