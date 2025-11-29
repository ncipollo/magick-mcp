use crate::mcp::server::MagickServerHandler;
use rmcp::handler::server::router::tool::ToolRoute;
use rmcp::handler::server::tool::ToolCallContext;
use rmcp::model::{CallToolResult, ErrorCode, ErrorData, Tool};
use serde_json::json;
use std::path::Path;

/// Execute a magick function by name
async fn func_execute_tool(
    context: ToolCallContext<'_, MagickServerHandler>,
) -> Result<CallToolResult, ErrorData> {
    // Extract name parameter from context
    let name = context
        .arguments
        .as_ref()
        .and_then(|args| args.get("name"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| ErrorData {
            code: ErrorCode::INVALID_PARAMS,
            message: "Missing required parameter: name".to_string().into(),
            data: None,
        })?;

    // Extract optional workspace parameter from context
    let workspace = context
        .arguments
        .as_ref()
        .and_then(|args| args.get("workspace"))
        .and_then(|v| v.as_str())
        .map(Path::new);

    // Extract optional input parameter from context
    let input = context
        .arguments
        .as_ref()
        .and_then(|args| args.get("input"))
        .and_then(|v| v.as_str());

    // Load the function
    let function = match crate::load_function(name) {
        Ok(f) => f,
        Err(e) => {
            let error_result = json!({
                "error": format!("Failed to load function '{}': {}", name, e),
                "success": false
            });
            return Ok(CallToolResult::structured_error(error_result));
        }
    };

    // Execute the function
    match crate::run_function(&function, workspace, input) {
        Ok(outputs) => {
            let result = json!({
                "outputs": outputs,
                "success": true,
                "function_name": name
            });
            Ok(CallToolResult::structured(result))
        }
        Err(e) => {
            let error_result = json!({
                "error": format!("Failed to execute function '{}': {}", name, e),
                "success": false
            });
            Ok(CallToolResult::structured_error(error_result))
        }
    }
}

/// Create the func_execute tool route
pub fn func_execute_tool_route() -> ToolRoute<MagickServerHandler> {
    let input_schema: serde_json::Value = json!({
        "type": "object",
        "properties": {
            "name": {
                "type": "string",
                "description": "Name of the function to execute"
            },
            "workspace": {
                "type": "string",
                "description": "Workspace path to set as the working directory for commands"
            },
            "input": {
                "type": "string",
                "description": "Optional input value to replace $input placeholders in commands"
            }
        },
        "required": ["name", "workspace"]
    });
    let tool = Tool::new(
        "func_execute",
        "Execute a magick function by name, running all commands in sequence",
        input_schema.as_object().unwrap().clone(),
    );
    ToolRoute::new_dyn(tool, |context| Box::pin(func_execute_tool(context)))
}
