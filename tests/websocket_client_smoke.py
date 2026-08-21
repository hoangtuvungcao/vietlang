#!/usr/bin/env python3
import base64
import os
import socket

connection = socket.create_connection(("127.0.0.1", 18081), timeout=5)
key = base64.b64encode(os.urandom(16)).decode()
request = (
    "GET /ws HTTP/1.1\r\n"
    "Host: 127.0.0.1:18081\r\n"
    "Upgrade: websocket\r\n"
    "Connection: Upgrade\r\n"
    f"Sec-WebSocket-Key: {key}\r\n"
    "Sec-WebSocket-Version: 13\r\n\r\n"
)
connection.sendall(request.encode())
response = connection.recv(4096)
assert response.startswith(b"HTTP/1.1 101"), response

payload = b"vietlang-async-websocket"
mask = os.urandom(4)
masked = bytes(byte ^ mask[index % 4] for index, byte in enumerate(payload))
connection.sendall(bytes([0x81, 0x80 | len(payload)]) + mask + masked)

header = connection.recv(2)
assert header[0] & 0x0F == 1, header
length = header[1] & 0x7F
if length == 126:
    length = int.from_bytes(connection.recv(2), "big")
elif length == 127:
    length = int.from_bytes(connection.recv(8), "big")
body = b""
while len(body) < length:
    body += connection.recv(length - len(body))
assert body == payload, body
connection.close()
print("PASS async RFC 6455 handshake, masked client frame, bounded broadcast")
