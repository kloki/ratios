use app::App;
use clap::Parser;
use input::Input;
mod app;
mod input;
use ratatui::{TerminalOptions, Viewport};
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// list of inputs
    inputs: Vec<Input>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    println!(
        "
    ┏┓┏┓╋┓┏┓┏
    ┛ ┗┻┗┗┗┛┛"
    );
    let terminal = ratatui::try_init_with_options(TerminalOptions {
        viewport: Viewport::Inline(3),
    })
    .unwrap();
    let result = App::new(cli.inputs).run(terminal).await;
    ratatui::restore();
    if result.is_err() {
        println!("\nSomething went wrong!")
    }
    println!();
}
