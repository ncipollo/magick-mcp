use rmcp::handler::server::ServerHandler;
use rmcp::model::ServerInfo;

/// Server handler for MCP tools
pub struct MagickServerHandler;

impl ServerHandler for MagickServerHandler {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: rmcp::model::ProtocolVersion::LATEST,
            capabilities: rmcp::model::ServerCapabilities {
                tools: Some(rmcp::model::ToolsCapability { list_changed: None }),
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
}
