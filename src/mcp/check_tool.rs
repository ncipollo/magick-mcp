use crate::mcp::server::MagickServerHandler;
use rmcp::handler::server::router::tool::ToolRoute;
use rmcp::handler::server::tool::ToolCallContext;
use rmcp::model::{CallToolResult, ErrorData, Tool};
use serde_json::json;

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

/// Create the check tool route
pub fn check_tool_route() -> ToolRoute<MagickServerHandler> {
    let input_schema: serde_json::Value = json!({
        "type": "object",
        "properties": {},
        "required": []
    });
    let tool = Tool::new(
        "check",
        "Check if ImageMagick is installed and return version or installation instructions",
        input_schema.as_object().unwrap().clone(),
    );
    ToolRoute::new_dyn(tool, |context| Box::pin(check_tool(context)))
}
