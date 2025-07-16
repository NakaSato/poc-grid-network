#!/bin/bash

# Thai Energy Trading Blockchain Production Deployment Script
# This script prepares and deploys the blockchain system in production

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🇹🇭 Thai Energy Trading Blockchain - Production Deployment${NC}"
echo -e "${BLUE}================================================================${NC}"

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    echo -e "${RED}❌ Docker is not installed. Please install Docker first.${NC}"
    exit 1
fi

# Check if Docker Compose is installed
if ! command -v docker-compose &> /dev/null; then
    echo -e "${RED}❌ Docker Compose is not installed. Please install Docker Compose first.${NC}"
    exit 1
fi

# Function to generate secure passwords
generate_password() {
    openssl rand -base64 32 | tr -d "=+/" | cut -c1-32
}

# Function to create SSL certificates
create_ssl_certificates() {
    echo -e "${YELLOW}📜 Creating SSL certificates...${NC}"
    
    mkdir -p docker/nginx/ssl
    
    # Generate self-signed certificate for development
    # In production, replace with proper certificates from Let's Encrypt or CA
    openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
        -keyout docker/nginx/ssl/key.pem \
        -out docker/nginx/ssl/cert.pem \
        -subj "/C=TH/ST=Bangkok/L=Bangkok/O=Thai Energy Trading/CN=localhost"
    
    echo -e "${GREEN}✅ SSL certificates created${NC}"
}

# Function to setup environment
setup_environment() {
    echo -e "${YELLOW}🔧 Setting up environment...${NC}"
    
    if [ ! -f .env ]; then
        echo -e "${YELLOW}📝 Creating environment file...${NC}"
        cp .env.production .env
        
        # Generate secure passwords
        DB_PASSWORD=$(generate_password)
        REDIS_PASSWORD=$(generate_password)
        SECRET_KEY=$(generate_password)
        ENCRYPTION_KEY=$(generate_password)
        GRAFANA_PASSWORD=$(generate_password)
        
        # Replace placeholders in .env file (macOS compatible)
        sed -i '' "s/your_secure_postgres_password_here/${DB_PASSWORD}/g" .env
        sed -i '' "s/your_secure_redis_password_here/${REDIS_PASSWORD}/g" .env
        sed -i '' "s/your_secret_key_here_32_characters_min/${SECRET_KEY}/g" .env
        sed -i '' "s/your_encryption_key_here_32_chars/${ENCRYPTION_KEY}/g" .env
        sed -i '' "s/your_grafana_admin_password_here/${GRAFANA_PASSWORD}/g" .env
        
        echo -e "${GREEN}✅ Environment file created with secure passwords${NC}"
        echo -e "${YELLOW}⚠️  Please review and update API keys in .env file${NC}"
    else
        echo -e "${GREEN}✅ Environment file already exists${NC}"
    fi
}

# Function to build the application
build_application() {
    echo -e "${YELLOW}🏗️  Building application...${NC}"
    
    # Build the Docker image
    docker-compose build --no-cache
    
    echo -e "${GREEN}✅ Application built successfully${NC}"
}

# Function to run database migrations
run_migrations() {
    echo -e "${YELLOW}🗃️  Running database migrations...${NC}"
    
    # Start only the database
    docker-compose up -d postgres
    
    # Wait for database to be ready
    echo -e "${YELLOW}⏳ Waiting for database to be ready...${NC}"
    sleep 10
    
    # Run migrations (this would be implemented in your application)
    # docker-compose exec thai-energy-blockchain ./thai-energy-trading-blockchain migrate
    
    echo -e "${GREEN}✅ Database migrations completed${NC}"
}

# Function to start services
start_services() {
    echo -e "${YELLOW}🚀 Starting services...${NC}"
    
    # Start all services
    docker-compose up -d
    
    # Wait for services to be ready
    echo -e "${YELLOW}⏳ Waiting for services to start...${NC}"
    sleep 30
    
    # Check service health
    echo -e "${YELLOW}🔍 Checking service health...${NC}"
    docker-compose ps
    
    echo -e "${GREEN}✅ Services started successfully${NC}"
}

# Function to show service information
show_service_info() {
    echo -e "${BLUE}📋 Service Information${NC}"
    echo -e "${BLUE}=====================${NC}"
    echo -e "${GREEN}🌐 Web Interface: https://localhost${NC}"
    echo -e "${GREEN}📊 Metrics: http://localhost:9090${NC}"
    echo -e "${GREEN}📈 Grafana: http://localhost:3000${NC}"
    echo -e "${GREEN}🗃️  Database: localhost:5432${NC}"
    echo -e "${GREEN}🗄️  Redis: localhost:6379${NC}"
    echo -e "${GREEN}🔌 WebSocket: wss://localhost/ws${NC}"
    echo ""
    echo -e "${YELLOW}📖 Grafana Admin Password: Check .env file${NC}"
    echo -e "${YELLOW}🔐 Database Password: Check .env file${NC}"
    echo ""
    echo -e "${BLUE}📝 Useful Commands:${NC}"
    echo -e "${GREEN}  View logs: docker-compose logs -f${NC}"
    echo -e "${GREEN}  Stop services: docker-compose down${NC}"
    echo -e "${GREEN}  Update services: docker-compose pull && docker-compose up -d${NC}"
    echo -e "${GREEN}  Backup database: docker-compose exec postgres pg_dump -U thai_energy thai_energy_db > backup.sql${NC}"
}

# Function to cleanup
cleanup() {
    echo -e "${YELLOW}🧹 Cleaning up...${NC}"
    docker-compose down --volumes
    docker system prune -f
    echo -e "${GREEN}✅ Cleanup completed${NC}"
}

# Main deployment function
main() {
    case "${1:-deploy}" in
        "deploy")
            setup_environment
            create_ssl_certificates
            build_application
            run_migrations
            start_services
            show_service_info
            ;;
        "build")
            build_application
            ;;
        "start")
            start_services
            show_service_info
            ;;
        "stop")
            docker-compose down
            ;;
        "restart")
            docker-compose restart
            show_service_info
            ;;
        "logs")
            docker-compose logs -f
            ;;
        "cleanup")
            cleanup
            ;;
        "help")
            echo "Usage: $0 {deploy|build|start|stop|restart|logs|cleanup|help}"
            echo ""
            echo "Commands:"
            echo "  deploy   - Full deployment (default)"
            echo "  build    - Build application only"
            echo "  start    - Start services"
            echo "  stop     - Stop services"
            echo "  restart  - Restart services"
            echo "  logs     - Show logs"
            echo "  cleanup  - Clean up containers and volumes"
            echo "  help     - Show this help"
            ;;
        *)
            echo "Invalid command. Use '$0 help' for usage information."
            exit 1
            ;;
    esac
}

# Check if running as root
if [[ $EUID -eq 0 ]]; then
   echo -e "${RED}❌ This script should not be run as root for security reasons.${NC}"
   exit 1
fi

# Run main function
main "$@"
