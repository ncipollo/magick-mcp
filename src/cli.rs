use clap::{Parser, Subcommand, ValueEnum};

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
    /// Start the MCP server
    Mcp,
    /// Install magick-mcp to MCP client configuration
    Install {
        /// Client type to install for
        #[arg(long, value_enum, default_value = "both")]
        r#type: ClientTypeArg,
    },
    /// Execute an ImageMagick command
    Magick {
        /// ImageMagick command arguments (e.g., "test.png -negate out.png")
        command: String,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ClientTypeArg {
    Cursor,
    Claude,
    Both,
}

impl From<ClientTypeArg> for crate::ClientType {
    fn from(arg: ClientTypeArg) -> Self {
        match arg {
            ClientTypeArg::Cursor => crate::ClientType::Cursor,
            ClientTypeArg::Claude => crate::ClientType::Claude,
            ClientTypeArg::Both => crate::ClientType::Both,
        }
    }
}

/// Handle command execution
pub fn handle_command(command: Commands) {
    match command {
        Commands::Check => match crate::check() {
            Ok(output) => {
                println!("{output}");
                std::process::exit(0);
            }
            Err(e) => {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        },
        Commands::Mcp => {
            let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
            if let Err(e) = rt.block_on(crate::mcp::run_server()) {
                eprintln!("Error running MCP server: {e}");
                std::process::exit(1);
            }
        }
        Commands::Install { r#type } => {
            let client_type: crate::ClientType = r#type.into();
            let config_paths = match crate::ConfigPaths::from_home_dir() {
                Ok(paths) => paths,
                Err(e) => {
                    eprintln!("Error getting config paths: {e}");
                    std::process::exit(1);
                }
            };
            match crate::install(client_type, config_paths) {
                Ok(_) => {
                    println!("Successfully installed magick-mcp to MCP configuration");
                    std::process::exit(0);
                }
                Err(e) => {
                    eprintln!("Error installing magick-mcp: {e}");
                    std::process::exit(1);
                }
            }
        }
        Commands::Magick { command } => match crate::magick(&command, None) {
            Ok(output) => {
                println!("{output}");
                std::process::exit(0);
            }
            Err(e) => {
                eprintln!("Error executing magick command: {e}");
                std::process::exit(1);
            }
        },
    }
}
