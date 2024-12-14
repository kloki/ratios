use app::App;
use clap::Parser;
mod app;
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// list of floats
    inputs: Vec<f64>,
}

use ratatui::{TerminalOptions, Viewport};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let terminal = ratatui::try_init_with_options(TerminalOptions {
        viewport: Viewport::Inline(4),
    })
    .unwrap();
    let result = App::new(cli.inputs).run(terminal).await;
    ratatui::restore();
    if result.is_err() {
        println!("\nSomething went wrong!")
    }
}
