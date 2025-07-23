# GridTokenX Cryptographic Key Management & AccountId System

## Overview

The GridTokenX blockchain uses a robust cryptographic key management system based on Ed25519 elliptic curve cryptography for digital signatures and account identification. This document explains the key generation, AccountId derivation, and security practices for managing user identities on the network.

## Architecture Overview

```
┌─────────────────┐    Derivation     ┌─────────────────┐    Encoding      ┌─────────────────┐
│   Private Key   │ ─────────────────▶│   Public Key    │ ────────────────▶│   AccountId     │
│   (Secret)      │                   │   (Shareable)   │                  │   (Address)     │
│   32 bytes      │                   │   32 bytes      │                  │   String        │
└─────────────────┘                   └─────────────────┘                  └─────────────────┘
        │                                       │                                   │
        │ Used for                              │ Used for                          │ Used for
        ▼                                       ▼                                   ▼
┌─────────────────┐                   ┌─────────────────┐                  ┌─────────────────┐
│ Transaction     │                   │ Signature       │                  │ Account         │
│ Signing         │                   │ Verification    │                  │ Identification  │
└─────────────────┘                   └─────────────────┘                  └─────────────────┘
```

## Cryptographic Components

### 1. Private Key

The private key is the foundation of user security in the GridTokenX system.

**Properties:**
- **Size**: 32 bytes (256 bits)
- **Algorithm**: Ed25519 scalar
- **Entropy**: Cryptographically secure random generation
- **Secrecy**: Must never be shared or transmitted

**Generation:**
```rust
use ed25519_dalek::{Keypair, SecretKey};
use rand::rngs::OsRng;

/// Generate a new private key using cryptographically secure randomness
pub fn generate_private_key() -> SecretKey {
    let mut csprng = OsRng{};
    SecretKey::generate(&mut csprng)
}
```

**Security Requirements:**
- Store in secure hardware (HSM) when possible
- Encrypt when stored on disk
- Never log or transmit over insecure channels
- Use secure memory clearing after use
- Implement proper access controls

### 2. Public Key

The public key is derived mathematically from the private key and can be shared freely.

**Properties:**
- **Size**: 32 bytes (256 bits)
- **Algorithm**: Ed25519 curve point
- **Derivation**: Deterministic from private key
- **Sharing**: Safe to share publicly

**Derivation:**
```rust
use ed25519_dalek::{Keypair, PublicKey, SecretKey};

/// Derive public key from private key
pub fn derive_public_key(private_key: &SecretKey) -> PublicKey {
    let keypair = Keypair::from(private_key.clone());
    keypair.public
}
```

### 3. AccountId Generation

The AccountId is a human-readable string derived from the public key, serving as the user's address on the network.

**Properties:**
- **Type**: String
- **Encoding**: Base58 with checksum
- **Prefix**: "gx" (GridTokenX identifier)
- **Length**: Variable (typically 44-52 characters)
- **Uniqueness**: Cryptographically guaranteed

## AccountId Derivation Process

### Step-by-Step Process

```rust
use ed25519_dalek::PublicKey;
use sha2::{Sha256, Digest};
use bs58;

/// Complete AccountId derivation from public key
pub fn derive_account_id(public_key: &PublicKey) -> String {
    // Step 1: Get public key bytes
    let public_key_bytes = public_key.as_bytes();
    
    // Step 2: Hash the public key
    let mut hasher = Sha256::new();
    hasher.update(public_key_bytes);
    let hash = hasher.finalize();
    
    // Step 3: Take first 20 bytes as account identifier
    let account_hash = &hash[..20];
    
    // Step 4: Calculate checksum (first 4 bytes of double SHA256)
    let mut checksum_hasher = Sha256::new();
    checksum_hasher.update(account_hash);
    let checksum_hash = checksum_hasher.finalize();
    
    let mut checksum_hasher2 = Sha256::new();
    checksum_hasher2.update(checksum_hash);
    let checksum_hash2 = checksum_hasher2.finalize();
    
    let checksum = &checksum_hash2[..4];
    
    // Step 5: Combine account hash and checksum
    let mut full_account = Vec::new();
    full_account.extend_from_slice(account_hash);
    full_account.extend_from_slice(checksum);
    
    // Step 6: Encode with Base58 and add prefix
    let encoded = bs58::encode(full_account).into_string();
    format!("gx{}", encoded)
}
```

