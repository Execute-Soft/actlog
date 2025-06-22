mod app;
mod cli;

use app::App;
use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();
    App::run(cli);
}
