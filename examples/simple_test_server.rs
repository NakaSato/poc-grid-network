use warp::Filter;

#[tokio::main]
async fn main() {
    println!("🔧 Starting simple test server on port 8081");
    
    // Health check endpoint
    let health = warp::path("health")
        .and(warp::get())
        .map(|| {
            println!("Health endpoint hit!");
            warp::reply::json(&serde_json::json!({
                "status": "healthy",
                "service": "Test API Bridge",
                "version": "1.0.0"
            }))
        });

    // System status endpoint
    let system_status = warp::path("api")
        .and(warp::path("v1"))
        .and(warp::path("system"))
        .and(warp::path("status"))
        .and(warp::path::end())
        .and(warp::get())
        .map(|| {
            println!("System status endpoint hit!");
            warp::reply::json(&serde_json::json!({
                "blockchain_status": "running",
                "consensus": "proof_of_authority",
                "network_id": "thai_energy_grid",
                "connected_nodes": 1,
                "current_block": 0,
                "gas_price": "0.001"
            }))
        });

    // Market data endpoint  
    let market_data = warp::path("api")
        .and(warp::path("v1"))
        .and(warp::path("trading"))
        .and(warp::path("market"))
        .and(warp::path::end())
        .and(warp::get())
        .map(|| {
            println!("Market data endpoint hit!");
            warp::reply::json(&serde_json::json!({
                "markets": [
                    {
                        "pair": "ETH/THB",
                        "price": "85000.00",
                        "volume_24h": "1250.50",
                        "change_24h": "+2.5%"
                    },
                    {
                        "pair": "ENERGY/THB", 
                        "price": "12.50",
                        "volume_24h": "8750.25",
                        "change_24h": "+1.2%"
                    }
                ],
                "total_volume_24h": "10000.75",
                "active_orders": 145,
                "recent_trades": 89
            }))
        });

    // Combine all routes with CORS
    let routes = health
        .or(system_status)
        .or(market_data)
        .with(warp::cors()
            .allow_any_origin()
            .allow_headers(vec!["content-type", "authorization"])
            .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"]))
        .with(warp::log("api_bridge"));

    println!("🚀 Test server running on http://0.0.0.0:8081");
    
    warp::serve(routes)
        .run(([0, 0, 0, 0], 8081))
        .await;
}
