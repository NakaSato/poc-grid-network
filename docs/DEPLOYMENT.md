# Thai Energy Trading Blockchain - Production Deployment

🇹🇭 **Thai Energy Trading Blockchain** - A secure, scalable blockchain system for energy trading in Thailand.

## 📋 Table of Contents

- [Overview](#overview)
- [Architecture](#architecture)
- [Prerequisites](#prerequisites)
- [Quick Start](#quick-start)
- [Configuration](#configuration)
- [Monitoring](#monitoring)
- [Security](#security)
- [Maintenance](#maintenance)
- [Troubleshooting](#troubleshooting)

## 🎯 Overview

This production deployment includes:

- **Blockchain Application**: Main Thai Energy Trading Blockchain service
- **Database**: PostgreSQL 15 with optimized configuration
- **Cache**: Redis with persistence
- **Reverse Proxy**: Nginx with SSL termination and rate limiting
- **Monitoring**: Prometheus + Grafana stack
- **Security**: SSL/TLS encryption, rate limiting, secure passwords

## 🏗️ Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Load Balancer │    │      Nginx      │    │   Blockchain    │
│   (External)    │───▶│  Reverse Proxy  │───▶│   Application   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                                │                        │
                                │                        ▼
                                │               ┌─────────────────┐
                                │               │   PostgreSQL    │
                                │               │    Database     │
                                │               └─────────────────┘
                                │                        │
                                │                        ▼
                                │               ┌─────────────────┐
                                │               │      Redis      │
                                │               │      Cache      │
                                │               └─────────────────┘
                                │
                                ▼
                       ┌─────────────────┐    ┌─────────────────┐
                       │   Prometheus    │───▶│     Grafana     │
                       │    Metrics      │    │   Dashboard     │
                       └─────────────────┘    └─────────────────┘
```

## 🔧 Prerequisites

- **Docker**: >= 20.10.0
- **Docker Compose**: >= 2.0.0
- **Operating System**: Linux (Ubuntu 20.04+ recommended)
- **Memory**: 4GB+ RAM recommended
- **Storage**: 20GB+ available space
- **Network**: Ports 80, 443, 3000, 5432, 6379, 9090 available

## 🚀 Quick Start

### 1. Clone and Setup

```bash
git clone <repository-url>
cd poc-simple-net
```

### 2. Deploy with One Command

```bash
./deploy.sh
```

This will:
- Generate secure passwords
- Create SSL certificates
- Build the application
- Start all services
- Show service information

### 3. Access Services

- **Web Interface**: https://localhost
- **Grafana Dashboard**: http://localhost:3000
- **Prometheus Metrics**: http://localhost:9090

## ⚙️ Configuration

### Environment Variables

Edit `.env` file to customize:

```bash
# Database Configuration
POSTGRES_DB=thai_energy_db
POSTGRES_USER=thai_energy
POSTGRES_PASSWORD=your_secure_password

# Redis Configuration
REDIS_PASSWORD=your_redis_password

# Application Configuration
SECRET_KEY=your_32_character_secret_key
ENCRYPTION_KEY=your_32_character_encryption_key

# External APIs
ENERGY_AUTHORITY_API_KEY=your_api_key
REGULATORY_API_KEY=your_api_key
```

### SSL Certificates

For production, replace self-signed certificates:

```bash
# Place your certificates in:
docker/nginx/ssl/cert.pem
docker/nginx/ssl/key.pem
```

### Database Optimization

Edit `docker/postgres/postgresql.conf` for production tuning:

```sql
-- Example optimizations
shared_buffers = 256MB
effective_cache_size = 1GB
work_mem = 4MB
maintenance_work_mem = 64MB
```

## 📊 Monitoring

### Grafana Dashboards

Access Grafana at http://localhost:3000

**Default Login**:
- Username: `admin`
- Password: Check `.env` file

### Key Metrics

- **Blockchain Performance**: Transaction throughput, block time
- **System Resources**: CPU, memory, disk usage
- **Database**: Query performance, connection counts
- **Network**: Request rates, response times

### Alerts

Configure alerts in Grafana for:
- High CPU usage (>80%)
- Memory usage (>85%)
- Database connection errors
- Transaction failures

## 🔐 Security

### SSL/TLS Configuration

```nginx
# Strong SSL configuration in nginx.conf
ssl_protocols TLSv1.2 TLSv1.3;
ssl_ciphers ECDHE-RSA-AES128-GCM-SHA256:ECDHE-RSA-AES256-GCM-SHA384;
ssl_prefer_server_ciphers off;
```

### Rate Limiting

```nginx
# Rate limiting configuration
limit_req_zone $binary_remote_addr zone=api:10m rate=10r/s;
limit_req zone=api burst=20 nodelay;
```

### Database Security

- Encrypted connections
- User privilege separation
- Regular security updates
- Backup encryption

## 🛠️ Maintenance

### Daily Operations

```bash
# View logs
./deploy.sh logs

# Restart services
./deploy.sh restart

# Check service health
docker-compose ps
```

### Database Backup

```bash
# Create backup
docker-compose exec postgres pg_dump -U thai_energy thai_energy_db > backup_$(date +%Y%m%d).sql

# Restore backup
docker-compose exec -T postgres psql -U thai_energy thai_energy_db < backup_20240101.sql
```

### Updates

```bash
# Update application
git pull
docker-compose build --no-cache
docker-compose up -d
```

### Scaling

```bash
# Scale blockchain service
docker-compose up -d --scale thai-energy-blockchain=3
```

## 📝 Management Commands

### Deployment Script

```bash
./deploy.sh deploy    # Full deployment
./deploy.sh build     # Build only
./deploy.sh start     # Start services
./deploy.sh stop      # Stop services
./deploy.sh restart   # Restart services
./deploy.sh logs      # Show logs
./deploy.sh cleanup   # Clean up
./deploy.sh help      # Show help
```

### Docker Compose Commands

```bash
# Start services
docker-compose up -d

# Stop services
docker-compose down

# View logs
docker-compose logs -f [service-name]

# Execute commands in containers
docker-compose exec thai-energy-blockchain bash
```

## 🐛 Troubleshooting

### Common Issues

#### Service Won't Start

```bash
# Check service status
docker-compose ps

# View service logs
docker-compose logs [service-name]

# Check resource usage
docker stats
```

#### Database Connection Issues

```bash
# Check database logs
docker-compose logs postgres

# Test connection
docker-compose exec postgres psql -U thai_energy -d thai_energy_db -c "SELECT 1;"
```

#### SSL Certificate Issues

```bash
# Regenerate certificates
./deploy.sh deploy

# Check certificate validity
openssl x509 -in docker/nginx/ssl/cert.pem -text -noout
```

### Performance Issues

```bash
# Check resource usage
docker stats

# Optimize database
docker-compose exec postgres psql -U thai_energy -d thai_energy_db -c "VACUUM ANALYZE;"

# Clear Redis cache
docker-compose exec redis redis-cli FLUSHALL
```

## 📚 Additional Resources

- [Docker Documentation](https://docs.docker.com/)
- [PostgreSQL Tuning](https://wiki.postgresql.org/wiki/Tuning_Your_PostgreSQL_Server)
- [Nginx Configuration](https://nginx.org/en/docs/)
- [Prometheus Monitoring](https://prometheus.io/docs/)
- [Grafana Dashboards](https://grafana.com/docs/)

## 🤝 Support

For issues and questions:
1. Check the logs: `./deploy.sh logs`
2. Review this documentation
3. Check Docker and system resources
4. Contact the development team

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

---

**Thai Energy Trading Blockchain** - Powering Thailand's Energy Future 🇹🇭⚡
