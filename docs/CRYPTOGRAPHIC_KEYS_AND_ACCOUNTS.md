# GridTokenX Cryptographic Key System & Account Management

## Overview

GridTokenX uses a robust cryptographic system based on Ed25519 elliptic curve cryptography for user authentication, transaction signing, and account identification. This document provides a comprehensive guide to understanding and implementing the key management system.

## Cryptographic Architecture

```
┌─────────────────┐    Mathematical     ┌─────────────────┐    Deterministic    ┌─────────────────┐
│   Private Key   │    Derivation       │   Public Key    │    Derivation       │   AccountId     │
│   (Secret)      │ ─────────────────→  │   (Shareable)   │ ─────────────────→  │   (Address)     │
│   32 bytes      │                     │   32 bytes      │                     │   String        │
│   Ed25519       │                     │   Curve Point   │                     │   "gx_<hex>"    │
└─────────────────┘                     └─────────────────┘                     └─────────────────┘
        │                                       │                                       │
        │ Used for                              │ Used for                              │ Used for
        ▼                                       ▼                                       ▼
┌─────────────────┐                   ┌─────────────────┐                     ┌─────────────────┐
│ Transaction     │                   │ Signature       │                     │ Public Address  │
│ Signing         │                   │ Verification    │                     │ Identification  │
│ Authorization   │                   │ Identity Proof  │                     │ Fund Reception  │
└─────────────────┘                   └─────────────────┘                     └─────────────────┘
```

## Key Components Explained

### 1. Private Key

The **private key** is the foundational secret that provides complete control over a user's account and assets.

#### Properties:
- **Size**: 32 bytes (256 bits)
- **Algorithm**: Ed25519 scalar value
- **Generation**: Cryptographically secure random number generation
- **Security**: Must never be shared, transmitted, or stored unencrypted
- **Uniqueness**: Statistically guaranteed to be unique across all users

#### Generation Process:
```rust
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;

// Generate cryptographically secure private key
let private_key = SigningKey::generate(&mut OsRng);
```

#### Security Requirements:
- **Confidentiality**: Store in secure hardware (HSMs) when possible
- **Encryption**: Always encrypt when stored on disk
- **Access Control**: Implement strict access permissions
- **Memory Safety**: Clear from memory after use (using zeroization)
- **Backup**: Secure backup with recovery mechanisms

### 2. Public Key

The **public key** is mathematically derived from the private key and can be shared freely without compromising security.

#### Properties:
- **Size**: 32 bytes (256 bits)
- **Algorithm**: Ed25519 curve point
- **Derivation**: Deterministic from private key using elliptic curve mathematics
- **Sharing**: Safe to share publicly and transmit over insecure channels
- **Verification**: Used to verify signatures created by the corresponding private key

#### Derivation Process:
```rust
use ed25519_dalek::{SigningKey, VerifyingKey};

// Derive public key from private key
let signing_key = SigningKey::generate(&mut OsRng);
let verifying_key = signing_key.verifying_key(); // This is the public key
```

#### Use Cases:
- **Signature Verification**: Verify that a transaction was signed by the private key holder
- **Identity Verification**: Prove that someone possesses the private key without revealing it
- **AccountId Generation**: Source material for creating the user's blockchain address
- **Encryption**: Can be used in asymmetric encryption schemes (though not primary use in GridTokenX)

### 3. AccountId (Blockchain Address)

The **AccountId** is a human-readable string derived from the public key that serves as the user's blockchain address.

#### Properties:
- **Type**: String with specific format
- **Prefix**: "gx_" (GridTokenX identifier)
- **Encoding**: Hexadecimal representation of hashed public key
- **Length**: 35 characters ("gx_" + 32 hex characters)
- **Uniqueness**: Cryptographically guaranteed uniqueness
- **Deterministic**: Same public key always produces the same AccountId

#### Derivation Process:

