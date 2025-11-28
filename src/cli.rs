use clap::{Parser, Subcommand};

/// Magick MCP - A Model Context Protocol server
#[derive(Parser, Debug)]
#[command(name = "magick-mcp")]
#[command(version = env!("CARGO_PKG_VERSION"))]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Check if ImageMagick is installed
    Check,
}

/// Handle command execution
pub fn handle_command(command: Commands) {
    match command {
        Commands::Check => match crate::check() {
            Ok(output) => {
                println!("{}", output);
                std::process::exit(0);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        },
    }
}
