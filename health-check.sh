#!/bin/bash

# Thai Energy Trading Blockchain - System Health Check
# This script monitors system health and provides detailed status reports

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Icons
CHECK="✅"
ERROR="❌"
WARNING="⚠️"
INFO="ℹ️"

echo -e "${BLUE}🇹🇭 Thai Energy Trading Blockchain - System Health Check${NC}"
echo -e "${BLUE}============================================================${NC}"

# Function to check service health
check_service_health() {
    local service=$1
    local port=$2
    local protocol=${3:-http}
    
    echo -e "${YELLOW}Checking ${service}...${NC}"
    
    if docker-compose ps ${service} | grep -q "Up"; then
        echo -e "  ${CHECK} Container Status: ${GREEN}Running${NC}"
        
        # Check if service is responding on port
        if [ ! -z "$port" ]; then
            if nc -z localhost $port 2>/dev/null; then
                echo -e "  ${CHECK} Port ${port}: ${GREEN}Open${NC}"
                
                # Additional HTTP/HTTPS checks
                if [[ "$protocol" == "http" ]]; then
                    if curl -sf http://localhost:$port/health >/dev/null 2>&1; then
                        echo -e "  ${CHECK} Health Endpoint: ${GREEN}Healthy${NC}"
                    else
                        echo -e "  ${WARNING} Health Endpoint: ${YELLOW}No response${NC}"
                    fi
                elif [[ "$protocol" == "https" ]]; then
                    if curl -sfk https://localhost:$port/health >/dev/null 2>&1; then
                        echo -e "  ${CHECK} Health Endpoint: ${GREEN}Healthy${NC}"
                    else
                        echo -e "  ${WARNING} Health Endpoint: ${YELLOW}No response${NC}"
                    fi
                fi
            else
                echo -e "  ${ERROR} Port ${port}: ${RED}Closed${NC}"
            fi
        fi
    else
        echo -e "  ${ERROR} Container Status: ${RED}Not Running${NC}"
    fi
    echo ""
}

# Function to check database health
check_database_health() {
    echo -e "${YELLOW}Checking PostgreSQL Database...${NC}"
    
    if docker-compose exec -T postgres pg_isready -U thai_energy >/dev/null 2>&1; then
        echo -e "  ${CHECK} Database Status: ${GREEN}Ready${NC}"
        
        # Check database size
        DB_SIZE=$(docker-compose exec -T postgres psql -U thai_energy -d thai_energy_db -t -c "SELECT pg_size_pretty(pg_database_size('thai_energy_db'));" 2>/dev/null | xargs)
        echo -e "  ${INFO} Database Size: ${CYAN}${DB_SIZE}${NC}"
        
        # Check connection count
        CONNECTIONS=$(docker-compose exec -T postgres psql -U thai_energy -d thai_energy_db -t -c "SELECT count(*) FROM pg_stat_activity WHERE datname = 'thai_energy_db';" 2>/dev/null | xargs)
        echo -e "  ${INFO} Active Connections: ${CYAN}${CONNECTIONS}${NC}"
        
        # Check table counts
        TABLES=$(docker-compose exec -T postgres psql -U thai_energy -d thai_energy_db -t -c "SELECT count(*) FROM information_schema.tables WHERE table_schema = 'public';" 2>/dev/null | xargs)
        echo -e "  ${INFO} Tables: ${CYAN}${TABLES}${NC}"
        
        # Check recent transactions
        RECENT_TXN=$(docker-compose exec -T postgres psql -U thai_energy -d thai_energy_db -t -c "SELECT count(*) FROM transactions WHERE created_at > NOW() - INTERVAL '1 hour';" 2>/dev/null | xargs || echo "0")
        echo -e "  ${INFO} Recent Transactions (1h): ${CYAN}${RECENT_TXN}${NC}"
        
    else
        echo -e "  ${ERROR} Database Status: ${RED}Not Ready${NC}"
    fi
    echo ""
}

# Function to check Redis health
check_redis_health() {
    echo -e "${YELLOW}Checking Redis Cache...${NC}"
    
    if docker-compose exec -T redis redis-cli ping >/dev/null 2>&1; then
        echo -e "  ${CHECK} Redis Status: ${GREEN}Running${NC}"
        
        # Check Redis memory usage
        REDIS_MEM=$(docker-compose exec -T redis redis-cli info memory | grep "used_memory_human" | cut -d: -f2 | tr -d '\r')
        echo -e "  ${INFO} Memory Usage: ${CYAN}${REDIS_MEM}${NC}"
        
        # Check key count
        KEY_COUNT=$(docker-compose exec -T redis redis-cli dbsize 2>/dev/null | xargs)
        echo -e "  ${INFO} Keys: ${CYAN}${KEY_COUNT}${NC}"
        
        # Check hit rate
        HITS=$(docker-compose exec -T redis redis-cli info stats | grep "keyspace_hits" | cut -d: -f2 | tr -d '\r')
        MISSES=$(docker-compose exec -T redis redis-cli info stats | grep "keyspace_misses" | cut -d: -f2 | tr -d '\r')
        if [ "$HITS" -gt 0 ] || [ "$MISSES" -gt 0 ]; then
            HIT_RATE=$(echo "scale=2; $HITS / ($HITS + $MISSES) * 100" | bc -l 2>/dev/null || echo "0")
            echo -e "  ${INFO} Hit Rate: ${CYAN}${HIT_RATE}%${NC}"
        fi
        
    else
        echo -e "  ${ERROR} Redis Status: ${RED}Not Running${NC}"
    fi
    echo ""
}

