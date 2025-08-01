mergeInto(LibraryManager.library, {
    WebSocket_Init: function(openCallback, messageCallback, closeCallback, errorCallback) {
        this._webSocketManager = {
            instances: {},
            nextId: 1,
            callbacks: {
                open: null,
                message: null,
                close: null,
                error: null
            }
        };
        
        var manager = this._webSocketManager;
        manager.callbacks.open = openCallback;
        manager.callbacks.message = messageCallback;
        manager.callbacks.close = closeCallback;
        manager.callbacks.error = errorCallback;
    },

    WebSocket_Connect: async function(baseUriPtr, uriPtr, protocolPtr, authTokenPtr, callbackPtr) {
        try {
            var manager = this._webSocketManager;
            var host = UTF8ToString(baseUriPtr);
            var uri = UTF8ToString(uriPtr);
            var protocol = UTF8ToString(protocolPtr);
            var authToken = UTF8ToString(authTokenPtr);
            if (authToken)
            {
                var tokenUrl = new URL('v1/identity/websocket-token', host);
                tokenUrl.protocol = host.startsWith("wss://") ? 'https:' : 'http:';
                var headers = new Headers();
                headers.set('Authorization', `Bearer ${authToken}`);

                var response = await fetch(tokenUrl, {
                    method: 'POST',
                    headers: headers
                });
                if (response.ok) {
                    const { token } = await response.json();
                    if (token) {
                        uri += `&token=${token}`;
                    }
                } else {
                    throw new Error(`Failed to verify token: ${response.statusText}`);
                }
            }

            var socket = new window.WebSocket(uri, protocol);
            socket.binaryType = "arraybuffer";

            var socketId = manager.nextId++;
            manager.instances[socketId] = socket;

            socket.onopen = function() {
                if (manager.callbacks.open) {
                    dynCall('vi', manager.callbacks.open, [socketId]);
                }
            };

            socket.onmessage = function(event) {
                if (manager.callbacks.message && event.data instanceof ArrayBuffer) {
                    var buffer = _malloc(event.data.byteLength);
                    HEAPU8.set(new Uint8Array(event.data), buffer);
                    dynCall('viii', manager.callbacks.message, [socketId, buffer, event.data.byteLength]);
                    _free(buffer);
                }
            };
            socket.onclose = function(event) {
                if (manager.callbacks.close) {
                    var reasonStr = event.reason || "";
                    var reasonArray = intArrayFromString(reasonStr);
                    var reasonPtr = _malloc(reasonArray.length);
                    HEAP8.set(reasonArray, reasonPtr);
                    dynCall('viii', manager.callbacks.close, [socketId, event.code, reasonPtr]);
                    _free(reasonPtr);
                }
                delete manager.instances[socketId];
            };

            socket.onerror = function(error) {
                if (manager.callbacks.error) {
                    dynCall('vi', manager.callbacks.error, [socketId]);
                }
            };

            dynCall('vi', callbackPtr, [socketId]);
        } catch (e) {
            console.error("WebSocket connection error:", e);
            dynCall('vi', callbackPtr, [-1]);
        }
    },

    WebSocket_Send: function(socketId, dataPtr, length) {
        var manager = this._webSocketManager;
        var socket = manager.instances[socketId];
        if (!socket || socket.readyState !== socket.OPEN) return -1;

        try {
            var data = new Uint8Array(HEAPU8.buffer, dataPtr, length);
            socket.send(data);
            return 0;
        } catch (e) {
            console.error("WebSocket send error:", e);
            return -1;
        }
    },

    WebSocket_Close: function(socketId, code, reasonPtr) {
        var manager = this._webSocketManager;
        var socket = manager.instances[socketId];
        if (!socket) return;

        var reason = UTF8ToString(reasonPtr);
        socket.close(code, reason);
        delete manager.instances[socketId];
    }
});