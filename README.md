# Encinitas Collector

## Overview
Encinitas Collector is a Rust-based web server designed for collecting and processing web request and response information coming from [the encinits dApp agent JS service worker](https://github.com/jcleira/encinitas-dapp-agent).

## Features
- **Data Capture**: Collects detailed information about web requests and responses.
- **Metrics Exposure**: Integrates with Prometheus to expose relevant metrics.

## Installation

1. **Clone the Repository**:
```bash
git clone https://github.com/yourusername/encinitas_collector.git
cd encinitas_collector
```

2. **Run the Server**:
```bash
cargo run
```

## Usage

### Starting the Server
Run the server using Cargo:
```bash
cargo run
```
The server will start listening on `localhost:3000` by default.

### Sending Data to the Capture Endpoint
Use `curl` or any HTTP client to send POST requests to the `/capture` endpoint:

```bash
curl -X POST http://localhost:8080/capture \
    -H "Content-Type: application/json" \
    -d '{
        "fetch_event": {
            "type": "FetchStart",
            "request": "RequestObjectRef123",
            "client_id": "Client123",
            "preload_response": null,
            "resulting_client_id": "Client456",
            "replaces_client_id": "Client789"
        },
        "request": {
            "url": "https://example.com/api/data",
            "method": "GET",
            "headers": {
                "Accept": "application/json",
                "Authorization": "Bearer sampletoken123"
            },
            "body": null,
            "referrer": "https://example.com",
            "referrer_policy": "no-referrer",
            "mode": "cors",
            "credentials": "include",
            "cache": "default",
            "redirect": "follow",
            "integrity": "",
            "keepalive": true,
            "signal": null
        },
        "response": {
            "url": "https://example.com/api/data",
            "type": "cors",
            "status": 200,
            "ok": true,
            "status_text": "OK",
            "headers": {
                "Content-Type": "application/json"
            },
            "body": "{\"key\":\"value\"}",
            "redirected": false,
            "body_used": false
        }
    }'

```

### Viewing Metrics
Metrics are exposed at the `/metrics` endpoint and can be accessed via:

```bash
curl http://localhost:8080/metrics
```
