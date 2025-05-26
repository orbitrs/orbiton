// Development server for the Orbit UI framework

use anyhow::Result;
use futures_util::{future, SinkExt, StreamExt};
use log::{debug, error, info};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::{Path, PathBuf},
    sync::Arc,
    thread,
};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

use crate::hmr::HmrContext;
use crate::hmr_inject::{get_hmr_client_js, is_html_file, process_html_file};

/// Development server
pub struct DevServer {
    /// Port to use for the server
    port: u16,
    /// Project directory
    project_dir: PathBuf,
    /// Server thread handle
    #[allow(dead_code)]
    thread_handle: Option<thread::JoinHandle<()>>,
    /// Broadcast channel for sending updates to connected clients
    tx: Option<broadcast::Sender<String>>,
    /// Use beta toolchain for building and testing
    use_beta: bool,
    /// HMR context for tracking changed modules
    hmr_context: Arc<HmrContext>,
}

impl Clone for DevServer {
    fn clone(&self) -> Self {
        Self {
            port: self.port,
            project_dir: self.project_dir.clone(),
            thread_handle: None, // Don't clone the thread handle
            tx: self.tx.clone(),
            use_beta: self.use_beta,
            hmr_context: Arc::clone(&self.hmr_context),
        }
    }
}

impl DevServer {
    /// Create a new development server
    #[allow(dead_code)] // Used in tests and maintenance operations
    pub fn new(port: u16, project_dir: &Path) -> Result<Self> {
        let (tx, _) = broadcast::channel(16);
        let hmr_context = Arc::new(HmrContext::new(project_dir.to_owned()));

        Ok(Self {
            port,
            project_dir: project_dir.to_owned(),
            thread_handle: None,
            tx: Some(tx),
            use_beta: false,
            hmr_context,
        })
    }

    /// Create a new development server with optional beta toolchain support
    pub fn new_with_options(port: u16, project_dir: &Path, use_beta: bool) -> Result<Self> {
        let (tx, _) = broadcast::channel(16);
        let hmr_context = Arc::new(HmrContext::new(project_dir.to_owned()));

        Ok(Self {
            port,
            project_dir: project_dir.to_owned(),
            thread_handle: None,
            tx: Some(tx),
            use_beta,
            hmr_context,
        })
    }

    /// Check if the dev server is using beta toolchain
    pub fn is_using_beta(&self) -> bool {
        self.use_beta
    }

    /// Get the server port
    #[allow(dead_code)] // Used in tests and maintenance operations
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Get the HMR context
    pub fn hmr_context(&self) -> &Arc<HmrContext> {
        &self.hmr_context
    }

    /// Start the development server
    pub fn start(&mut self) -> Result<&thread::JoinHandle<()>> {
        let port = self.port;
        let project_dir = self.project_dir.clone();
        let tx = self.tx.take().expect("Missing broadcast channel");

        let handle = thread::spawn(move || {
            // Set up the Tokio runtime
            let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

            rt.block_on(async {
                // Start WebSocket server
                let ws_rx = tx.subscribe();
                let ws_handle = tokio::spawn(Self::run_websocket_server(port, ws_rx));

                // Start HTTP server
                let server = tiny_http::Server::http(format!("0.0.0.0:{port}"))
                    .expect("Failed to start HTTP server");

                info!("Development server started on port {port}");
                info!("WebSocket server started on port {}", port + 1);

                let _broadcast_tx = tx; // Keep tx alive

                for request in server.incoming_requests() {
                    debug!("Received request: {url:?}", url = request.url());

                    // Special handling for HMR client script
                    if request.url() == "/__orbit_hmr_client.js" {
                        debug!("Serving HMR client script");
                        let response = tiny_http::Response::from_string(get_hmr_client_js())
                            .with_header(
                                tiny_http::Header::from_bytes(
                                    &b"Content-Type"[..],
                                    &b"application/javascript"[..],
                                )
                                .unwrap(),
                            );
                        let _ = request.respond(response);
                        continue;
                    }

                    // Handle static files
                    let url = request.url().trim_start_matches('/');
                    let file_path = if url.is_empty() {
                        project_dir.join("index.html")
                    } else {
                        project_dir.join(url)
                    };

                    if file_path.exists() && file_path.is_file() {
                        // Special handling for HTML files to inject HMR client
                        if is_html_file(&file_path) {
                            debug!("Processing HTML file: {file_path:?}");
                            match process_html_file(&file_path, port) {
                                Ok(content) => {
                                    let response = tiny_http::Response::from_data(content)
                                        .with_header(
                                            tiny_http::Header::from_bytes(
                                                &b"Content-Type"[..],
                                                &b"text/html"[..],
                                            )
                                            .unwrap(),
                                        );
                                    let _ = request.respond(response);
                                }
                                Err(e) => {
                                    error!("Failed to process HTML file: {e}");
                                    // Fall back to serving the file without injection
                                    let file = std::fs::File::open(&file_path)
                                        .expect("Failed to open file");
                                    let response = tiny_http::Response::from_file(file);
                                    let _ = request.respond(response);
                                }
                            }
                        } else {
                            // Serve non-HTML files normally
                            let file =
                                std::fs::File::open(&file_path).expect("Failed to open file");
                            let response = tiny_http::Response::from_file(file);
                            let _ = request.respond(response);
                        }
                    } else {
                        // File not found, return 404
                        let response = tiny_http::Response::from_string("File not found")
                            .with_status_code(404);
                        let _ = request.respond(response);
                    }
                }

                // Wait for WebSocket server to finish
                let _ = ws_handle.await;
            });
        });

        self.thread_handle = Some(handle);
        Ok(self.thread_handle.as_ref().unwrap())
    }

