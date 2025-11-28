use clap::Parser;
use magick_mcp::cli;

fn main() {
    let args = cli::Args::parse();
    cli::handle_command(args.command);
}
