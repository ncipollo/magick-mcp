use rmcp::handler::server::ServerHandler;
use rmcp::model::{
    ErrorCode, ErrorData, ListResourcesResult, ReadResourceResult, ResourceContents, ServerInfo,
};
use rmcp::service::{RequestContext, RoleServer};

use crate::mcp::help_resource::{HELP_RESOURCE_URI, help_resource, read_help_resource};

/// Server handler for MCP tools
pub struct MagickServerHandler;

impl ServerHandler for MagickServerHandler {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: rmcp::model::ProtocolVersion::LATEST,
            capabilities: rmcp::model::ServerCapabilities {
                tools: Some(rmcp::model::ToolsCapability { list_changed: None }),
                resources: Some(rmcp::model::ResourcesCapability::default()),
                ..Default::default()
            },
            server_info: rmcp::model::Implementation {
                name: "magick-mcp".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                title: None,
                icons: None,
                website_url: None,
            },
            instructions: Some(
                "A Model Context Protocol server for checking ImageMagick installation."
                    .to_string(),
            ),
        }
    }

    fn list_resources(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParam>,
        _context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<ListResourcesResult, ErrorData>> + Send + '_ {
        std::future::ready(Ok(ListResourcesResult {
            resources: vec![help_resource()],
            next_cursor: None,
        }))
    }

    fn read_resource(
        &self,
        request: rmcp::model::ReadResourceRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<ReadResourceResult, ErrorData>> + Send + '_ {
        std::future::ready({
            if request.uri == HELP_RESOURCE_URI {
                match read_help_resource() {
                    Ok(help_text) => Ok(ReadResourceResult {
                        contents: vec![ResourceContents::text(help_text, HELP_RESOURCE_URI)],
                    }),
                    Err(e) => Err(ErrorData {
                        code: ErrorCode::INTERNAL_ERROR,
                        message: format!("Failed to read ImageMagick help: {e}").into(),
                        data: None,
                    }),
                }
            } else {
                Err(ErrorData {
                    code: ErrorCode::INVALID_PARAMS,
                    message: format!("Unknown resource URI: {}", request.uri).into(),
                    data: None,
                })
            }
        })
    }
}