# Function to check system resources
check_system_resources() {
    echo -e "${YELLOW}Checking System Resources...${NC}"
    
    # CPU usage
    CPU_USAGE=$(docker stats --no-stream --format "table {{.CPUPerc}}" | tail -n +2 | sed 's/%//' | awk '{sum+=$1} END {print sum}')
    if (( $(echo "$CPU_USAGE > 80" | bc -l) )); then
        echo -e "  ${ERROR} CPU Usage: ${RED}${CPU_USAGE}%${NC}"
    elif (( $(echo "$CPU_USAGE > 60" | bc -l) )); then
        echo -e "  ${WARNING} CPU Usage: ${YELLOW}${CPU_USAGE}%${NC}"
    else
        echo -e "  ${CHECK} CPU Usage: ${GREEN}${CPU_USAGE}%${NC}"
    fi
    
    # Memory usage
    MEMORY_USAGE=$(docker stats --no-stream --format "table {{.MemPerc}}" | tail -n +2 | sed 's/%//' | awk '{sum+=$1} END {print sum}')
    if (( $(echo "$MEMORY_USAGE > 85" | bc -l) )); then
        echo -e "  ${ERROR} Memory Usage: ${RED}${MEMORY_USAGE}%${NC}"
    elif (( $(echo "$MEMORY_USAGE > 70" | bc -l) )); then
        echo -e "  ${WARNING} Memory Usage: ${YELLOW}${MEMORY_USAGE}%${NC}"
    else
        echo -e "  ${CHECK} Memory Usage: ${GREEN}${MEMORY_USAGE}%${NC}"
    fi
    
    # Disk usage
    DISK_USAGE=$(df -h / | tail -1 | awk '{print $5}' | sed 's/%//')
    if [ "$DISK_USAGE" -gt 90 ]; then
        echo -e "  ${ERROR} Disk Usage: ${RED}${DISK_USAGE}%${NC}"
    elif [ "$DISK_USAGE" -gt 80 ]; then
        echo -e "  ${WARNING} Disk Usage: ${YELLOW}${DISK_USAGE}%${NC}"
    else
        echo -e "  ${CHECK} Disk Usage: ${GREEN}${DISK_USAGE}%${NC}"
    fi
    
    # Docker volume usage
    DOCKER_SPACE=$(docker system df --format "table {{.Size}}" | tail -n +2 | head -1)
    echo -e "  ${INFO} Docker Space: ${CYAN}${DOCKER_SPACE}${NC}"
    
    echo ""
}

# Function to check network connectivity
check_network_connectivity() {
    echo -e "${YELLOW}Checking Network Connectivity...${NC}"
    
    # Check external connectivity
    if curl -sf https://google.com >/dev/null 2>&1; then
        echo -e "  ${CHECK} External Connectivity: ${GREEN}OK${NC}"
    else
        echo -e "  ${ERROR} External Connectivity: ${RED}Failed${NC}"
    fi
    
    # Check internal container networking
    if docker-compose exec -T thai-energy-blockchain ping -c 1 postgres >/dev/null 2>&1; then
        echo -e "  ${CHECK} Internal Networking: ${GREEN}OK${NC}"
    else
        echo -e "  ${ERROR} Internal Networking: ${RED}Failed${NC}"
    fi
    
    echo ""
}

# Function to check logs for errors
check_logs_for_errors() {
    echo -e "${YELLOW}Checking Recent Logs for Errors...${NC}"
    
    # Check for errors in the last 100 lines
    ERROR_COUNT=$(docker-compose logs --tail=100 2>/dev/null | grep -i "error\|exception\|failed\|panic" | wc -l)
    
    if [ "$ERROR_COUNT" -gt 10 ]; then
        echo -e "  ${ERROR} Recent Errors: ${RED}${ERROR_COUNT} errors found${NC}"
    elif [ "$ERROR_COUNT" -gt 0 ]; then
        echo -e "  ${WARNING} Recent Errors: ${YELLOW}${ERROR_COUNT} errors found${NC}"
    else
        echo -e "  ${CHECK} Recent Errors: ${GREEN}No errors found${NC}"
    fi
    
    echo ""
}

