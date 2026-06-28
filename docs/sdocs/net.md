# std.net

Networking primitives - TCP socket communication and DNS resolution.

All functions are backed by the Rust runtime with a global connection handle map.

## Functions

### `tcp_connect(host: String, port: Int) -> Int`
Opens a TCP connection to `host`:`port`. Returns a socket handle (positive integer). Returns -1 on failure.

### `tcp_send(socket: Int, data: String) -> Int`
Sends `data` over the socket. Returns the number of bytes sent, or -1 on error.

### `tcp_recv(socket: Int, size: Int) -> String`
Receives up to `size` bytes from the socket. Returns the received data as a string.

### `tcp_close(socket: Int) -> Bool`
Closes the socket connection. Returns `true` on success.

### `dns_resolve(host: String) -> String`
Resolves a hostname to an IP address string. Returns the first resolved address, or an empty string on failure.

## Examples

```nimble
load std.net

let sock = net.tcp_connect("example.com", 80)
if sock >= 0:
    let sent = net.tcp_send(sock, "GET / HTTP/1.0\r\n\r\n")
    let resp = net.tcp_recv(sock, 4096)
    net.tcp_close(sock)

let ip = net.dns_resolve("example.com")
```
