pub mod check_tool;
pub mod help_resource;
pub mod magick_tool;
pub mod server;

use crate::mcp::check_tool::check_tool_route;
use crate::mcp::magick_tool::magick_tool_route;
use rmcp::handler::server::router::Router;
use rmcp::service::ServiceExt;
use rmcp::transport::io::stdio;
use server::MagickServerHandler;

/// Run the MCP server over stdio
pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    let handler = MagickServerHandler;
    let router = Router::new(handler)
        .with_tool(check_tool_route())
        .with_tool(magick_tool_route());

    // Create stdio transport
    let (stdin, stdout) = stdio();

    // Serve over stdio
    let running_service = router.serve((stdin, stdout)).await?;

    // Wait for the service to complete
    running_service.waiting().await?;

    Ok(())
}
