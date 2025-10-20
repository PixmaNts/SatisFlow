# Satisflow Server

Production-ready REST API server for the Satisflow project, built with Rust and Axum.

## Features

- RESTful API with comprehensive endpoints
- Environment-based configuration
- Structured JSON logging for production
- Graceful shutdown handling
- CORS support
- Health checks
- Docker support
- Non-root container execution

## Quick Start

### Local Development

1. Clone the repository and navigate to the server directory:
   ```bash
   cd crates/satisflow-server
   ```

2. Copy the environment configuration:
   ```bash
   cp .env.example .env
   ```

3. Install dependencies and run:
   ```bash
   cargo run
   ```

The server will start on `http://localhost:3000` by default.

### Docker Deployment

1. Build and run with Docker Compose:
   ```bash
   docker-compose up -d
   ```

2. Or build and run with plain Docker:
   ```bash
   docker build -t satisflow-server .
   docker run -p 3000:3000 -e RUST_LOG=info satisflow-server
   ```

## Configuration

The server can be configured using environment variables. See `.env.example` for all available options:

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | `3000` | Server port |
| `HOST` | `127.0.0.1` | Server host |
| `RUST_LOG` | `info` | Logging level |
| `CORS_ORIGINS` | `http://localhost:5173` | Allowed CORS origins |
| `ENVIRONMENT` | `development` | Environment mode |

### Environment-specific Configuration

#### Development
- Human-readable logging
- Permissive CORS (allows all origins)
- Debug-level logging

#### Production
- Structured JSON logging
- Restricted CORS origins
- Info-level logging
- Non-root container execution

## API Endpoints

### Health Check
- `GET /health` - Server health status

### Factories
- `GET /api/factories` - List all factories
- `POST /api/factories` - Create a new factory
- `GET /api/factories/{id}` - Get a specific factory
- `PUT /api/factories/{id}` - Update a factory
- `DELETE /api/factories/{id}` - Delete a factory

### Logistics
- `GET /api/logistics` - List all logistics lines
- `POST /api/logistics` - Create a new logistics line
- `GET /api/logistics/{id}` - Get a specific logistics line
- `DELETE /api/logistics/{id}` - Delete a logistics line

### Dashboard
- `GET /api/dashboard/summary` - Get dashboard summary
- `GET /api/dashboard/items` - Get item balances
- `GET /api/dashboard/power` - Get power statistics

### Game Data
- `GET /api/game-data/recipes` - Get all recipes
- `GET /api/game-data/items` - Get all items
- `GET /api/game-data/machines` - Get all machines

## Logging

The server uses structured logging with `tracing` and `tracing-subscriber`.

### Development
Human-readable format with colors:
```
 INFO  satisflow_server: Satisflow server listening on 127.0.0.1:3000 in development mode
```

### Production
JSON format for log aggregation:
```json
{
  "timestamp": "2023-10-20T12:00:00.000Z",
  "level": "info",
  "message": "Satisflow server listening on 0.0.0.0:3000 in production mode",
  "target": "satisflow_server"
}
```

## Graceful Shutdown

The server supports graceful shutdown on:
- SIGINT (Ctrl+C)
- SIGTERM (Docker stop)

The server will:
1. Stop accepting new connections
2. Complete in-flight requests
3. Clean up resources
4. Exit gracefully

## Security

### Container Security
- Runs as non-root user
- Minimal runtime image
- No shell access by default

### CORS Configuration
- Production mode restricts origins
- Development mode allows all origins for convenience

### Logging
- No sensitive data in logs
- Structured format for analysis

## Monitoring

### Health Checks
The server provides a health check endpoint:
```bash
curl http://localhost:3000/health
```

Response:
```json
{
  "status": "healthy",
  "timestamp": "2023-10-20T12:00:00.000Z",
  "service": "satisflow-server"
}
```

### Docker Health Checks
Built-in health checks monitor:
- Endpoint availability
- Response time
- Service health

## Deployment Examples

### Docker Compose (Production)
```yaml
version: '3.8'
services:
  satisflow-server:
    image: satisflow-server:latest
    ports:
      - "3000:3000"
    environment:
      - ENVIRONMENT=production
      - RUST_LOG=info
      - CORS_ORIGINS=https://yourdomain.com
    restart: unless-stopped
```

### Kubernetes
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: satisflow-server
spec:
  replicas: 3
  selector:
    matchLabels:
      app: satisflow-server
  template:
    metadata:
      labels:
        app: satisflow-server
    spec:
      containers:
      - name: satisflow-server
        image: satisflow-server:latest
        ports:
        - containerPort: 3000
        env:
        - name: ENVIRONMENT
          value: "production"
        - name: RUST_LOG
          value: "info"
        livenessProbe:
          httpGet:
            path: /health
            port: 3000
          initialDelaySeconds: 30
          periodSeconds: 10
```

## Testing

Run the test suite:
```bash
cargo test
```

Run integration tests:
```bash
cargo test --test integration_tests
```

## Contributing

1. Follow the existing code style
2. Add tests for new features
3. Update documentation
4. Ensure all tests pass before submitting

## License

This project is licensed under the MIT License.