```rust
use ed25519_dalek::VerifyingKey;
use sha2::{Sha256, Digest};

pub fn derive_account_id(public_key: &VerifyingKey) -> String {
    // Step 1: Get public key bytes
    let public_key_bytes = public_key.to_bytes();
    
    // Step 2: Hash the public key with SHA256
    let mut hasher = Sha256::new();
    hasher.update(&public_key_bytes);
    let hash = hasher.finalize();
    
    // Step 3: Take first 16 bytes for account identifier
    let account_hash = &hash[..16];
    
    // Step 4: Encode as hexadecimal
    let account_hex = hex::encode(account_hash);
    
    // Step 5: Add GridTokenX prefix
    format!("gx_{}", account_hex)
}
```

#### Format Specification:
- **Valid Format**: `gx_[32 hexadecimal characters]`
- **Example**: `gx_a1b2c3d4e5f6789012345678901234ab`
- **Validation**: Must start with "gx_", followed by exactly 32 hex characters

## Complete Implementation Example

### Keypair Management Structure

```rust
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

/// Complete keypair management for GridTokenX accounts
#[derive(Clone)]
pub struct GridTokenXKeyPair {
    signing_key: SigningKey,      // Private key (keep secret!)
    verifying_key: VerifyingKey,  // Public key (safe to share)
    account_id: String,           // Derived blockchain address
}

impl GridTokenXKeyPair {
    /// Generate a new keypair with AccountId
    pub fn generate() -> Result<Self, Box<dyn std::error::Error>> {
        // Generate cryptographically secure private key
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();
        let account_id = derive_account_id(&verifying_key);
        
        Ok(Self {
            signing_key,
            verifying_key,
            account_id,
        })
    }
    
    /// Import keypair from existing private key bytes
    pub fn from_private_key_bytes(private_key_bytes: &[u8; 32]) -> Result<Self, Box<dyn std::error::Error>> {
        let signing_key = SigningKey::from_bytes(private_key_bytes);
        let verifying_key = signing_key.verifying_key();
        let account_id = derive_account_id(&verifying_key);
        
        Ok(Self {
            signing_key,
            verifying_key,
            account_id,
        })
    }
    
    /// Get the AccountId (blockchain address)
    pub fn account_id(&self) -> &str {
        &self.account_id
    }
    
    /// Get the public key
    pub fn public_key(&self) -> &VerifyingKey {
        &self.verifying_key
    }
    
    /// Sign a message (typically a transaction)
    pub fn sign(&self, message: &[u8]) -> Signature {
        self.signing_key.sign(message)
    }
    
    /// Verify a signature against this keypair's public key
    pub fn verify(&self, message: &[u8], signature: &Signature) -> Result<(), ed25519_dalek::SignatureError> {
        self.verifying_key.verify(message, signature)
    }
    
    /// Export private key bytes (use with extreme caution!)
    pub fn export_private_key_bytes(&self) -> [u8; 32] {
        self.signing_key.to_bytes()
    }
    
    /// Export public key bytes
    pub fn export_public_key_bytes(&self) -> [u8; 32] {
        self.verifying_key.to_bytes()
    }
    
    /// Sign a transaction with automatic serialization
    pub fn sign_transaction<T: Serialize>(&self, transaction: &T) -> Result<TransactionSignature, Box<dyn std::error::Error>> {
        // Serialize transaction
        let transaction_json = serde_json::to_string(transaction)?;
        let transaction_bytes = transaction_json.as_bytes();
        
        // Create message hash that includes AccountId
        let mut hasher = Sha256::new();
        hasher.update(transaction_bytes);
        hasher.update(self.account_id.as_bytes());
        let message_hash = hasher.finalize();
        
        // Sign the hash
        let signature = self.sign(&message_hash);
        
        Ok(TransactionSignature {
            signature: signature.to_bytes().to_vec(),
            public_key: self.verifying_key.to_bytes().to_vec(),
            account_id: self.account_id.clone(),
            timestamp: chrono::Utc::now(),
        })
    }
}

/// Transaction signature structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionSignature {
    pub signature: Vec<u8>,          // Ed25519 signature bytes
    pub public_key: Vec<u8>,         // Public key bytes
    pub account_id: String,          // Signer's AccountId
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
```

