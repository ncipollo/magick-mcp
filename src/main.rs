use clap::Parser;

/// Magick MCP - A Model Context Protocol server
#[derive(Parser, Debug)]
#[command(name = "magick-mcp")]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Args {}

fn main() {
    let _args = Args::parse();
}
