use app::App;
use clap::Parser;
use value::Value;
mod app;
mod value;
use ratatui::{TerminalOptions, Viewport};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// examples 0.1 100 0.2:percentage "210:sugar(grams)"
    inputs: Vec<Value>,
}

fn banner() {
    println!(
        "
 ┏┓┏┓╋┓┏┓┏
 ┛ ┗┻┗┗┗┛┛   <q> to leave, <tab> to cycle inputs"
    );
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    if cli.inputs.is_empty() {
        return;
    }
    banner();
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
