# std.net

Networking primitives for socket communication.

## Functions

### `connect(address: String) -> Int`
Opens a connection to the given `address`. Returns a socket file descriptor.

### `send(socket: Int, data: String) -> Int`
Sends `data` over the socket. Returns the number of bytes sent.

### `recv(socket: Int, buffer: String, size: Int) -> Int`
Receives up to `size` bytes into `buffer` from the socket. Returns the number of bytes received.

## Examples

```nimble
load std.net

let sock = net.connect("127.0.0.1:8080")
net.send(sock, "Hello")
```
