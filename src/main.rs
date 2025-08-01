use clap::Parser;

#[derive(Parser)]
#[command(name = "claudectl")]
#[command(author, version)]
#[command(about = "A command-line tool for managing Claude projects", long_about = None)]
#[command(arg_required_else_help = true)]
struct Cli {}

fn main() {
    let _cli = Cli::parse();
    
    // For now, just parse the CLI
    // The arg_required_else_help will automatically show help when no args are provided
}