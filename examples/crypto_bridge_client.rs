use serde_json::json;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🧪 GridTokenX Crypto Bridge Client Demo");
    println!("=======================================");
    println!();

    let base_url = "http://localhost:8081";
    let client = reqwest::Client::new();

    // Test 1: Health check
    println!("1️⃣ Health Check");
    println!("--------------");
    match test_health_check(&client, base_url).await {
        Ok(_) => println!("✅ Health check passed"),
        Err(e) => println!("❌ Health check failed: {}", e),
    }
    println!();

    // Test 2: Generate keypair
    println!("2️⃣ Generate Keypair");
    println!("------------------");
    let keypair = match test_generate_keypair(&client, base_url).await {
        Ok(kp) => {
            println!("✅ Keypair generated successfully");
            println!("   AccountId: {}", kp.account_id);
            println!("   Public Key: {}...", &kp.public_key[0..16]);
            Some(kp)
        }
        Err(e) => {
            println!("❌ Keypair generation failed: {}", e);
            None
        }
    };
    println!();

    if let Some(kp) = keypair {
        // Test 3: Validate AccountId
        println!("3️⃣ Validate AccountId");
        println!("--------------------");
        match test_validate_account_id(&client, base_url, &kp.account_id).await {
            Ok(is_valid) => {
                if is_valid {
                    println!("✅ AccountId validation passed");
                } else {
                    println!("❌ AccountId validation failed");
                }
            }
            Err(e) => println!("❌ AccountId validation error: {}", e),
        }
        println!();

        // Test 4: Derive AccountId from public key
        println!("4️⃣ Derive AccountId from Public Key");
        println!("----------------------------------");
        match test_derive_account_id(&client, base_url, &kp.public_key).await {
            Ok(derived_account_id) => {
                println!("✅ AccountId derived successfully");
                println!("   Original:  {}", kp.account_id);
                println!("   Derived:   {}", derived_account_id);
                if kp.account_id == derived_account_id {
                    println!("✅ Derived AccountId matches original");
                } else {
                    println!("❌ Derived AccountId doesn't match");
                }
            }
            Err(e) => println!("❌ AccountId derivation failed: {}", e),
        }
        println!();

        // Test 5: Sign and verify transaction
        println!("5️⃣ Sign and Verify Transaction");
        println!("-----------------------------");
        let transaction_data = "Send 100 kWh from Alice to Bob";
        match test_sign_transaction(&client, base_url, &kp.private_key, transaction_data).await {
            Ok(signature) => {
                println!("✅ Transaction signed successfully");
                println!("   Signature: {}...", &signature[0..16]);
                
                // Now verify the signature
                match test_verify_signature(&client, base_url, &kp.public_key, &signature, transaction_data).await {
                    Ok(is_valid) => {
                        if is_valid {
                            println!("✅ Signature verification passed");
                        } else {
                            println!("❌ Signature verification failed");
                        }
                    }
                    Err(e) => println!("❌ Signature verification error: {}", e),
                }
            }
            Err(e) => println!("❌ Transaction signing failed: {}", e),
        }
        println!();
    }

    // Test 6: Generate test accounts
    println!("6️⃣ Generate Test Accounts");
    println!("------------------------");
    match test_generate_test_accounts(&client, base_url, 3).await {
        Ok(accounts) => {
            println!("✅ Test accounts generated successfully");
            for (i, account) in accounts.iter().enumerate() {
                println!("   {}. {} - {}", 
                    i + 1, 
                    account.account_id,
                    if account.is_valid { "✅ Valid" } else { "❌ Invalid" }
                );
            }
        }
        Err(e) => println!("❌ Test account generation failed: {}", e),
    }
    println!();

    // Test 7: Hash data
    println!("7️⃣ Hash Data");
    println!("-----------");
    let test_data = "GridTokenX blockchain data";
    match test_hash_data(&client, base_url, test_data).await {
        Ok(hash) => {
            println!("✅ Data hashed successfully");
            println!("   Original: {}", test_data);
            println!("   SHA256:   {}", hash);
        }
        Err(e) => println!("❌ Data hashing failed: {}", e),
    }
    println!();

    // Test 8: Bridge info
    println!("8️⃣ Bridge Information");
    println!("--------------------");
    match test_bridge_info(&client, base_url).await {
        Ok(_) => println!("✅ Bridge info retrieved successfully"),
        Err(e) => println!("❌ Bridge info failed: {}", e),
    }
    println!();

    println!("🎉 All tests completed!");
    println!();
    println!("💡 Integration Tips:");
    println!("   • Store private keys securely (encrypted storage)");
    println!("   • Always validate AccountIds before processing");
    println!("   • Use HTTPS in production environments");
    println!("   • Implement proper error handling in your client");
    println!("   • Consider implementing retry logic for network calls");

    Ok(())
}

#[derive(serde::Deserialize)]
struct KeypairResponse {
    account_id: String,
    public_key: String,
    private_key: String,
    success: bool,
}

#[derive(serde::Deserialize)]
struct ValidationResponse {
    is_valid: bool,
    success: bool,
}

#[derive(serde::Deserialize)]
struct DeriveResponse {
    account_id: String,
    success: bool,
}

#[derive(serde::Deserialize)]
struct SignResponse {
    signature: String,
    success: bool,
}

#[derive(serde::Deserialize)]
struct VerifyResponse {
    is_valid: bool,
    success: bool,
}

#[derive(serde::Deserialize)]
struct TestAccountInfo {
    account_id: String,
    is_valid: bool,
}

#[derive(serde::Deserialize)]
struct TestAccountsResponse {
    accounts: Vec<TestAccountInfo>,
    success: bool,
}

