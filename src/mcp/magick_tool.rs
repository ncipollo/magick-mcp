use crate::mcp::server::MagickServerHandler;
use rmcp::handler::server::router::tool::ToolRoute;
use rmcp::handler::server::tool::ToolCallContext;
use rmcp::model::{CallToolResult, ErrorCode, ErrorData, Tool};
use serde_json::json;

/// Execute an ImageMagick command
///
/// The provided text should be an ImageMagick command (don't include 'magick').
/// It should not contain subcommands like 'convert', 'identify', etc.
async fn magick_tool(
    context: ToolCallContext<'_, MagickServerHandler>,
) -> Result<CallToolResult, ErrorData> {
    // Extract command parameter from context
    let command = context
        .arguments
        .as_ref()
        .and_then(|args| args.get("command"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| ErrorData {
            code: ErrorCode::INVALID_PARAMS,
            message: "Missing required parameter: command".to_string().into(),
            data: None,
        })?;

    match crate::magick(command) {
        Ok(output) => {
            let result = json!({
                "output": output,
                "success": true
            });
            Ok(CallToolResult::structured(result))
        }
        Err(e) => {
            let error_result = json!({
                "error": format!("Magick command failed: {}", e),
                "success": false
            });
            Ok(CallToolResult::structured_error(error_result))
        }
    }
}

/// Create the magick tool route
pub fn magick_tool_route() -> ToolRoute<MagickServerHandler> {
    let input_schema: serde_json::Value = json!({
        "type": "object",
        "properties": {
            "command": {
                "type": "string",
                "description": "ImageMagick command arguments (e.g., 'test.png -negate out.png'). Do not include 'magick' prefix or subcommands like 'convert', 'identify', etc."
            }
        },
        "required": ["command"]
    });
    let tool = Tool::new(
        "magick",
        "Execute an ImageMagick command. The provided text should be an ImageMagick command (don't include 'magick'). It should not contain subcommands like 'convert', 'identify', etc.",
        input_schema.as_object().unwrap().clone(),
    );
    ToolRoute::new_dyn(tool, |context| Box::pin(magick_tool(context)))
}