    /// Send an update to all connected WebSocket clients
    pub fn broadcast_update(&self, message: String) -> Result<()> {
        if let Some(tx) = &self.tx {
            tx.send(message)
                .map_err(|e| anyhow::anyhow!("Failed to broadcast message: {}", e))?;
        }
        Ok(())
    }

    /// Trigger an HMR update for specific modules
    pub fn send_hmr_update(&self, modules: Vec<String>) -> Result<()> {
        let message = serde_json::json!({
            "type": "hmr",
            "modules": modules
        })
        .to_string();

        self.broadcast_update(message)
    }

    /// Trigger a full page reload for all clients
    pub fn send_reload_command(&self) -> Result<()> {
        let message = serde_json::json!({
            "type": "reload"
        })
        .to_string();

        self.broadcast_update(message)
    }

    /// Send rebuild status to all clients
    pub fn send_rebuild_status(&self, status: &str) -> Result<()> {
        let message = serde_json::json!({
            "type": "rebuild",
            "status": status
        })
        .to_string();

        self.broadcast_update(message)
    }

    async fn handle_websocket_connection(
        ws_stream: tokio_tungstenite::WebSocketStream<TcpStream>,
        addr: SocketAddr,
        mut rx: broadcast::Receiver<String>,
    ) {
        info!("WebSocket connection established: {addr}");
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        // Send initial connection acknowledgment
        let hello_msg = serde_json::json!({
            "type": "hello",
            "message": "Orbit HMR connected"
        })
        .to_string();

        if let Err(e) = ws_sender.send(Message::Text(hello_msg)).await {
            error!("Error sending hello message: {e}");
            return;
        }

        let send_task = tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                ws_sender
                    .send(Message::Text(msg))
                    .await
                    .unwrap_or_else(|e| error!("Error sending message: {e}"));
            }
        });

        let recv_task = tokio::spawn(async move {
            while let Some(msg) = ws_receiver.next().await {
                if let Ok(msg) = msg {
                    if msg.is_close() {
                        break;
                    }

                    // Handle incoming messages from client
                    if let Message::Text(text) = msg {
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
                            if let Some(msg_type) = json.get("type").and_then(|t| t.as_str()) {
                                match msg_type {
                                    "register" => {
                                        if let Some(path) = json.get("url").and_then(|p| p.as_str())
                                        {
                                            debug!("Client registered for path: {path}");
                                            // Could store client info in a map for targeted updates
                                        }
                                    }
                                    "hmr_ready" => {
                                        debug!("Client reported HMR ready state");
                                    }
                                    _ => debug!("Received unknown message type: {msg_type}"),
                                }
                            }
                        }
                    }
                }
            }
        });

        future::select(send_task, recv_task).await;
        info!("WebSocket connection closed: {addr}");
    }

    /// Start the WebSocket server
    async fn run_websocket_server(port: u16, rx: broadcast::Receiver<String>) -> Result<()> {
        let addr = (IpAddr::V4(Ipv4Addr::LOCALHOST), port + 1);
        let listener = TcpListener::bind(addr).await?;
        info!("WebSocket server listening on: localhost:{}", port + 1);

        while let Ok((stream, addr)) = listener.accept().await {
            let ws_stream = accept_async(stream).await?;
            let rx = rx.resubscribe();

            tokio::spawn(async move {
                Self::handle_websocket_connection(ws_stream, addr, rx).await;
            });
        }
        Ok(())
    }
}