# Function to check SSL certificates
check_ssl_certificates() {
    echo -e "${YELLOW}Checking SSL Certificates...${NC}"
    
    if [ -f "docker/nginx/ssl/cert.pem" ]; then
        CERT_EXPIRY=$(openssl x509 -in docker/nginx/ssl/cert.pem -noout -dates | grep "notAfter" | cut -d= -f2)
        DAYS_UNTIL_EXPIRY=$(echo $(( ($(date -d "$CERT_EXPIRY" +%s) - $(date +%s)) / 86400 )))
        
        if [ "$DAYS_UNTIL_EXPIRY" -lt 7 ]; then
            echo -e "  ${ERROR} SSL Certificate: ${RED}Expires in ${DAYS_UNTIL_EXPIRY} days${NC}"
        elif [ "$DAYS_UNTIL_EXPIRY" -lt 30 ]; then
            echo -e "  ${WARNING} SSL Certificate: ${YELLOW}Expires in ${DAYS_UNTIL_EXPIRY} days${NC}"
        else
            echo -e "  ${CHECK} SSL Certificate: ${GREEN}Valid for ${DAYS_UNTIL_EXPIRY} days${NC}"
        fi
    else
        echo -e "  ${ERROR} SSL Certificate: ${RED}Not found${NC}"
    fi
    
    echo ""
}

# Function to generate summary report
generate_summary() {
    echo -e "${BLUE}📊 Health Check Summary${NC}"
    echo -e "${BLUE}========================${NC}"
    
    # Count running services
    RUNNING_SERVICES=$(docker-compose ps --services --filter "status=running" | wc -l)
    TOTAL_SERVICES=$(docker-compose ps --services | wc -l)
    
    if [ "$RUNNING_SERVICES" -eq "$TOTAL_SERVICES" ]; then
        echo -e "  ${CHECK} Services: ${GREEN}${RUNNING_SERVICES}/${TOTAL_SERVICES} running${NC}"
    else
        echo -e "  ${ERROR} Services: ${RED}${RUNNING_SERVICES}/${TOTAL_SERVICES} running${NC}"
    fi
    
    # Overall health status
    if [ "$RUNNING_SERVICES" -eq "$TOTAL_SERVICES" ]; then
        echo -e "  ${CHECK} Overall Status: ${GREEN}Healthy${NC}"
    else
        echo -e "  ${ERROR} Overall Status: ${RED}Unhealthy${NC}"
    fi
    
    echo ""
    echo -e "${BLUE}🔧 Recommendations:${NC}"
    echo -e "  • Monitor resource usage regularly"
    echo -e "  • Check logs for errors: ./deploy.sh logs"
    echo -e "  • Update SSL certificates before expiry"
    echo -e "  • Backup database regularly"
    echo -e "  • Keep system updated"
    echo ""
}

# Main health check function
main() {
    case "${1:-full}" in
        "full")
            check_service_health "thai-energy-blockchain" "3000" "http"
            check_service_health "nginx" "80" "http"
            check_service_health "postgres" "5432"
            check_service_health "redis" "6379"
            check_service_health "prometheus" "9090" "http"
            check_service_health "grafana" "3000" "http"
            check_database_health
            check_redis_health
            check_system_resources
            check_network_connectivity
            check_logs_for_errors
            check_ssl_certificates
            generate_summary
            ;;
        "quick")
            check_service_health "thai-energy-blockchain" "3000" "http"
            check_service_health "postgres" "5432"
            check_service_health "redis" "6379"
            check_system_resources
            generate_summary
            ;;
        "services")
            check_service_health "thai-energy-blockchain" "3000" "http"
            check_service_health "nginx" "80" "http"
            check_service_health "postgres" "5432"
            check_service_health "redis" "6379"
            check_service_health "prometheus" "9090" "http"
            check_service_health "grafana" "3000" "http"
            ;;
        "resources")
            check_system_resources
            ;;
        "database")
            check_database_health
            ;;
        "help")
            echo "Usage: $0 {full|quick|services|resources|database|help}"
            echo ""
            echo "Commands:"
            echo "  full      - Complete health check (default)"
            echo "  quick     - Quick health check"
            echo "  services  - Check services only"
            echo "  resources - Check system resources"
            echo "  database  - Check database health"
            echo "  help      - Show this help"
            ;;
        *)
            echo "Invalid command. Use '$0 help' for usage information."
            exit 1
            ;;
    esac
}

# Check if Docker and Docker Compose are available
if ! command -v docker &> /dev/null || ! command -v docker-compose &> /dev/null; then
    echo -e "${RED}❌ Docker or Docker Compose not found. Please install them first.${NC}"
    exit 1
fi

# Check if running in project directory
if [ ! -f "docker-compose.yml" ]; then
    echo -e "${RED}❌ docker-compose.yml not found. Please run from project root directory.${NC}"
    exit 1
fi

# Install bc if not available (for calculations)
if ! command -v bc &> /dev/null; then
    echo -e "${YELLOW}⚠️  Installing bc for calculations...${NC}"
    if [[ "$OSTYPE" == "darwin"* ]]; then
        brew install bc 2>/dev/null || echo "Please install bc manually: brew install bc"
    else
        sudo apt-get update && sudo apt-get install -y bc 2>/dev/null || echo "Please install bc manually"
    fi
fi

# Run main function
main "$@"
