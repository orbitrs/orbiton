// HMR client-side script for Orbit Framework
// This is injected into HTML during development to enable hot module reloading

(function() {
    // Configuration
    const config = {
        reconnectInterval: 2000,  // Reconnection interval in ms
        reconnectMaxAttempts: 10,
        debug: true
    };

    // HMR state
    let socket = null;
    let reconnectAttempts = 0;
    let isConnected = false;

    // Create a logger that respects the debug setting
    const log = {
        info: (message) => {
            if (config.debug) console.info(`[Orbit HMR] ${message}`);
        },
        warn: (message) => {
            if (config.debug) console.warn(`[Orbit HMR] ${message}`);
        },
        error: (message) => {
            if (config.debug) console.error(`[Orbit HMR] ${message}`);
        }
    };

    // Initialize the HMR system
    function init() {
        log.info("Initializing HMR system");
        
        // Connect to the WebSocket server
        const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
        const port = parseInt(window.location.port) + 1; // WebSocket server port
        const wsUrl = `${protocol}//${window.location.hostname}:${port}`;
        
        connectWebSocket(wsUrl);
        
        // Listen for page unload to close the socket
        window.addEventListener('beforeunload', () => {
            if (socket && socket.readyState === WebSocket.OPEN) {
                socket.close();
            }
        });
    }

    // Connect to the WebSocket server
    function connectWebSocket(url) {
        if (socket) {
            socket.close();
        }

        try {
            log.info(`Connecting to WebSocket server: ${url}`);
            socket = new WebSocket(url);

            socket.onopen = () => {
                log.info('WebSocket connection established');
                isConnected = true;
                reconnectAttempts = 0;
                
                // Register the client
                socket.send(JSON.stringify({
                    type: 'register',
                    url: window.location.pathname
                }));
            };

            socket.onclose = () => {
                log.info('WebSocket connection closed');
                isConnected = false;
                attemptReconnect(url);
            };

            socket.onerror = (error) => {
                log.error(`WebSocket error: ${error}`);
                isConnected = false;
            };

            socket.onmessage = (event) => {
                handleMessage(event.data);
            };
        } catch (error) {
            log.error(`Failed to connect to WebSocket: ${error}`);
            attemptReconnect(url);
        }
    }

    // Handle incoming WebSocket messages
    function handleMessage(data) {
        try {
            const message = JSON.parse(data);
            
            switch (message.type) {
                case 'fileChange':
                    log.info(`File change detected: ${message.paths.join(', ')}`);
                    break;
                    
                case 'rebuild':
                    handleRebuild(message);
                    break;
                    
                case 'hmr':
                    handleHmrUpdate(message);
                    break;
                    
                default:
                    log.warn(`Unknown message type: ${message.type}`);
            }
        } catch (error) {
            log.error(`Error handling message: ${error}`);
        }
    }

    // Handle rebuild messages
    function handleRebuild(message) {
        const statusIndicator = document.getElementById('orbit-hmr-status') || 
            createStatusIndicator();
        
        if (message.status === 'started') {
            log.info('Project rebuild started');
            statusIndicator.textContent = 'Rebuilding...';
            statusIndicator.className = 'orbit-hmr-status rebuilding';
        }
        else if (message.status === 'completed') {
            log.info('Project rebuild completed successfully');
            statusIndicator.textContent = 'Rebuild successful';
            statusIndicator.className = 'orbit-hmr-status success';
            
            // Hide the indicator after a delay
            setTimeout(() => {
                statusIndicator.className = 'orbit-hmr-status hidden';
            }, 3000);
        }
        else if (message.status === 'failed') {
            log.error('Project rebuild failed');
            statusIndicator.textContent = 'Rebuild failed';
            statusIndicator.className = 'orbit-hmr-status error';
        }
    }    // Handle HMR updates
    function handleHmrUpdate(message) {
        log.info(`HMR update for modules: ${message.modules.join(', ')}`);
        
        // Create a status indicator to show HMR activity
        const statusIndicator = document.getElementById('orbit-hmr-status') || 
            createStatusIndicator();
        
        statusIndicator.textContent = 'Applying HMR updates...';
        statusIndicator.className = 'orbit-hmr-status rebuilding';
        
        try {
            // Apply HMR updates - the actual implementation depends on your framework
            if (window.__ORBIT_APPLY_HMR) {
                const result = window.__ORBIT_APPLY_HMR(message.modules);
                
                // Handle Promise or direct result
                if (result instanceof Promise) {
                    result.then(() => {
                        statusIndicator.textContent = 'HMR update successful';
                        statusIndicator.className = 'orbit-hmr-status success';
                        setTimeout(() => {
                            statusIndicator.className = 'orbit-hmr-status hidden';
                        }, 3000);
                    }).catch((error) => {
                        log.error(`HMR update failed: ${error}`);
                        statusIndicator.textContent = 'HMR failed, reloading page...';
                        statusIndicator.className = 'orbit-hmr-status error';
                        setTimeout(() => {
                            window.location.reload();
                        }, 1000);
                    });
                } else {
                    statusIndicator.textContent = 'HMR update successful';
                    statusIndicator.className = 'orbit-hmr-status success';
                    setTimeout(() => {
                        statusIndicator.className = 'orbit-hmr-status hidden';
                    }, 3000);
                }
            } else {
                // If no HMR handler is registered, perform a full reload
                log.warn('No HMR handler registered, performing full page reload');
                statusIndicator.textContent = 'No HMR handler, reloading page...';
                statusIndicator.className = 'orbit-hmr-status rebuilding';
                setTimeout(() => {
                    window.location.reload();
                }, 500);
            }
        } catch (error) {
            log.error(`Error during HMR update: ${error}`);
            statusIndicator.textContent = 'HMR error, reloading page...';
            statusIndicator.className = 'orbit-hmr-status error';
            setTimeout(() => {
                window.location.reload();
            }, 1000);
        }
    }

    // Create a status indicator element
    function createStatusIndicator() {
        const indicator = document.createElement('div');
        indicator.id = 'orbit-hmr-status';
        indicator.className = 'orbit-hmr-status hidden';
        
        const style = document.createElement('style');
        style.textContent = `
            .orbit-hmr-status {
                position: fixed;
                bottom: 20px;
                right: 20px;
                padding: 8px 12px;
                border-radius: 4px;
                font-family: sans-serif;
                font-size: 14px;
                z-index: 9999;
                transition: opacity 0.3s ease;
                color: white;
                box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
            }
            .orbit-hmr-status.hidden {
                opacity: 0;
                pointer-events: none;
            }
            .orbit-hmr-status.rebuilding {
                background-color: #f39c12;
            }
            .orbit-hmr-status.success {
                background-color: #2ecc71;
            }
            .orbit-hmr-status.error {
                background-color: #e74c3c;
            }
        `;
        
        document.head.appendChild(style);
        document.body.appendChild(indicator);
        return indicator;
    }

    // Attempt to reconnect to the WebSocket server
    function attemptReconnect(url) {
        if (reconnectAttempts >= config.reconnectMaxAttempts) {
            log.error('Maximum reconnection attempts reached');
            return;
        }
        
        reconnectAttempts++;
        
        const delay = config.reconnectInterval;
        log.info(`Attempting to reconnect in ${delay}ms (attempt ${reconnectAttempts}/${config.reconnectMaxAttempts})`);
        
        setTimeout(() => {
            connectWebSocket(url);
        }, delay);
    }

    // Register an HMR update handler
    window.__ORBIT_REGISTER_HMR_HANDLER = function(handler) {
        window.__ORBIT_APPLY_HMR = handler;
        log.info('HMR handler registered');
    };

    // Initialize on page load
    if (document.readyState === 'complete') {
        init();
    } else {
        window.addEventListener('load', init);
    }
})();
