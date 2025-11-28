use rmcp::handler::server::router::tool::ToolRoute;
use rmcp::handler::server::tool::ToolCallContext;
use rmcp::model::ErrorData;
use rmcp::model::{CallToolResult, Tool};
use serde_json::{Map, json};
use std::sync::Arc;

use crate::mcp::server::MagickServerHandler;

/// Check if ImageMagick is installed and return version or installation instructions
async fn check_tool(
    _context: ToolCallContext<'_, MagickServerHandler>,
) -> Result<CallToolResult, ErrorData> {
    match crate::check() {
        Ok(output) => {
            let result = json!({
                "installed": output.contains("Version:"),
                "message": output
            });
            Ok(CallToolResult::structured(result))
        }
        Err(e) => {
            let error_result = json!({
                "error": format!("Check failed: {}", e)
            });
            Ok(CallToolResult::structured_error(error_result))
        }
    }
}

pub fn check_tool_route() -> ToolRoute<MagickServerHandler> {
    let tool = Tool::new(
        "check",
        "Check if ImageMagick is installed and return version or installation instructions",
        Arc::new(Map::new()),
    );
    ToolRoute::new_dyn(tool, |context| Box::pin(check_tool(context)))
}