## Transaction Flow Example

Here's how the cryptographic system works in practice:

### 1. Account Creation
```rust
// Generate new account
let keypair = GridTokenXKeyPair::generate()?;

println!("New Account Created:");
println!("AccountId: {}", keypair.account_id());
println!("Public Key: {}", hex::encode(keypair.export_public_key_bytes()));
// Private key should never be displayed in production!
```

### 2. Transaction Signing
```rust
#[derive(Serialize)]
struct EnergyTransfer {
    from: String,
    to: String,
    energy_amount: f64,
    price_per_kwh: u128,
}

// Create transaction
let transaction = EnergyTransfer {
    from: keypair.account_id().to_string(),
    to: "gx_recipient_account".to_string(),
    energy_amount: 100.0,
    price_per_kwh: 1250, // 12.50 THB in cents
};

// Sign transaction
let signature = keypair.sign_transaction(&transaction)?;

println!("Transaction signed:");
println!("Signature: {}", hex::encode(&signature.signature));
println!("Signer: {}", signature.account_id);
```

### 3. Signature Verification
```rust
pub fn verify_transaction_signature<T: Serialize>(
    transaction: &T,
    signature: &TransactionSignature
) -> Result<bool, Box<dyn std::error::Error>> {
    // Validate AccountId format
    if !validate_account_id(&signature.account_id)? {
        return Ok(false);
    }
    
    // Recreate message hash
    let transaction_json = serde_json::to_string(transaction)?;
    let transaction_bytes = transaction_json.as_bytes();
    
    let mut hasher = Sha256::new();
    hasher.update(transaction_bytes);
    hasher.update(signature.account_id.as_bytes());
    let message_hash = hasher.finalize();
    
    // Verify signature
    let public_key = VerifyingKey::from_bytes(&signature.public_key.try_into().unwrap())?;
    let ed_signature = ed25519_dalek::Signature::from_bytes(&signature.signature.try_into().unwrap())?;
    
    match public_key.verify(&message_hash, &ed_signature) {
        Ok(()) => {
            // Additional check: verify AccountId matches public key
            let derived_account_id = derive_account_id(&public_key);
            Ok(derived_account_id == signature.account_id)
        }
        Err(_) => Ok(false)
    }
}
```

## Security Best Practices

### Private Key Security

1. **Storage**:
   ```rust
   // Use hardware security modules (HSM) in production
   // Encrypt private keys at rest
   use aes_gcm::{Aes256Gcm, Key, Nonce};
   
   pub struct SecureKeyStorage {
       cipher: Aes256Gcm,
   }
   
   impl SecureKeyStorage {
       pub fn store_private_key(&self, private_key: &[u8; 32], password: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
           let key = derive_key_from_password(password);
           let cipher = Aes256Gcm::new(&key);
           
           let nonce = Nonce::from_slice(b"unique nonce"); // Use random nonce in production
           let ciphertext = cipher.encrypt(nonce, private_key.as_ref())?;
           Ok(ciphertext)
       }
   }
   ```

2. **Memory Safety**:
   ```rust
   use zeroize::Zeroize;
   
   impl Drop for GridTokenXKeyPair {
       fn drop(&mut self) {
           // Clear sensitive data from memory
           // Note: ed25519_dalek handles this internally
       }
   }
   ```

3. **Access Control**:
   ```rust
   // Implement role-based access to private keys
   pub struct KeyManager {
       keys: HashMap<String, GridTokenXKeyPair>,
       permissions: HashMap<String, Vec<Permission>>,
   }
   
   impl KeyManager {
       pub fn access_key(&self, account_id: &str, user_id: &str) -> Option<&GridTokenXKeyPair> {
           if self.has_permission(user_id, account_id, Permission::Sign) {
               self.keys.get(account_id)
           } else {
               None
           }
       }
   }
   ```

### AccountId Validation

