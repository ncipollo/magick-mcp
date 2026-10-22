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
        let mut capabilities = rmcp::model::ServerCapabilities::default();
        capabilities.tools = Some(rmcp::model::ToolsCapability { list_changed: None });
        capabilities.resources = Some(rmcp::model::ResourcesCapability::default());
        let mut info = ServerInfo::new(capabilities).with_instructions(
            "A Model Context Protocol server for checking ImageMagick installation.",
        );
        info.server_info =
            rmcp::model::Implementation::new("magick-mcp", env!("CARGO_PKG_VERSION"));
        info
    }

    fn list_resources(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<ListResourcesResult, ErrorData>> + Send + '_ {
        std::future::ready(Ok(ListResourcesResult {
            resources: vec![help_resource()],
            next_cursor: None,
            meta: None,
        }))
    }

    fn read_resource(
        &self,
        request: rmcp::model::ReadResourceRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<ReadResourceResult, ErrorData>> + Send + '_ {
        std::future::ready({
            if request.uri == HELP_RESOURCE_URI {
                match read_help_resource() {
                    Ok(help_text) => Ok(ReadResourceResult::new(vec![ResourceContents::text(
                        help_text,
                        HELP_RESOURCE_URI,
                    )])),
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
