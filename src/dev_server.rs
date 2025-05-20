// Development server for the Orbit UI framework

use anyhow::Result;
use log::{debug, info};
use std::path::{Path, PathBuf};
use std::thread;

/// Development server
pub struct DevServer {
    /// Port to use for the server
    port: u16,
    /// Project directory
    project_dir: PathBuf,
    /// Server thread handle
    thread_handle: Option<thread::JoinHandle<()>>,
}

impl DevServer {
    /// Create a new development server
    pub fn new(port: u16, project_dir: &Path) -> Result<Self> {
        Ok(Self {
            port,
            project_dir: project_dir.to_owned(),
            thread_handle: None,
        })
    }

    /// Start the development server
    pub fn start(&mut self) -> Result<&thread::JoinHandle<()>> {
        let port = self.port;
        let project_dir = self.project_dir.clone();

        let handle = thread::spawn(move || {
            let server = tiny_http::Server::http(format!("0.0.0.0:{}", port))
                .expect("Failed to start HTTP server");

            info!("Development server started on port {}", port);

            for request in server.incoming_requests() {
                debug!("Received request: {:?}", request.url());

                // Handle static files
                let url = request.url().trim_start_matches('/');
                let file_path = if url.is_empty() {
                    project_dir.join("index.html")
                } else {
                    project_dir.join(url)
                };

                if file_path.exists() && file_path.is_file() {
                    // Serve the file
                    let file = std::fs::File::open(&file_path).expect("Failed to open file");

                    let response = tiny_http::Response::from_file(file);
                    let _ = request.respond(response);
                } else {
                    // File not found, return 404
                    let response =
                        tiny_http::Response::from_string("File not found").with_status_code(404);
                    let _ = request.respond(response);
                }
            }
        });

        self.thread_handle = Some(handle);
        Ok(self.thread_handle.as_ref().unwrap())
    }
}
