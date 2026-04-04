# net Module

HTTP helpers.

## Functions
- `http_get(url str) -> str | error`: Performs a blocking GET request and returns the response text.

## Example
```nimble
load net
body = net.http_get("https://example.com")?
out(len(body))
```
