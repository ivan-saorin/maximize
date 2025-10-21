# 🐳 Docker Deployment Guide

Complete guide for running Maximize in Docker.

## Quick Start

```bash
# Build and start
docker-compose up -d

# Attach for authentication
docker attach maximize

# In the CLI:
# Select: 2 (Login / Re-authenticate)
# Complete OAuth flow
# Ctrl+P, Ctrl+Q to detach (keep running)
```

## Building the Image

### Using Docker Compose (Recommended)

```bash
# Build the image
docker-compose build

# Start the container
docker-compose up -d

# View logs
docker-compose logs -f

# Stop
docker-compose down
```

### Using Docker Directly

```bash
# Build
docker build -t maximize:latest .

# Run
docker run -d \
  --name maximize \
  -p 8081:8081 \
  -v maximize-tokens:/app/.maximize \
  maximize:latest
```

## Authentication in Docker

Since OAuth requires interactive browser access, you need to attach to the container:

### Method 1: Docker Compose (Easiest)

```bash
# Start container
docker-compose up -d

# Attach to interactive CLI
docker attach maximize

# In the menu:
# 1. Select option 2 (Login)
# 2. The OAuth URL will be displayed
# 3. Open URL in your browser
# 4. Complete authentication
# 5. Paste code back into CLI

# Detach without stopping container
# Press: Ctrl+P, then Ctrl+Q
```

### Method 2: Docker Exec

```bash
# Start container
docker-compose up -d

# Run authentication in container
docker exec -it maximize /app/maximize

# Complete OAuth flow
# Exit CLI when done
```

### Method 3: Pre-authenticate

```bash
# Run maximize locally first
./target/release/maximize
# Complete authentication (option 2)
# This creates ~/.maximize/tokens.json

# Copy tokens to Docker volume
docker run -v maximize-tokens:/tokens alpine sh -c \
  "cp ~/.maximize/tokens.json /tokens/"

# Start container - already authenticated!
docker-compose up -d
```

## Configuration

### Using Environment Variables

```yaml
# docker-compose.yml
environment:
  - PORT=8081
  - BIND_ADDRESS=0.0.0.0
  - LOG_LEVEL=debug
  - DEFAULT_MODEL=l
  - TOKEN_FILE=/app/.maximize/tokens.json
```

### Using Config File

```bash
# Create config.json locally
cat > config.json << 'EOF'
{
  "server": {
    "port": 8081,
    "bind_address": "0.0.0.0",
    "log_level": "info"
  },
  "models": {
    "default": "l"
  }
}
EOF

# Mount in docker-compose.yml (already configured)
volumes:
  - ./config.json:/app/config.json:ro
```

## Volumes

### Token Storage Volume

```bash
# Inspect volume
docker volume inspect maximize-tokens

# Backup tokens
docker run --rm \
  -v maximize-tokens:/source:ro \
  -v $(pwd):/backup \
  alpine tar czf /backup/tokens-backup.tar.gz -C /source .

# Restore tokens
docker run --rm \
  -v maximize-tokens:/target \
  -v $(pwd):/backup \
  alpine tar xzf /backup/tokens-backup.tar.gz -C /target
```

## Multi-Stage Build

The Dockerfile uses multi-stage builds for optimal image size:

1. **Builder stage**: Compiles Rust application
2. **Runtime stage**: Minimal Debian slim with only runtime deps

Result: ~50MB final image vs ~2GB if including build tools.

## Production Deployment

### Docker Compose Production

```yaml
version: '3.8'

services:
  maximize:
    image: maximize:latest
    container_name: maximize-prod
    ports:
      - "8081:8081"
    volumes:
      - /var/maximize/tokens:/app/.maximize
      - /var/maximize/config.json:/app/config.json:ro
    environment:
      - PORT=8081
      - BIND_ADDRESS=0.0.0.0
      - LOG_LEVEL=info
      - RUST_LOG=info
    restart: always
    security_opt:
      - no-new-privileges:true
    read_only: true
    tmpfs:
      - /tmp
    networks:
      - maximize-network

networks:
  maximize-network:
    driver: bridge
```

### Behind Reverse Proxy

#### Nginx

```nginx
upstream maximize {
    server localhost:8081;
}

server {
    listen 80;
    server_name maximize.example.com;

    location / {
        proxy_pass http://maximize;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
        
        # For streaming responses
        proxy_buffering off;
        proxy_read_timeout 300s;
    }
}
```

#### Traefik