```rust
/// Comprehensive AccountId validation
pub fn validate_account_id(account_id: &str) -> Result<bool, Box<dyn std::error::Error>> {
    // Check minimum length
    if account_id.len() != 35 {
        return Ok(false);
    }
    
    // Check prefix
    if !account_id.starts_with("gx_") {
        return Ok(false);
    }
    
    // Extract and validate hex part
    let hex_part = &account_id[3..];
    if hex_part.len() != 32 {
        return Ok(false);
    }
    
    // Verify hex encoding
    match hex::decode(hex_part) {
        Ok(decoded) if decoded.len() == 16 => Ok(true),
        _ => Ok(false),
    }
}

/// Verify AccountId matches public key
pub fn verify_account_id_matches_public_key(
    account_id: &str, 
    public_key: &VerifyingKey
) -> Result<bool, Box<dyn std::error::Error>> {
    let derived_account_id = derive_account_id(public_key);
    Ok(derived_account_id == account_id)
}
```

## Account Management System

### Multi-Account Management
```rust
use tokio::sync::RwLock;
use std::collections::HashMap;

pub struct AccountManager {
    accounts: RwLock<HashMap<String, GridTokenXKeyPair>>,
    account_lookup: RwLock<HashMap<[u8; 20], String>>,
}

impl AccountManager {
    pub fn new() -> Self {
        Self {
            accounts: RwLock::new(HashMap::new()),
            account_lookup: RwLock::new(HashMap::new()),
        }
    }
    
    /// Create a new account
    pub async fn create_account(&self) -> Result<String, Box<dyn std::error::Error>> {
        let keypair = GridTokenXKeyPair::generate()?;
        let account_id = keypair.account_id().to_string();
        
        // Add to storage
        let mut accounts = self.accounts.write().await;
        accounts.insert(account_id.clone(), keypair);
        
        println!("Created new account: {}", account_id);
        Ok(account_id)
    }
    
    /// Import existing account
    pub async fn import_account(&self, private_key_bytes: [u8; 32]) -> Result<String, Box<dyn std::error::Error>> {
        let keypair = GridTokenXKeyPair::from_private_key_bytes(&private_key_bytes)?;
        let account_id = keypair.account_id().to_string();
        
        let mut accounts = self.accounts.write().await;
        accounts.insert(account_id.clone(), keypair);
        
        println!("Imported account: {}", account_id);
        Ok(account_id)
    }
    
    /// Sign transaction for specific account
    pub async fn sign_transaction<T: Serialize>(
        &self, 
        account_id: &str, 
        transaction: &T
    ) -> Result<TransactionSignature, Box<dyn std::error::Error>> {
        let accounts = self.accounts.read().await;
        let keypair = accounts.get(account_id)
            .ok_or("Account not found")?;
        
        keypair.sign_transaction(transaction)
    }
    
    /// List all managed accounts
    pub async fn list_accounts(&self) -> Vec<String> {
        let accounts = self.accounts.read().await;
        accounts.keys().cloned().collect()
    }
}
```

## Command-Line Tools

The system includes comprehensive CLI tools for key management:

### Key Generation
```bash
# Generate new keypair
cargo run --example key_manager_cli generate --output ./keys --name my_account

# Output:
# ✅ Successfully generated new keypair!
# AccountId: gx_a1b2c3d4e5f6789012345678901234ab
# Files created:
#   Private Key: ./keys/my_account_private_key.hex
#   Public Key:  ./keys/my_account_public_key.hex
#   AccountId:   ./keys/my_account_account_id.txt
```

### AccountId Derivation
```bash
# Derive AccountId from public key
cargo run --example key_manager_cli derive --pubkey a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef123456

# Output:
# ✅ AccountId derived successfully!
# Input Public Key: a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef123456
# Derived AccountId: gx_a1b2c3d4e5f6789012345678901234ab
```

### Validation
```bash
# Validate AccountId format
cargo run --example key_manager_cli validate gx_a1b2c3d4e5f6789012345678901234ab

# Verify AccountId matches public key
cargo run --example key_manager_cli verify \
  --account gx_a1b2c3d4e5f6789012345678901234ab \
  --pubkey a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef123456
```

