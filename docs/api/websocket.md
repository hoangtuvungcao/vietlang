# Module `std.websocket`

## Exported Functions

### `fn ws_handshake_response(sec_websocket_key: String) -> String`

Generate RFC 6455 Handshake Response

### `fn ws_encode_text_frame(message: String)`

Encode WebSocket Text Frame (Unmasked Server -> Client)

### `fn ws_encode_ping_frame(payload: String = "")`

Encode WebSocket Ping Frame

### `fn ws_room_manager_new()`

WebSocket Room Broadcaster Manager

### `fn ws_room_join(manager, room_name: String, client_id: String)`

### `fn ws_room_broadcast(manager, room_name: String, event: String, data)`