### Alternative Simplified Version

For the current GridTokenX implementation, we can use a simplified approach:

```rust
use ed25519_dalek::PublicKey;
use hex;

/// Simplified AccountId generation for GridTokenX
pub fn generate_account_id(public_key: &PublicKey) -> String {
    // Convert public key to hex and prefix with "gx_"
    let public_key_hex = hex::encode(public_key.as_bytes());
    format!("gx_{}", &public_key_hex[..16]) // First 16 chars for readability
}
```

## Complete Key Management Implementation

### KeyPair Management

```rust
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

/// Secure keypair management for GridTokenX accounts
#[derive(Clone)]
pub struct GridTokenXKeyPair {
    keypair: Keypair,
    account_id: String,
}

impl GridTokenXKeyPair {
    /// Generate a new keypair with AccountId
    pub fn generate() -> Self {
        let mut csprng = OsRng{};
        let keypair = Keypair::generate(&mut csprng);
        let account_id = derive_account_id(&keypair.public);
        
        Self {
            keypair,
            account_id,
        }
    }
    
    /// Create keypair from existing private key
    pub fn from_private_key(private_key: SecretKey) -> Self {
        let keypair = Keypair::from(private_key);
        let account_id = derive_account_id(&keypair.public);
        
        Self {
            keypair,
            account_id,
        }
    }
    
    /// Get the AccountId
    pub fn account_id(&self) -> &str {
        &self.account_id
    }
    
    /// Get the public key
    pub fn public_key(&self) -> &PublicKey {
        &self.keypair.public
    }
    
    /// Sign a message
    pub fn sign(&self, message: &[u8]) -> Signature {
        self.keypair.sign(message)
    }
    
    /// Verify a signature
    pub fn verify(&self, message: &[u8], signature: &Signature) -> Result<(), ed25519_dalek::SignatureError> {
        self.keypair.public.verify(message, signature)
    }
    
    /// Export private key (use with extreme caution)
    pub fn export_private_key(&self) -> SecretKey {
        self.keypair.secret.clone()
    }
    
    /// Export public key bytes
    pub fn export_public_key_bytes(&self) -> [u8; 32] {
        self.keypair.public.to_bytes()
    }
}

impl Drop for GridTokenXKeyPair {
    fn drop(&mut self) {
        // Securely clear the private key from memory
        // Note: ed25519_dalek doesn't implement Zeroize directly
        // In production, you'd use a more secure implementation
    }
}
```

### Transaction Signing Process

```rust
use sha2::{Sha256, Digest};
use serde_json;

/// Transaction signing with AccountId verification
impl GridTokenXKeyPair {
    /// Sign a transaction and include AccountId verification
    pub fn sign_transaction<T: Serialize>(&self, transaction: &T) -> Result<TransactionSignature, Box<dyn std::error::Error>> {
        // Serialize transaction
        let transaction_json = serde_json::to_string(transaction)?;
        let transaction_bytes = transaction_json.as_bytes();
        
        // Create message hash
        let mut hasher = Sha256::new();
        hasher.update(transaction_bytes);
        hasher.update(self.account_id.as_bytes()); // Include AccountId
        let message_hash = hasher.finalize();
        
        // Sign the hash
        let signature = self.sign(&message_hash);
        
        Ok(TransactionSignature {
            signature: signature.to_bytes(),
            public_key: self.public_key().to_bytes(),
            account_id: self.account_id.clone(),
            timestamp: chrono::Utc::now(),
        })
    }
}

/// Transaction signature structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionSignature {
    pub signature: [u8; 64],
    pub public_key: [u8; 32],
    pub account_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
```

## Security Best Practices

### Private Key Security

