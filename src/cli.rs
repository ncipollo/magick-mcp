use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

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
    /// Manage magick functions
    Func {
        #[command(subcommand)]
        func_command: FuncCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum FuncCommands {
    /// List all available functions
    List,
    /// Print a function in human-readable format
    Print {
        /// Name of the function to print
        name: String,
    },
    /// Execute a function by name
    Execute {
        /// Name of the function to execute
        name: String,
        /// Input value to replace $input placeholders in commands
        #[arg(long)]
        input: Option<String>,
    },
    /// Save a function from a JSON file
    Save {
        /// Path to the JSON file containing the function
        #[arg(long)]
        file: PathBuf,
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
        Commands::Func { func_command } => handle_func_command(func_command),
    }
}

/// Handle function subcommand execution
fn handle_func_command(func_command: FuncCommands) {
    match func_command {
        FuncCommands::List => match crate::list_functions() {
            Ok(functions) => {
                if functions.is_empty() {
                    println!("No functions found");
                } else {
                    for name in functions {
                        println!("{name}");
                    }
                }
                std::process::exit(0);
            }
            Err(e) => {
                eprintln!("Error listing functions: {e}");
                std::process::exit(1);
            }
        },
        FuncCommands::Print { name } => match crate::load_function(&name) {
            Ok(function) => {
                println!("Name: {}", function.name);
                println!("Commands:");
                for command in &function.commands {
                    println!("  - {command}");
                }
                std::process::exit(0);
            }
            Err(e) => {
                eprintln!("Error loading function '{name}': {e}");
                std::process::exit(1);
            }
        },
        FuncCommands::Execute { name, input } => {
            let function = match crate::load_function(&name) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("Error loading function '{name}': {e}");
                    std::process::exit(1);
                }
            };
            let input_ref = input.as_deref();
            match crate::run_function(&function, None, input_ref) {
                Ok(outputs) => {
                    for output in outputs {
                        println!("{output}");
                    }
                    std::process::exit(0);
                }
                Err(e) => {
                    eprintln!("Error executing function '{name}': {e}");
                    std::process::exit(1);
                }
            }
        }
        FuncCommands::Save { file } => {
            let contents = match std::fs::read_to_string(&file) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error reading file '{}': {e}", file.display());
                    std::process::exit(1);
                }
            };
            let function: crate::Function = match serde_json::from_str(&contents) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("Error parsing JSON from '{}': {e}", file.display());
                    std::process::exit(1);
                }
            };
            match crate::save_function(function) {
                Ok(_) => {
                    println!("Function saved successfully");
                    std::process::exit(0);
                }
                Err(e) => {
                    eprintln!("Error saving function: {e}");
                    std::process::exit(1);
                }
            }
        }
    }
}
