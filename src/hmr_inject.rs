// Module for handling HMR client code injection

use anyhow::Result;
use log::debug;
use std::io::Read;
use std::path::Path;

/// The HMR client script as a static string
pub const HMR_CLIENT_SCRIPT: &str = include_str!("hmr_client.js");

/// Checks if a file is an HTML file based on extension
pub fn is_html_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        ext.to_string_lossy().eq_ignore_ascii_case("html")
    } else {
        false
    }
}

/// Inject HMR client code into an HTML response
pub fn inject_hmr_client(html_content: &str, _port: u16) -> Result<String> {
    debug!("Injecting HMR client code into HTML response");

    // Check if the HTML content already has the HMR client script
    if html_content.contains("__ORBIT_REGISTER_HMR_HANDLER") {
        debug!("HMR client code already present in HTML");
        return Ok(html_content.to_owned());
    }

    // Find where to inject the script (before closing </body> tag)
    if let Some(pos) = html_content.to_lowercase().rfind("</body>") {
        let (before, after) = html_content.split_at(pos);

        // We can either embed the script or reference it as an external file
        // Using external file is often better for debugging
        let script = format!(
            "<script type=\"text/javascript\" src=\"/__orbit_hmr_client.js?v={}\"></script>\n",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        );

        // Inject the script
        let injected_html = format!("{}{}{}", before, script, after);
        debug!("HMR client code injected successfully");

        Ok(injected_html)
    } else {
        // If no </body> tag is found, append the script at the end
        debug!("No </body> tag found, appending HMR client code at the end");
        let script = format!(
            "<script type=\"text/javascript\" src=\"/__orbit_hmr_client.js?v={}\"></script>\n",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        );
        let injected_html = format!("{}{}", html_content, script);

        Ok(injected_html)
    }
}

/// Serve HMR client code as a standalone JavaScript file
pub fn get_hmr_client_js() -> &'static str {
    HMR_CLIENT_SCRIPT
}

/// Process HTML file and inject HMR client code
pub fn process_html_file(path: &Path, port: u16) -> Result<Vec<u8>> {
    debug!("Processing HTML file: {:?}", path);

    // Read HTML file content
    let mut file = std::fs::File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // Inject HMR client code
    let injected_content = inject_hmr_client(&content, port)?;

    Ok(injected_content.into_bytes())
}
