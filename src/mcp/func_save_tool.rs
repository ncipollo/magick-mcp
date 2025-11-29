use crate::mcp::server::MagickServerHandler;
use rmcp::handler::server::router::tool::ToolRoute;
use rmcp::handler::server::tool::ToolCallContext;
use rmcp::model::{CallToolResult, ErrorCode, ErrorData, Tool};
use serde_json::json;

/// Save a magick function
async fn func_save_tool(
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

    // Extract commands array from context
    let commands_value = context
        .arguments
        .as_ref()
        .and_then(|args| args.get("commands"))
        .ok_or_else(|| ErrorData {
            code: ErrorCode::INVALID_PARAMS,
            message: "Missing required parameter: commands".to_string().into(),
            data: None,
        })?;

    let commands: Vec<String> = commands_value
        .as_array()
        .ok_or_else(|| ErrorData {
            code: ErrorCode::INVALID_PARAMS,
            message: "Parameter 'commands' must be an array".to_string().into(),
            data: None,
        })?
        .iter()
        .map(|v| {
            v.as_str().map(|s| s.to_string()).ok_or_else(|| ErrorData {
                code: ErrorCode::INVALID_PARAMS,
                message: "All items in 'commands' array must be strings"
                    .to_string()
                    .into(),
                data: None,
            })
        })
        .collect::<Result<Vec<String>, ErrorData>>()?;

    let function = crate::Function {
        name: name.to_string(),
        commands,
    };

    match crate::save_function(function) {
        Ok(_) => {
            let result = json!({
                "success": true,
                "message": format!("Function '{}' saved successfully", name)
            });
            Ok(CallToolResult::structured(result))
        }
        Err(e) => {
            let error_result = json!({
                "error": format!("Failed to save function: {}", e),
                "success": false
            });
            Ok(CallToolResult::structured_error(error_result))
        }
    }
}

/// Create the func_save tool route
pub fn func_save_tool_route() -> ToolRoute<MagickServerHandler> {
    let input_schema: serde_json::Value = json!({
        "type": "object",
        "properties": {
            "name": {
                "type": "string",
                "description": "Name of the function to save"
            },
            "commands": {
                "type": "array",
                "items": {
                    "type": "string"
                },
                "description": "Array of ImageMagick command strings to execute in sequence"
            }
        },
        "required": ["name", "commands"]
    });
    let tool = Tool::new(
        "func_save",
        "Save a magick function with a name and array of commands",
        input_schema.as_object().unwrap().clone(),
    );
    ToolRoute::new_dyn(tool, |context| Box::pin(func_save_tool(context)))
}
