use rmcp::model::Resource;

/// URI for the ImageMagick help resource
pub const HELP_RESOURCE_URI: &str = "magick://help";

/// Create the help resource metadata
pub fn help_resource() -> Resource {
    Resource::new(
        rmcp::model::RawResource {
            uri: HELP_RESOURCE_URI.to_string(),
            name: "ImageMagick Help".to_string(),
            title: Some("ImageMagick Help Documentation".to_string()),
            description: Some("Help documentation for ImageMagick command-line tool. Use this to learn about the available commands and options.".to_string()),
            mime_type: Some("text/plain".to_string()),
            size: None,
            icons: None,
        },
        None,
    )
}

/// Read the help resource contents
///
/// # Returns
///
/// Returns the help text from `magick --help`, or an error if execution fails
pub fn read_help_resource() -> Result<String, crate::feature::ShellError> {
    crate::help()
}