```yaml
labels:
  - "traefik.enable=true"
  - "traefik.http.routers.maximize.rule=Host(`maximize.example.com`)"
  - "traefik.http.services.maximize.loadbalancer.server.port=8081"
```

## Healthcheck

Add healthcheck to docker-compose.yml:

```yaml
healthcheck:
  test: ["CMD", "curl", "-f", "http://localhost:8081/healthz"]
  interval: 30s
  timeout: 10s
  retries: 3
  start_period: 40s
```

Or install curl in Dockerfile:

```dockerfile
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*
```

## Troubleshooting

### Container Exits Immediately

The CLI mode requires interactive terminal. Use one of these:

```bash
# Method 1: Keep container running
docker-compose up -d
docker attach maximize

# Method 2: Run in foreground
docker-compose up

# Method 3: Override entrypoint for server-only mode
# (if you've already authenticated)
docker run -d \
  -p 8081:8081 \
  -v maximize-tokens:/app/.maximize \
  maximize:latest \
  --bind 0.0.0.0
```

### Authentication Issues

```bash
# Check if tokens exist
docker exec maximize ls -la /app/.maximize/

# View token status
docker exec -it maximize /app/maximize
# Select option 4 (Show Token Status)

# Re-authenticate
docker exec -it maximize /app/maximize
# Select option 2 (Login)
```

### Permission Issues

```bash
# Fix token directory permissions
docker exec maximize chown -R $(id -u):$(id -g) /app/.maximize/
```

### View Logs

```bash
# Follow logs
docker-compose logs -f

# Last 100 lines
docker-compose logs --tail=100

# Specific service
docker logs maximize -f
```

### Port Already in Use

```bash
# Change port in docker-compose.yml
ports:
  - "8082:8081"  # Host:Container

# Or use environment variable
PORT=8082 docker-compose up -d
```

## Performance Tuning

### CPU Limits

```yaml
services:
  maximize:
    # ... other config
    deploy:
      resources:
        limits:
          cpus: '2'
          memory: 512M
        reservations:
          cpus: '1'
          memory: 256M
```

### Network Optimization

```bash
# Use host network for lowest latency (Linux only)
network_mode: "host"

# Note: You'll need to use host's IP and ports
```

## Kubernetes Deployment

<details>
<summary>Click to expand Kubernetes manifests</summary>

```yaml
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: maximize-tokens
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 1Gi
---
apiVersion: v1
kind: ConfigMap
metadata:
  name: maximize-config
data:
  config.json: |
    {
      "server": {
        "port": 8081,
        "bind_address": "0.0.0.0"
      },
      "models": {
        "default": "l"
      }
    }
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: maximize
spec:
  replicas: 1
  selector:
    matchLabels:
      app: maximize
  template:
    metadata:
      labels:
        app: maximize
    spec:
      containers:
      - name: maximize
        image: maximize:latest
        ports:
        - containerPort: 8081
        volumeMounts:
        - name: tokens
          mountPath: /app/.maximize
        - name: config
          mountPath: /app/config.json
          subPath: config.json
        env:
        - name: PORT
          value: "8081"
        - name: BIND_ADDRESS
          value: "0.0.0.0"
        - name: LOG_LEVEL
          value: "info"
      volumes:
      - name: tokens
        persistentVolumeClaim:
          claimName: maximize-tokens
      - name: config
        configMap:
          name: maximize-config
---
apiVersion: v1
kind: Service
metadata:
  name: maximize
spec:
  selector:
    app: maximize
  ports:
  - port: 8081
    targetPort: 8081
  type: LoadBalancer
```

</details>

## Best Practices

1. **Always use volumes** for token storage
2. **Enable restart policies** for production
3. **Use environment variables** for configuration
4. **Implement health checks** for monitoring
5. **Run as non-root** when possible
6. **Enable read-only filesystem** in production
7. **Use specific image tags** (not :latest) in production
8. **Backup token volume** regularly
9. **Monitor container logs** for issues
10. **Use reverse proxy** for TLS termination

## Security Considerations

1. **Network isolation**: Use Docker networks
2. **No privileged mode**: Never use --privileged
3. **Read-only root**: Use read_only: true
4. **Drop capabilities**: Use cap_drop
5. **Resource limits**: Set memory and CPU limits
6. **Secrets management**: Use Docker secrets for sensitive data
7. **Regular updates**: Rebuild image regularly for security patches

---

**Need help?** Open an issue with your Docker setup details and logs.