## Integration with Backend APIs

### REST API Integration

```rust
// API endpoint for account creation
#[post("/api/v1/accounts")]
async fn create_account(account_manager: web::Data<AccountManager>) -> Result<impl Responder> {
    let account_id = account_manager.create_account().await?;
    
    Ok(web::Json(json!({
        "account_id": account_id,
        "status": "created",
        "message": "Account created successfully"
    })))
}

// API endpoint for transaction signing
#[post("/api/v1/transactions/sign")]
async fn sign_transaction(
    account_manager: web::Data<AccountManager>,
    req: web::Json<SignTransactionRequest>
) -> Result<impl Responder> {
    let signature = account_manager
        .sign_transaction(&req.account_id, &req.transaction)
        .await?;
    
    Ok(web::Json(json!({
        "signature": hex::encode(&signature.signature),
        "public_key": hex::encode(&signature.public_key),
        "account_id": signature.account_id,
        "timestamp": signature.timestamp
    })))
}
```

## Testing and Validation

### Comprehensive Test Suite
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let keypair1 = GridTokenXKeyPair::generate().unwrap();
        let keypair2 = GridTokenXKeyPair::generate().unwrap();
        
        // AccountIds should be different
        assert_ne!(keypair1.account_id(), keypair2.account_id());
        
        // AccountIds should be valid format
        assert!(validate_account_id(keypair1.account_id()).unwrap());
        assert!(validate_account_id(keypair2.account_id()).unwrap());
        
        // AccountIds should start with "gx_"
        assert!(keypair1.account_id().starts_with("gx_"));
        assert!(keypair2.account_id().starts_with("gx_"));
    }
    
    #[test]
    fn test_account_id_derivation_consistency() {
        let keypair = GridTokenXKeyPair::generate().unwrap();
        let derived_id = derive_account_id(keypair.public_key());
        
        // Derived AccountId should match keypair's AccountId
        assert_eq!(keypair.account_id(), derived_id);
    }
    
    #[test]
    fn test_transaction_signing_and_verification() {
        let keypair = GridTokenXKeyPair::generate().unwrap();
        
        #[derive(Serialize)]
        struct TestTransaction {
            from: String,
            to: String,
            amount: u64,
        }
        
        let transaction = TestTransaction {
            from: keypair.account_id().to_string(),
            to: "gx_recipient".to_string(),
            amount: 1000,
        };
        
        // Sign transaction
        let signature = keypair.sign_transaction(&transaction).unwrap();
        
        // Verify signature
        let is_valid = verify_transaction_signature(&transaction, &signature).unwrap();
        assert!(is_valid);
        
        // Signature should contain correct AccountId
        assert_eq!(signature.account_id, keypair.account_id());
    }
}
```

## Production Considerations

### Hardware Security Modules (HSM)
```rust
// Production implementation with HSM integration
pub struct ProductionKeyManager {
    hsm_client: HsmClient,
    key_ids: HashMap<String, HsmKeyId>,
}

impl ProductionKeyManager {
    pub async fn sign_transaction_with_hsm(
        &self,
        account_id: &str,
        transaction_hash: &[u8]
    ) -> Result<Signature, HsmError> {
        let key_id = self.key_ids.get(account_id)
            .ok_or(HsmError::KeyNotFound)?;
        
        self.hsm_client.sign(*key_id, transaction_hash).await
    }
}
```

### Performance Optimizations
```rust
// Batch signature verification
pub fn verify_batch_signatures<T: Serialize>(
    transactions: &[(T, TransactionSignature)]
) -> Vec<bool> {
    transactions
        .par_iter() // Parallel processing
        .map(|(transaction, signature)| {
            verify_transaction_signature(transaction, signature)
                .unwrap_or(false)
        })
        .collect()
}
```

This comprehensive system provides secure, scalable, and user-friendly cryptographic key management for GridTokenX blockchain, ensuring that users can safely manage their assets while maintaining the highest security standards.