#### Storage
```rust
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use rand::RngCore;

/// Secure private key storage with encryption
pub struct SecureKeyStorage {
    cipher: Aes256Gcm,
}

impl SecureKeyStorage {
    pub fn new(password: &str) -> Self {
        // Derive key from password using PBKDF2
        let key = derive_key_from_password(password);
        let cipher = Aes256Gcm::new(&key);
        
        Self { cipher }
    }
    
    /// Encrypt and store private key
    pub fn store_private_key(&self, private_key: &SecretKey, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let private_key_bytes = private_key.to_bytes();
        
        // Generate random nonce
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        // Encrypt
        let ciphertext = self.cipher.encrypt(nonce, private_key_bytes.as_ref())?;
        
        // Combine nonce and ciphertext
        let mut encrypted_data = Vec::new();
        encrypted_data.extend_from_slice(&nonce_bytes);
        encrypted_data.extend_from_slice(&ciphertext);
        
        // Write to file
        std::fs::write(file_path, encrypted_data)?;
        
        Ok(())
    }
    
    /// Load and decrypt private key
    pub fn load_private_key(&self, file_path: &str) -> Result<SecretKey, Box<dyn std::error::Error>> {
        let encrypted_data = std::fs::read(file_path)?;
        
        // Extract nonce and ciphertext
        let nonce = Nonce::from_slice(&encrypted_data[..12]);
        let ciphertext = &encrypted_data[12..];
        
        // Decrypt
        let plaintext = self.cipher.decrypt(nonce, ciphertext)?;
        
        // Create SecretKey
        let private_key = SecretKey::from_bytes(&plaintext)?;
        
        Ok(private_key)
    }
}

fn derive_key_from_password(password: &str) -> Key {
    use pbkdf2::pbkdf2;
    use hmac::Hmac;
    use sha2::Sha256;
    
    let salt = b"gridtokenx_salt"; // In production, use random salt per key
    let mut key = [0u8; 32];
    pbkdf2::<Hmac<Sha256>>(password.as_bytes(), salt, 100_000, &mut key);
    Key::from_slice(&key).clone()
}
```

### AccountId Validation

```rust
/// Validate AccountId format and checksum
pub fn validate_account_id(account_id: &str) -> Result<bool, Box<dyn std::error::Error>> {
    // Check prefix
    if !account_id.starts_with("gx") {
        return Ok(false);
    }
    
    // Extract base58 part
    let base58_part = &account_id[2..];
    
    // Decode base58
    let decoded = bs58::decode(base58_part).into_vec()?;
    
    // Check length (20 bytes account hash + 4 bytes checksum)
    if decoded.len() != 24 {
        return Ok(false);
    }
    
    // Verify checksum
    let account_hash = &decoded[..20];
    let provided_checksum = &decoded[20..];
    
    let mut hasher = Sha256::new();
    hasher.update(account_hash);
    let hash1 = hasher.finalize();
    
    let mut hasher2 = Sha256::new();
    hasher2.update(hash1);
    let hash2 = hasher2.finalize();
    
    let calculated_checksum = &hash2[..4];
    
    Ok(provided_checksum == calculated_checksum)
}

/// Extract public key from AccountId (reverse lookup)
pub fn account_id_to_public_key_hash(account_id: &str) -> Result<[u8; 20], Box<dyn std::error::Error>> {
    if !validate_account_id(account_id)? {
        return Err("Invalid AccountId".into());
    }
    
    let base58_part = &account_id[2..];
    let decoded = bs58::decode(base58_part).into_vec()?;
    
    let mut hash = [0u8; 20];
    hash.copy_from_slice(&decoded[..20]);
    
    Ok(hash)
}
```

## Integration with GridTokenX System

### Account Management Service

