#!/bin/bash

# Thai Energy Trading Blockchain - Docker Deployment Preparation
# This script prepares the project for Docker deployment

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}🇹🇭 Thai Energy Trading Blockchain - Deployment Preparation${NC}"
echo -e "${BLUE}================================================================${NC}"

echo -e "${YELLOW}📋 Checking deployment prerequisites...${NC}"

# Check if Rust project compiles
echo -e "${BLUE}Building Rust project...${NC}"
cargo build --release
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Rust build successful${NC}"
else
    echo -e "${RED}❌ Rust build failed${NC}"
    exit 1
fi

# Check essential files exist
echo -e "${BLUE}Checking essential files...${NC}"

required_files=(
    "Dockerfile"
    "docker-compose.yml"
    ".env"
    "config/production.toml"
    "deploy.sh"
)

for file in "${required_files[@]}"; do
    if [ -f "$file" ]; then
        echo -e "${GREEN}✅ $file exists${NC}"
    else
        echo -e "${RED}❌ $file missing${NC}"
        exit 1
    fi
done

# Check Docker configurations
echo -e "${BLUE}Checking Docker configurations...${NC}"

docker_configs=(
    "docker/nginx/nginx.conf"
    "docker/postgres/init.sql" 
    "docker/prometheus/prometheus.yml"
)

for config in "${docker_configs[@]}"; do
    if [ -f "$config" ]; then
        echo -e "${GREEN}✅ $config exists${NC}"
    else
        echo -e "${YELLOW}⚠️  $config missing (optional)${NC}"
    fi
done

# Check environment variables
echo -e "${BLUE}Validating environment configuration...${NC}"
if grep -q "your_secure" .env; then
    echo -e "${YELLOW}⚠️  Please update placeholder values in .env file${NC}"
else
    echo -e "${GREEN}✅ Environment variables configured${NC}"
fi

echo ""
echo -e "${GREEN}🎉 Deployment preparation complete!${NC}"
echo -e "${BLUE}Next steps:${NC}"
echo -e "1. Start Docker: ${YELLOW}Docker Desktop or 'sudo systemctl start docker'${NC}"
echo -e "2. Deploy: ${YELLOW}./deploy.sh${NC}"
echo -e "3. Or manually: ${YELLOW}docker-compose up -d${NC}"
echo ""
echo -e "${BLUE}Access points after deployment:${NC}"
echo -e "• Main Application: ${CYAN}http://localhost:8080${NC}"
echo -e "• Grafana Dashboard: ${CYAN}http://localhost:3000${NC}"
echo -e "• Prometheus Metrics: ${CYAN}http://localhost:9090${NC}"
echo -e "• WebSocket API: ${CYAN}ws://localhost:9944${NC}"