#[derive(serde::Deserialize)]
struct HashResponse {
    hash_hex: String,
    success: bool,
}

async fn test_health_check(client: &reqwest::Client, base_url: &str) -> Result<(), Box<dyn Error>> {
    let response = client
        .get(&format!("{}/crypto/health", base_url))
        .send()
        .await?;

    if response.status().is_success() {
        let json: serde_json::Value = response.json().await?;
        println!("   Status: {}", json["status"].as_str().unwrap_or("unknown"));
        println!("   Service: {}", json["service"].as_str().unwrap_or("unknown"));
        Ok(())
    } else {
        Err(format!("Health check failed with status: {}", response.status()).into())
    }
}

async fn test_generate_keypair(client: &reqwest::Client, base_url: &str) -> Result<KeypairResponse, Box<dyn Error>> {
    let response = client
        .post(&format!("{}/crypto/generate-keypair", base_url))
        .json(&json!({}))
        .send()
        .await?;

    if response.status().is_success() {
        let keypair: KeypairResponse = response.json().await?;
        if keypair.success {
            Ok(keypair)
        } else {
            Err("Keypair generation failed".into())
        }
    } else {
        Err(format!("Request failed with status: {}", response.status()).into())
    }
}

async fn test_validate_account_id(client: &reqwest::Client, base_url: &str, account_id: &str) -> Result<bool, Box<dyn Error>> {
    let response = client
        .post(&format!("{}/crypto/validate-account-id", base_url))
        .json(&json!({ "account_id": account_id }))
        .send()
        .await?;

    if response.status().is_success() {
        let validation: ValidationResponse = response.json().await?;
        if validation.success {
            Ok(validation.is_valid)
        } else {
            Err("Validation request failed".into())
        }
    } else {
        Err(format!("Request failed with status: {}", response.status()).into())
    }
}

async fn test_derive_account_id(client: &reqwest::Client, base_url: &str, public_key: &str) -> Result<String, Box<dyn Error>> {
    let response = client
        .post(&format!("{}/crypto/derive-account-id", base_url))
        .json(&json!({ "public_key": public_key }))
        .send()
        .await?;

    if response.status().is_success() {
        let derive: DeriveResponse = response.json().await?;
        if derive.success {
            Ok(derive.account_id)
        } else {
            Err("Derivation request failed".into())
        }
    } else {
        Err(format!("Request failed with status: {}", response.status()).into())
    }
}

async fn test_sign_transaction(client: &reqwest::Client, base_url: &str, private_key: &str, transaction_data: &str) -> Result<String, Box<dyn Error>> {
    let response = client
        .post(&format!("{}/crypto/sign-transaction", base_url))
        .json(&json!({ 
            "private_key": private_key,
            "transaction_data": transaction_data
        }))
        .send()
        .await?;

    if response.status().is_success() {
        let sign: SignResponse = response.json().await?;
        if sign.success {
            Ok(sign.signature)
        } else {
            Err("Signing request failed".into())
        }
    } else {
        Err(format!("Request failed with status: {}", response.status()).into())
    }
}

async fn test_verify_signature(client: &reqwest::Client, base_url: &str, public_key: &str, signature: &str, message: &str) -> Result<bool, Box<dyn Error>> {
    let response = client
        .post(&format!("{}/crypto/verify-signature", base_url))
        .json(&json!({ 
            "public_key": public_key,
            "signature": signature,
            "message": message
        }))
        .send()
        .await?;

    if response.status().is_success() {
        let verify: VerifyResponse = response.json().await?;
        if verify.success {
            Ok(verify.is_valid)
        } else {
            Err("Verification request failed".into())
        }
    } else {
        Err(format!("Request failed with status: {}", response.status()).into())
    }
}

async fn test_generate_test_accounts(client: &reqwest::Client, base_url: &str, count: usize) -> Result<Vec<TestAccountInfo>, Box<dyn Error>> {
    let response = client
        .post(&format!("{}/crypto/generate-test-accounts", base_url))
        .json(&json!({ "count": count }))
        .send()
        .await?;

    if response.status().is_success() {
        let accounts: TestAccountsResponse = response.json().await?;
        if accounts.success {
            Ok(accounts.accounts)
        } else {
            Err("Test accounts generation failed".into())
        }
    } else {
        Err(format!("Request failed with status: {}", response.status()).into())
    }
}

async fn test_hash_data(client: &reqwest::Client, base_url: &str, data: &str) -> Result<String, Box<dyn Error>> {
    let response = client
        .post(&format!("{}/crypto/hash-data", base_url))
        .json(&json!({ 
            "data": data,
            "encoding": "utf8"
        }))
        .send()
        .await?;

    if response.status().is_success() {
        let hash: HashResponse = response.json().await?;
        if hash.success {
            Ok(hash.hash_hex)
        } else {
            Err("Hash request failed".into())
        }
    } else {
        Err(format!("Request failed with status: {}", response.status()).into())
    }
}

async fn test_bridge_info(client: &reqwest::Client, base_url: &str) -> Result<(), Box<dyn Error>> {
    let response = client
        .get(&format!("{}/crypto/info", base_url))
        .send()
        .await?;

    if response.status().is_success() {
        let json: serde_json::Value = response.json().await?;
        println!("   Name: {}", json["name"].as_str().unwrap_or("unknown"));
        println!("   Version: {}", json["version"].as_str().unwrap_or("unknown"));
        println!("   Features: {:?}", json["features"].as_array().unwrap_or(&vec![]));
        Ok(())
    } else {
        Err(format!("Info request failed with status: {}", response.status()).into())
    }
}