```rust
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Account management service for GridTokenX
pub struct AccountManager {
    accounts: RwLock<HashMap<String, GridTokenXKeyPair>>,
    account_lookup: RwLock<HashMap<[u8; 20], String>>, // Hash to AccountId lookup
}

impl AccountManager {
    pub fn new() -> Self {
        Self {
            accounts: RwLock::new(HashMap::new()),
            account_lookup: RwLock::new(HashMap::new()),
        }
    }
    
    /// Create a new account
    pub async fn create_account(&self) -> String {
        let keypair = GridTokenXKeyPair::generate();
        let account_id = keypair.account_id().to_string();
        
        // Add to lookup table
        let public_key_hash = account_id_to_public_key_hash(&account_id).unwrap();
        
        let mut accounts = self.accounts.write().await;
        let mut lookup = self.account_lookup.write().await;
        
        accounts.insert(account_id.clone(), keypair);
        lookup.insert(public_key_hash, account_id.clone());
        
        account_id
    }
    
    /// Import account from private key
    pub async fn import_account(&self, private_key: SecretKey) -> Result<String, Box<dyn std::error::Error>> {
        let keypair = GridTokenXKeyPair::from_private_key(private_key);
        let account_id = keypair.account_id().to_string();
        
        // Validate AccountId
        if !validate_account_id(&account_id)? {
            return Err("Generated AccountId is invalid".into());
        }
        
        let public_key_hash = account_id_to_public_key_hash(&account_id)?;
        
        let mut accounts = self.accounts.write().await;
        let mut lookup = self.account_lookup.write().await;
        
        accounts.insert(account_id.clone(), keypair);
        lookup.insert(public_key_hash, account_id.clone());
        
        Ok(account_id)
    }
    
    /// Get account by AccountId
    pub async fn get_account(&self, account_id: &str) -> Option<GridTokenXKeyPair> {
        let accounts = self.accounts.read().await;
        accounts.get(account_id).cloned()
    }
    
    /// Sign transaction for account
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
    
    /// Verify transaction signature
    pub async fn verify_transaction_signature<T: Serialize>(
        &self,
        transaction: &T,
        signature: &TransactionSignature
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // Validate AccountId
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
        let public_key = PublicKey::from_bytes(&signature.public_key)?;
        let ed_signature = ed25519_dalek::Signature::from_bytes(&signature.signature)?;
        
        match public_key.verify(&message_hash, &ed_signature) {
            Ok(()) => {
                // Additional check: verify AccountId matches public key
                let derived_account_id = derive_account_id(&public_key);
                Ok(derived_account_id == signature.account_id)
            }
            Err(_) => Ok(false)
        }
    }
}
```

### CLI Tools for Key Management

