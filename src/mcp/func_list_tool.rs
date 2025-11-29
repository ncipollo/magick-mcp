use crate::mcp::server::MagickServerHandler;
use rmcp::handler::server::router::tool::ToolRoute;
use rmcp::handler::server::tool::ToolCallContext;
use rmcp::model::{CallToolResult, ErrorData, Tool};
use serde_json::json;

/// List all available magick functions
async fn func_list_tool(
    _context: ToolCallContext<'_, MagickServerHandler>,
) -> Result<CallToolResult, ErrorData> {
    match crate::list_functions() {
        Ok(functions) => {
            let result = json!({
                "functions": functions,
                "count": functions.len()
            });
            Ok(CallToolResult::structured(result))
        }
        Err(e) => {
            let error_result = json!({
                "error": format!("Failed to list functions: {}", e)
            });
            Ok(CallToolResult::structured_error(error_result))
        }
    }
}

/// Create the func_list tool route
pub fn func_list_tool_route() -> ToolRoute<MagickServerHandler> {
    let input_schema: serde_json::Value = json!({
        "type": "object",
        "properties": {},
        "required": []
    });
    let tool = Tool::new(
        "func_list",
        "List all available magick functions",
        input_schema.as_object().unwrap().clone(),
    );
    ToolRoute::new_dyn(tool, |context| Box::pin(func_list_tool(context)))
}
