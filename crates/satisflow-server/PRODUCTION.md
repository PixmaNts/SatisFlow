# Production Deployment Guide

This guide provides detailed instructions for deploying the Satisflow server in a production environment.

## Prerequisites

- Docker 20.10+ installed
- At least 512MB RAM available
- Port 3000 (or custom port) available

## Quick Start

### Using Docker Compose (Recommended)

1. Copy the environment configuration:
   ```bash
   cp .env.example .env
   ```

2. Edit the `.env` file with your production settings:
   ```env
   PORT=3000
   HOST=0.0.0.0
   RUST_LOG=info
   CORS_ORIGINS=https://yourdomain.com
   ENVIRONMENT=production
   ```

3. Start the service:
   ```bash
   docker-compose up -d
   ```

### Using Deployment Scripts

#### Linux/macOS
```bash
# Build the image
./deploy.sh build

# Run the container
./deploy.sh run

# Check status
./deploy.sh status
```

#### Windows
```cmd
# Build the image
deploy.bat build

# Run the container
deploy.bat run

# Check status
deploy.bat status
```

## Configuration Options

### Environment Variables

| Variable | Default | Description | Production Value |
|----------|---------|-------------|------------------|
| `PORT` | `3000` | Server port | `3000` or `80` (with reverse proxy) |
| `HOST` | `127.0.0.1` | Server host | `0.0.0.0` |
| `RUST_LOG` | `info` | Log level | `info` or `warn` |
| `CORS_ORIGINS` | `http://localhost:5173` | Allowed CORS origins | `https://yourdomain.com` |
| `ENVIRONMENT` | `development` | Environment mode | `production` |

### Log Levels

- `error` - Only errors
- `warn` - Warnings and errors
- `info` - Info, warnings, and errors (recommended for production)
- `debug` - Debug info and above (not recommended for production)
- `trace` - All logs (not recommended for production)

## Security Considerations

### Network Security

1. **Use HTTPS**: Place the server behind a reverse proxy (nginx, Apache, Caddy) that handles SSL/TLS termination.

2. **Firewall**: Configure firewall rules to only allow necessary ports:
   ```bash
   # Allow only port 80 and 443
   sudo ufw allow 80/tcp
   sudo ufw allow 443/tcp
   sudo ufw deny 3000/tcp
   ```

3. **Private Network**: Run the server on a private network when possible.

### Container Security

1. **Non-root User**: The container runs as a non-root user by default.

2. **Minimal Image**: Based on Debian slim for minimal attack surface.

3. **Read-only Filesystem**: Consider running with read-only filesystem:
   ```yaml
   services:
     satisflow-server:
       read_only: true
       tmpfs:
         - /tmp
   ```

### CORS Configuration

In production, restrict CORS origins to your actual domain:
```env
CORS_ORIGINS=https://app.yourdomain.com,https://admin.yourdomain.com
```

## Monitoring

### Health Checks

The server provides a health check endpoint:
```bash
curl https://yourdomain.com/health
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

Check container health:
```bash
docker ps --format "table {{.Names}}\t{{.Status}}"
```

### Logging

#### Production Logs (JSON format)
```json
{
  "timestamp": "2023-10-20T12:00:00.000Z",
  "level": "info",
  "message": "Satisflow server listening on 0.0.0.0:3000 in production mode",
  "target": "satisflow_server"
}
```

#### Log Collection

For log aggregation, consider:
- ELK Stack (Elasticsearch, Logstash, Kibana)
- Fluentd/Fluent Bit
- Docker logging drivers

## Performance Optimization

### Resource Limits

Set appropriate resource limits in Docker:
```yaml
services:
  satisflow-server:
    deploy:
      resources:
        limits:
          cpus: '0.5'
          memory: 512M
        reservations:
          cpus: '0.25'
          memory: 256M
```

### Reverse Proxy Configuration

#### Nginx Example
```nginx
server {
    listen 80;
    server_name yourdomain.com;
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name yourdomain.com;
    
    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;
    
    location / {
        proxy_pass http://localhost:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

## Scaling

### Horizontal Scaling

For high availability, run multiple instances behind a load balancer:

```yaml
version: '3.8'
services:
  satisflow-server:
    image: satisflow-server:latest
    deploy:
      replicas: 3
    # ... other config
```

### Load Balancer

Use a load balancer (nginx, HAProxy, cloud load balancer) to distribute traffic.

## Backup and Recovery

### Data Persistence

The server is stateless, but if you add persistence:
```yaml
services:
  satisflow-server:
    volumes:
      - ./data:/app/data
      - ./logs:/app/logs
```

### Backup Strategy

1. **Configuration Backup**: Backup `.env` and `docker-compose.yml`
2. **Image Backup**: Tag and push images to registry
3. **Log Backup**: Rotate and archive logs

## Troubleshooting

### Common Issues

1. **Container won't start**:
   ```bash
   docker logs satisflow-server-prod
   ```

2. **Health check failing**:
   ```bash
   curl -v http://localhost:3000/health
   ```

3. **CORS errors**:
   - Check `CORS_ORIGINS` environment variable
   - Verify request origin matches allowed origins

4. **High memory usage**:
   - Check for memory leaks
   - Monitor with `docker stats`

### Debug Mode

For debugging, temporarily enable debug logging:
```bash
docker run -e RUST_LOG=debug satisflow-server:latest
```

## Maintenance

### Updates

1. Update the image:
   ```bash
   docker pull satisflow-server:latest
   ```

2. Redeploy:
   ```bash
   docker-compose up -d --force-recreate
   ```

### Log Rotation

Configure log rotation in Docker:
```yaml
services:
  satisflow-server:
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"
```

## Support

For issues:
1. Check logs: `docker logs <container>`
2. Verify configuration
3. Check health endpoint
4. Review this guide

## Security Best Practices

1. **Regular Updates**: Keep Docker and images updated
2. **Minimal Exposure**: Only expose necessary ports
3. **HTTPS**: Always use HTTPS in production
4. **Monitoring**: Monitor logs and metrics
5. **Backup**: Regular configuration backups
6. **Testing**: Test updates in staging first