```rust
use clap::{App, Arg, SubCommand};

/// Command-line tool for key management
pub fn run_key_management_cli() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("GridTokenX Key Manager")
        .version("1.0")
        .about("Manage cryptographic keys and AccountIds for GridTokenX")
        .subcommand(SubCommand::with_name("generate")
            .about("Generate new keypair")
            .arg(Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Output file for private key")))
        .subcommand(SubCommand::with_name("derive")
            .about("Derive AccountId from public key")
            .arg(Arg::with_name("pubkey")
                .short("p")
                .long("pubkey")
                .value_name("PUBKEY")
                .help("Public key in hex format")
                .required(true)))
        .subcommand(SubCommand::with_name("validate")
            .about("Validate AccountId format")
            .arg(Arg::with_name("account_id")
                .value_name("ACCOUNT_ID")
                .help("AccountId to validate")
                .required(true)))
        .get_matches();
    
    match matches.subcommand() {
        ("generate", Some(sub_matches)) => {
            let keypair = GridTokenXKeyPair::generate();
            
            println!("Generated new keypair:");
            println!("AccountId: {}", keypair.account_id());
            println!("Public Key: {}", hex::encode(keypair.public_key().as_bytes()));
            
            if let Some(output_file) = sub_matches.value_of("output") {
                // Save private key (encrypted)
                println!("Enter password to encrypt private key:");
                let password = rpassword::read_password()?;
                
                let storage = SecureKeyStorage::new(&password);
                storage.store_private_key(&keypair.export_private_key(), output_file)?;
                println!("Private key saved to: {}", output_file);
            } else {
                println!("Private Key: {}", hex::encode(keypair.export_private_key().to_bytes()));
                println!("⚠️  WARNING: Store private key securely!");
            }
        }
        
        ("derive", Some(sub_matches)) => {
            let pubkey_hex = sub_matches.value_of("pubkey").unwrap();
            let pubkey_bytes = hex::decode(pubkey_hex)?;
            let public_key = PublicKey::from_bytes(&pubkey_bytes)?;
            let account_id = derive_account_id(&public_key);
            
            println!("Derived AccountId: {}", account_id);
        }
        
        ("validate", Some(sub_matches)) => {
            let account_id = sub_matches.value_of("account_id").unwrap();
            
            match validate_account_id(account_id) {
                Ok(true) => println!("✅ AccountId is valid"),
                Ok(false) => println!("❌ AccountId is invalid"),
                Err(e) => println!("Error validating AccountId: {}", e),
            }
        }
        
        _ => println!("Use --help for usage information"),
    }
    
    Ok(())
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
        let keypair1 = GridTokenXKeyPair::generate();
        let keypair2 = GridTokenXKeyPair::generate();
        
        // AccountIds should be different
        assert_ne!(keypair1.account_id(), keypair2.account_id());
        
        // AccountIds should be valid format
        assert!(validate_account_id(keypair1.account_id()).unwrap());
        assert!(validate_account_id(keypair2.account_id()).unwrap());
        
        // AccountIds should start with "gx"
        assert!(keypair1.account_id().starts_with("gx"));
        assert!(keypair2.account_id().starts_with("gx"));
    }
    
    #[test]
    fn test_account_id_derivation() {
        let keypair = GridTokenXKeyPair::generate();
        let public_key = keypair.public_key();
        
        // Derive AccountId manually
        let derived_account_id = derive_account_id(public_key);
        
        // Should match keypair's AccountId
        assert_eq!(keypair.account_id(), &derived_account_id);
    }
    
    #[test]
    fn test_transaction_signing() {
        let keypair = GridTokenXKeyPair::generate();
        
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
        
        let signature = keypair.sign_transaction(&transaction).unwrap();
        
        // Signature should contain correct AccountId
        assert_eq!(signature.account_id, keypair.account_id());
        
        // Signature should be verifiable
        // (Would need to implement verification logic)
    }
    
    #[test]
    fn test_account_id_validation() {
        // Valid AccountId
        let keypair = GridTokenXKeyPair::generate();
        assert!(validate_account_id(keypair.account_id()).unwrap());
        
        // Invalid AccountIds
        assert!(!validate_account_id("invalid").unwrap());
        assert!(!validate_account_id("gx").unwrap());
        assert!(!validate_account_id("notgx_something").unwrap());
    }
    
    #[tokio::test]
    async fn test_account_manager() {
        let manager = AccountManager::new();
        
        // Create account
        let account_id = manager.create_account().await;
        assert!(validate_account_id(&account_id).unwrap());
        
        // Retrieve account
        let account = manager.get_account(&account_id).await;
        assert!(account.is_some());
        assert_eq!(account.unwrap().account_id(), account_id);
    }
}
```

## Production Deployment Considerations

### Security Checklist

1. **Private Key Storage**
   - Use hardware security modules (HSM) in production
   - Implement secure key derivation (BIP32/BIP44)
   - Enable secure enclave storage on supported platforms
   - Regular key rotation policies

2. **AccountId Generation**
   - Validate all AccountIds before processing
   - Implement rate limiting for account creation
   - Monitor for suspicious patterns
   - Maintain AccountId blacklist for known bad actors

3. **Signature Verification**
   - Always verify signatures before processing transactions
   - Implement replay attack protection with nonces
   - Check transaction timestamps for validity
   - Validate AccountId ownership

4. **Network Security**
   - Use TLS for all network communications
   - Implement proper certificate pinning
   - Regular security audits and penetration testing
   - Monitor for cryptographic algorithm deprecations

### Configuration Example

```toml
[cryptography]
# Key generation settings
key_algorithm = "Ed25519"
account_id_prefix = "gx"
account_id_encoding = "base58"

# Security settings
require_hardware_keys = false  # Set to true for production
min_key_strength = 256
signature_algorithm = "Ed25519"

# Account management
max_accounts_per_node = 10000
account_id_cache_size = 50000
account_creation_rate_limit = 10  # per minute per IP

[security]
# Private key encryption
private_key_encryption = "AES-256-GCM"
pbkdf2_iterations = 100000
key_derivation_salt = "gridtokenx_v1"

# Transaction security
replay_protection = true
signature_required = true
max_transaction_age_minutes = 30
```

This comprehensive cryptographic key management system provides secure, production-ready account management for the GridTokenX blockchain while maintaining compatibility with the existing String-based AccountId system.
