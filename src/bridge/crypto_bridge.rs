use serde::{Deserialize, Serialize};
use std::sync::Arc;
use warp::{Filter, Rejection, Reply};
use crate::{SystemResult, SystemError};
use crate::crypto::GridTokenXKeyPair;
use crate::utils::crypto;

/// Request/Response structures for crypto API endpoints

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateKeypairRequest {
    /// Optional entropy for key generation (hex string)
    pub entropy: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateKeypairResponse {
    pub account_id: String,
    pub public_key: String,
    pub private_key: String,
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateAccountIdRequest {
    pub account_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateAccountIdResponse {
    pub account_id: String,
    pub is_valid: bool,
    pub format_details: Option<AccountIdFormatDetails>,
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountIdFormatDetails {
    pub length: usize,
    pub prefix: String,
    pub hex_part: String,
    pub hex_length: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeriveAccountIdRequest {
    pub public_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeriveAccountIdResponse {
    pub public_key: String,
    pub account_id: String,
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignTransactionRequest {
    pub private_key: String,
    pub transaction_data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignTransactionResponse {
    pub transaction_data: String,
    pub signature: String,
    pub signer_account_id: String,
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifySignatureRequest {
    pub public_key: String,
    pub signature: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifySignatureResponse {
    pub public_key: String,
    pub signature: String,
    pub message: String,
    pub is_valid: bool,
    pub success: bool,
    pub message_response: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateTestAccountsRequest {
    pub count: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateTestAccountsResponse {
    pub accounts: Vec<TestAccountInfo>,
    pub count: usize,
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestAccountInfo {
    pub account_id: String,
    pub is_valid: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HashDataRequest {
    pub data: String,
    pub encoding: Option<String>, // "hex", "base64", or "utf8" (default)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HashDataResponse {
    pub original_data: String,
    pub hash_hex: String,
    pub hash_bytes: Vec<u8>,
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub success: bool,
    pub code: u16,
}

/// Crypto Bridge providing HTTP APIs for cryptographic operations
pub struct CryptoBridge {
    config: CryptoBridgeConfig,
}

#[derive(Debug, Clone)]
pub struct CryptoBridgeConfig {
    pub host: String,
    pub port: u16,
    pub cors_origins: Vec<String>,
    pub max_request_size: u64,
    pub rate_limit_per_minute: u32,
}

impl Default for CryptoBridgeConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8081,
            cors_origins: vec!["*".to_string()],
            max_request_size: 1024 * 1024, // 1MB
            rate_limit_per_minute: 100,
        }
    }
}

impl CryptoBridge {
    /// Create a new crypto bridge instance
    pub fn new(config: CryptoBridgeConfig) -> Self {
        Self { config }
    }

    /// Start the crypto bridge server
    pub async fn start(&self) -> SystemResult<()> {
        let cors = warp::cors()
            .allow_any_origin()
            .allow_headers(vec!["content-type", "authorization"])
            .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"]);

        let routes = self.create_routes().with(cors);

        println!("🔐 GridTokenX Crypto Bridge starting on {}:{}", self.config.host, self.config.port);
        println!("📚 Available endpoints:");
        println!("  POST /crypto/generate-keypair    - Generate new keypair");
        println!("  POST /crypto/validate-account-id - Validate AccountId format");
        println!("  POST /crypto/derive-account-id   - Derive AccountId from public key");
        println!("  POST /crypto/sign-transaction    - Sign transaction data");
        println!("  POST /crypto/verify-signature    - Verify signature");
        println!("  POST /crypto/generate-test-accounts - Generate test accounts");
        println!("  POST /crypto/hash-data           - Hash data with SHA256");
        println!("  GET  /crypto/health              - Health check");
        println!("  GET  /crypto/info                - Bridge information");

        let addr = format!("{}:{}", self.config.host, self.config.port)
            .parse::<std::net::SocketAddr>()
            .map_err(|e| SystemError::Configuration(format!("Invalid address: {}", e)))?;

        warp::serve(routes)
            .run(addr)
            .await;

        Ok(())
    }

    /// Create all API routes
    fn create_routes(&self) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        let generate_keypair = warp::path("crypto")
            .and(warp::path("generate-keypair"))
            .and(warp::path::end())
            .and(warp::post())
            .and(warp::body::json())
            .and_then(Self::handle_generate_keypair);

        let validate_account_id = warp::path("crypto")
            .and(warp::path("validate-account-id"))
            .and(warp::path::end())
            .and(warp::post())
            .and(warp::body::json())
            .and_then(Self::handle_validate_account_id);

        let derive_account_id = warp::path("crypto")
            .and(warp::path("derive-account-id"))
            .and(warp::path::end())
            .and(warp::post())
            .and(warp::body::json())
            .and_then(Self::handle_derive_account_id);

        let sign_transaction = warp::path("crypto")
            .and(warp::path("sign-transaction"))
            .and(warp::path::end())
            .and(warp::post())
            .and(warp::body::json())
            .and_then(Self::handle_sign_transaction);

        let verify_signature = warp::path("crypto")
            .and(warp::path("verify-signature"))
            .and(warp::path::end())
            .and(warp::post())
            .and(warp::body::json())
            .and_then(Self::handle_verify_signature);

        let generate_test_accounts = warp::path("crypto")
            .and(warp::path("generate-test-accounts"))
            .and(warp::path::end())
            .and(warp::post())
            .and(warp::body::json())
            .and_then(Self::handle_generate_test_accounts);

        let hash_data = warp::path("crypto")
            .and(warp::path("hash-data"))
            .and(warp::path::end())
            .and(warp::post())
            .and(warp::body::json())
            .and_then(Self::handle_hash_data);

        let health = warp::path("crypto")
            .and(warp::path("health"))
            .and(warp::path::end())
            .and(warp::get())
            .and_then(Self::handle_health);

        let info = warp::path("crypto")
            .and(warp::path("info"))
            .and(warp::path::end())
            .and(warp::get())
            .and_then(Self::handle_info);

        generate_keypair
            .or(validate_account_id)
            .or(derive_account_id)
            .or(sign_transaction)
            .or(verify_signature)
            .or(generate_test_accounts)
            .or(hash_data)
            .or(health)
            .or(info)
            .recover(Self::handle_rejection)
    }

    /// Generate a new keypair
    async fn handle_generate_keypair(
        request: GenerateKeypairRequest,
    ) -> Result<impl Reply, Rejection> {
        match Self::generate_keypair_impl(request).await {
            Ok(response) => Ok(warp::reply::json(&response)),
            Err(err) => Ok(warp::reply::json(&ErrorResponse {
                error: err.to_string(),
                success: false,
                code: 500,
            })),
        }
    }

    async fn generate_keypair_impl(
        request: GenerateKeypairRequest,
    ) -> SystemResult<GenerateKeypairResponse> {
        let keypair = if let Some(entropy_hex) = request.entropy {
            // Use provided entropy (for testing/deterministic generation)
            let entropy_bytes = hex::decode(&entropy_hex)
                .map_err(|_| SystemError::InvalidInput("Invalid entropy hex".to_string()))?;
            
            if entropy_bytes.len() != 32 {
                return Err(SystemError::InvalidInput("Entropy must be 32 bytes".to_string()));
            }
            
            let mut entropy_array = [0u8; 32];
            entropy_array.copy_from_slice(&entropy_bytes);
            GridTokenXKeyPair::from_private_key_bytes(&entropy_array)?
        } else {
            // Generate random keypair
            GridTokenXKeyPair::generate()?
        };

        Ok(GenerateKeypairResponse {
            account_id: keypair.account_id().to_string(),
            public_key: hex::encode(keypair.export_public_key_bytes()),
            private_key: hex::encode(keypair.export_private_key_bytes()),
            success: true,
            message: "Keypair generated successfully".to_string(),
        })
    }

    /// Validate AccountId format
    async fn handle_validate_account_id(
        request: ValidateAccountIdRequest,
    ) -> Result<impl Reply, Rejection> {
        match Self::validate_account_id_impl(request).await {
            Ok(response) => Ok(warp::reply::json(&response)),
            Err(err) => Ok(warp::reply::json(&ErrorResponse {
                error: err.to_string(),
                success: false,
                code: 400,
            })),
        }
    }

    async fn validate_account_id_impl(
        request: ValidateAccountIdRequest,
    ) -> SystemResult<ValidateAccountIdResponse> {
        let is_valid = crypto::validate_account_id(&request.account_id)
            .unwrap_or(false);

        let format_details = if request.account_id.starts_with("gx_") && request.account_id.len() == 35 {
            Some(AccountIdFormatDetails {
                length: request.account_id.len(),
                prefix: "gx_".to_string(),
                hex_part: request.account_id[3..].to_string(),
                hex_length: request.account_id[3..].len(),
            })
        } else {
            None
        };

        Ok(ValidateAccountIdResponse {
            account_id: request.account_id,
            is_valid,
            format_details,
            success: true,
            message: if is_valid {
                "Valid AccountId format".to_string()
            } else {
                "Invalid AccountId format".to_string()
            },
        })
    }

    /// Derive AccountId from public key
    async fn handle_derive_account_id(
        request: DeriveAccountIdRequest,
    ) -> Result<impl Reply, Rejection> {
        match Self::derive_account_id_impl(request).await {
            Ok(response) => Ok(warp::reply::json(&response)),
            Err(err) => Ok(warp::reply::json(&ErrorResponse {
                error: err.to_string(),
                success: false,
                code: 400,
            })),
        }
    }

    async fn derive_account_id_impl(
        request: DeriveAccountIdRequest,
    ) -> SystemResult<DeriveAccountIdResponse> {
        let public_key_bytes = hex::decode(&request.public_key)
            .map_err(|_| SystemError::InvalidInput("Invalid public key hex".to_string()))?;

        if public_key_bytes.len() != 32 {
            return Err(SystemError::InvalidInput("Public key must be 32 bytes".to_string()));
        }

        let account_id = crypto::derive_account_id(&public_key_bytes)
            .map_err(|e| SystemError::Internal(e.to_string()))?;

        Ok(DeriveAccountIdResponse {
            public_key: request.public_key,
            account_id,
            success: true,
            message: "AccountId derived successfully".to_string(),
        })
    }

    /// Sign transaction data
    async fn handle_sign_transaction(
        request: SignTransactionRequest,
    ) -> Result<impl Reply, Rejection> {
        match Self::sign_transaction_impl(request).await {
            Ok(response) => Ok(warp::reply::json(&response)),
            Err(err) => Ok(warp::reply::json(&ErrorResponse {
                error: err.to_string(),
                success: false,
                code: 400,
            })),
        }
    }

    async fn sign_transaction_impl(
        request: SignTransactionRequest,
    ) -> SystemResult<SignTransactionResponse> {
        let private_key_bytes = hex::decode(&request.private_key)
            .map_err(|_| SystemError::InvalidInput("Invalid private key hex".to_string()))?;

        if private_key_bytes.len() != 32 {
            return Err(SystemError::InvalidInput("Private key must be 32 bytes".to_string()));
        }

        let mut private_key_array = [0u8; 32];
        private_key_array.copy_from_slice(&private_key_bytes);

        let keypair = GridTokenXKeyPair::from_private_key_bytes(&private_key_array)?;
        let transaction_bytes = request.transaction_data.as_bytes();
        
        let signature = keypair.sign(transaction_bytes);

        Ok(SignTransactionResponse {
            transaction_data: request.transaction_data,
            signature: hex::encode(&signature.to_bytes()),
            signer_account_id: keypair.account_id().to_string(),
            success: true,
            message: "Transaction signed successfully".to_string(),
        })
    }

    /// Verify signature
    async fn handle_verify_signature(
        request: VerifySignatureRequest,
    ) -> Result<impl Reply, Rejection> {
        match Self::verify_signature_impl(request).await {
            Ok(response) => Ok(warp::reply::json(&response)),
            Err(err) => Ok(warp::reply::json(&ErrorResponse {
                error: err.to_string(),
                success: false,
                code: 400,
            })),
        }
    }

    async fn verify_signature_impl(
        request: VerifySignatureRequest,
    ) -> SystemResult<VerifySignatureResponse> {
        let public_key_bytes = hex::decode(&request.public_key)
            .map_err(|_| SystemError::InvalidInput("Invalid public key hex".to_string()))?;

        let signature_bytes = hex::decode(&request.signature)
            .map_err(|_| SystemError::InvalidInput("Invalid signature hex".to_string()))?;

        if public_key_bytes.len() != 32 {
            return Err(SystemError::InvalidInput("Public key must be 32 bytes".to_string()));
        }

        let message_bytes = request.message.as_bytes();
        let is_valid = crypto::verify_signature(&public_key_bytes, &signature_bytes, message_bytes)
            .map_err(|e| SystemError::Internal(e.to_string()))?;

        Ok(VerifySignatureResponse {
            public_key: request.public_key,
            signature: request.signature,
            message: request.message,
            is_valid,
            success: true,
            message_response: if is_valid {
                "Signature is valid".to_string()
            } else {
                "Signature is invalid".to_string()
            },
        })
    }

    /// Generate test accounts
    async fn handle_generate_test_accounts(
        request: GenerateTestAccountsRequest,
    ) -> Result<impl Reply, Rejection> {
        match Self::generate_test_accounts_impl(request).await {
            Ok(response) => Ok(warp::reply::json(&response)),
            Err(err) => Ok(warp::reply::json(&ErrorResponse {
                error: err.to_string(),
                success: false,
                code: 400,
            })),
        }
    }

    async fn generate_test_accounts_impl(
        request: GenerateTestAccountsRequest,
    ) -> SystemResult<GenerateTestAccountsResponse> {
        let count = request.count.unwrap_or(5).min(50); // Max 50 accounts
        let mut accounts = Vec::new();

        for _ in 0..count {
            let account_id = crypto::generate_test_account_id();
            let is_valid = crypto::validate_account_id(&account_id).unwrap_or(false);
            
            accounts.push(TestAccountInfo {
                account_id,
                is_valid,
            });
        }

        Ok(GenerateTestAccountsResponse {
            accounts,
            count,
            success: true,
            message: format!("Generated {} test accounts", count),
        })
    }

    /// Hash data with SHA256
    async fn handle_hash_data(
        request: HashDataRequest,
    ) -> Result<impl Reply, Rejection> {
        match Self::hash_data_impl(request).await {
            Ok(response) => Ok(warp::reply::json(&response)),
            Err(err) => Ok(warp::reply::json(&ErrorResponse {
                error: err.to_string(),
                success: false,
                code: 400,
            })),
        }
    }

    async fn hash_data_impl(
        request: HashDataRequest,
    ) -> SystemResult<HashDataResponse> {
        let data_bytes = match request.encoding.as_deref() {
            Some("hex") => hex::decode(&request.data)
                .map_err(|_| SystemError::InvalidInput("Invalid hex data".to_string()))?,
            Some("base64") => {
                use base64::Engine;
                base64::engine::general_purpose::STANDARD.decode(&request.data)
                    .map_err(|_| SystemError::InvalidInput("Invalid base64 data".to_string()))?
            },
            _ => request.data.as_bytes().to_vec(), // Default to UTF-8
        };

        let hash = crypto::hash_sha256(&data_bytes);
        let hash_hex = hex::encode(&hash);

        Ok(HashDataResponse {
            original_data: request.data,
            hash_hex,
            hash_bytes: hash.to_vec(),
            success: true,
            message: "Data hashed successfully".to_string(),
        })
    }

    /// Health check endpoint
    async fn handle_health() -> Result<impl Reply, Rejection> {
        let response = serde_json::json!({
            "status": "healthy",
            "service": "GridTokenX Crypto Bridge",
            "version": "1.0.0",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "endpoints": [
                "/crypto/generate-keypair",
                "/crypto/validate-account-id",
                "/crypto/derive-account-id",
                "/crypto/sign-transaction",
                "/crypto/verify-signature",
                "/crypto/generate-test-accounts",
                "/crypto/hash-data",
                "/crypto/health",
                "/crypto/info"
            ]
        });

        Ok(warp::reply::json(&response))
    }

    /// Bridge information endpoint
    async fn handle_info() -> Result<impl Reply, Rejection> {
        let response = serde_json::json!({
            "name": "GridTokenX Crypto Bridge",
            "description": "HTTP API bridge for GridTokenX cryptographic operations",
            "version": "1.0.0",
            "features": [
                "Ed25519 keypair generation",
                "AccountId validation and derivation",
                "Transaction signing and verification",
                "SHA256 hashing",
                "Test account generation"
            ],
            "security": {
                "algorithm": "Ed25519",
                "hash_function": "SHA256",
                "account_id_format": "gx_[32_hex_characters]"
            },
            "usage": {
                "content_type": "application/json",
                "cors_enabled": true,
                "rate_limiting": "100 requests per minute"
            }
        });

        Ok(warp::reply::json(&response))
    }

    /// Handle rejections and convert to JSON error responses
    async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
        let (code, message) = if err.is_not_found() {
            (404, "Endpoint not found")
        } else if err.find::<warp::filters::body::BodyDeserializeError>().is_some() {
            (400, "Invalid JSON body")
        } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
            (405, "Method not allowed")
        } else {
            (500, "Internal server error")
        };

        let response = ErrorResponse {
            error: message.to_string(),
            success: false,
            code,
        };

        Ok(warp::reply::with_status(
            warp::reply::json(&response),
            warp::http::StatusCode::from_u16(code).unwrap(),
        ))
    }
}

/// Convenience function to start the crypto bridge with default configuration
pub async fn start_crypto_bridge() -> SystemResult<()> {
    let bridge = CryptoBridge::new(CryptoBridgeConfig::default());
    bridge.start().await
}

/// Convenience function to start the crypto bridge with custom configuration
pub async fn start_crypto_bridge_with_config(config: CryptoBridgeConfig) -> SystemResult<()> {
    let bridge = CryptoBridge::new(config);
    bridge.start().await
}
