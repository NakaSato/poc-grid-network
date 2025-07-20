#!/bin/bash

# Thai Energy Trading Blockchain - Deployment Verification Script

echo "🇹🇭 Thai Energy Trading Blockchain - Deployment Status Check"
echo "================================================================"

# Check system components
echo -e "\n📦 Infrastructure Status:"
echo "------------------------"
docker ps --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}" | grep -E "(thai-postgres|thai-redis)" || echo "❌ No Thai infrastructure containers found"

echo -e "\n🔗 Blockchain Application:"
echo "-------------------------"
if pgrep -f "thai-energy-trading-blockchain" > /dev/null; then
    echo "✅ Thai Energy Trading Blockchain is running"
    ps aux | grep thai-energy-trading-blockchain | grep -v grep | head -1
else
    echo "❌ Thai Energy Trading Blockchain is not running"
fi

echo -e "\n🌐 Network Connectivity:"
echo "------------------------"
# Test PostgreSQL connection
if timeout 3 bash -c "</dev/tcp/localhost/5432"; then
    echo "✅ PostgreSQL accessible on port 5432"
else
    echo "❌ PostgreSQL not accessible"
fi

# Test Redis connection  
if timeout 3 bash -c "</dev/tcp/localhost/6379"; then
    echo "✅ Redis accessible on port 6379"
else
    echo "❌ Redis not accessible"
fi

echo -e "\n🔍 Active Listening Ports:"
echo "-------------------------"
netstat -an | grep LISTEN | grep -E "(543[0-9]|637[0-9]|[0-9]{4})" | head -10

echo -e "\n⚡ System Ready Summary:"
echo "======================"

POSTGRES_OK=0
REDIS_OK=0
APP_OK=0

if docker ps | grep -q thai-postgres; then
    POSTGRES_OK=1
    echo "✅ PostgreSQL Database: Running"
else
    echo "❌ PostgreSQL Database: Not Running"
fi

if docker ps | grep -q thai-redis; then
    REDIS_OK=1
    echo "✅ Redis Cache: Running"  
else
    echo "❌ Redis Cache: Not Running"
fi

if pgrep -f "thai-energy-trading-blockchain" > /dev/null; then
    APP_OK=1
    echo "✅ Blockchain Application: Running"
else
    echo "❌ Blockchain Application: Not Running"
fi

echo -e "\n🎯 Deployment Status:"
if [ $POSTGRES_OK -eq 1 ] && [ $REDIS_OK -eq 1 ] && [ $APP_OK -eq 1 ]; then
    echo "✅ FULL DEPLOYMENT SUCCESSFUL - Thai Energy Trading System is operational"
    echo "🔗 System Type: Hybrid Deployment (Containerized Infrastructure + Local Blockchain Node)"
    echo "🏗️ Architecture: Proof-of-Authority Blockchain with P2P Network"
    echo "⚡ Ready for: Energy trading, grid integration, and token transactions"
else
    echo "⚠️ PARTIAL DEPLOYMENT - Some components missing"
fi

echo -e "\n📝 Next Steps:"
echo "- Monitor application logs for startup completion"
echo "- Test energy trading operations via P2P interface"
echo "- Verify database connectivity and schema initialization" 
echo "- Check blockchain node synchronization status"

echo -e "\nDeployment check completed at $(date)"
