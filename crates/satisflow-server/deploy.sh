#!/bin/bash

# Satisflow Server Deployment Script
# This script helps deploy the Satisflow server in production

set -e

# Configuration
IMAGE_NAME="satisflow-server"
CONTAINER_NAME="satisflow-server-prod"
PORT=${PORT:-3000}
HOST=${HOST:-0.0.0.0}
ENVIRONMENT=${ENVIRONMENT:-production}
RUST_LOG=${RUST_LOG:-info}
CORS_ORIGINS=${CORS_ORIGINS:-https://yourdomain.com}

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Helper functions
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    log_error "Docker is not installed. Please install Docker first."
    exit 1
fi

# Parse command line arguments
COMMAND=${1:-"help"}

case $COMMAND in
    build)
        log_info "Building Docker image..."
        docker build -t ${IMAGE_NAME}:latest -f crates/satisflow-server/Dockerfile .
        log_info "Build completed successfully!"
        ;;
    
    run)
        log_info "Starting container..."
        # Stop existing container if running
        if docker ps -a --format 'table {{.Names}}' | grep -q "^${CONTAINER_NAME}$"; then
            log_warn "Stopping existing container..."
            docker stop ${CONTAINER_NAME} || true
            docker rm ${CONTAINER_NAME} || true
        fi
        
        # Run new container
        docker run -d \
            --name ${CONTAINER_NAME} \
            -p ${PORT}:3000 \
            -e PORT=3000 \
            -e HOST=${HOST} \
            -e ENVIRONMENT=${ENVIRONMENT} \
            -e RUST_LOG=${RUST_LOG} \
            -e CORS_ORIGINS=${CORS_ORIGINS} \
            --restart unless-stopped \
            ${IMAGE_NAME}:latest
        
        log_info "Container started successfully!"
        log_info "Server is running on http://${HOST}:${PORT}"
        ;;
    
    stop)
        log_info "Stopping container..."
        if docker ps --format 'table {{.Names}}' | grep -q "^${CONTAINER_NAME}$"; then
            docker stop ${CONTAINER_NAME}
            log_info "Container stopped!"
        else
            log_warn "Container is not running."
        fi
        ;;
    
    restart)
        log_info "Restarting container..."
        $0 stop
        sleep 2
        $0 run
        ;;
    
    logs)
        log_info "Showing container logs..."
        docker logs -f ${CONTAINER_NAME}
        ;;
    
    status)
        log_info "Container status:"
        docker ps -a --filter name=${CONTAINER_NAME} --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}"
        ;;
    
    health)
        log_info "Checking health..."
        if curl -f http://localhost:${PORT}/health > /dev/null 2>&1; then
            log_info "Service is healthy!"
        else
            log_error "Service is unhealthy or not responding!"
            exit 1
        fi
        ;;
    
    clean)
        log_warn "Cleaning up..."
        docker stop ${CONTAINER_NAME} || true
        docker rm ${CONTAINER_NAME} || true
        docker rmi ${IMAGE_NAME}:latest || true
        log_info "Cleanup completed!"
        ;;
    
    help|*)
        echo "Satisflow Server Deployment Script"
        echo ""
        echo "Usage: $0 [COMMAND]"
        echo ""
        echo "Commands:"
        echo "  build    Build the Docker image"
        echo "  run      Run the container in production mode"
        echo "  stop     Stop the running container"
        echo "  restart  Restart the container"
        echo "  logs     Show container logs"
        echo "  status   Show container status"
        echo "  health   Check service health"
        echo "  clean    Remove container and image"
        echo "  help     Show this help message"
        echo ""
        echo "Environment Variables:"
        echo "  PORT           Server port (default: 3000)"
        echo "  HOST           Server host (default: 0.0.0.0)"
        echo "  ENVIRONMENT    Environment mode (default: production)"
        echo "  RUST_LOG       Log level (default: info)"
        echo "  CORS_ORIGINS   Allowed CORS origins (default: https://yourdomain.com)"
        echo ""
        echo "Examples:"
        echo "  $0 build                    # Build the image"
        echo "  $0 run                      # Run with defaults"
        echo "  PORT=8080 $0 run            # Run on port 8080"
        echo "  RUST_LOG=debug $0 run       # Run with debug logging"
        ;;
esac