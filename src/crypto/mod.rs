//! # GridTokenX Cryptographic Key Management
//! 
//! This module provides secure key management, AccountId derivation, and transaction signing
//! for the GridTokenX blockchain system.

use crate::types::AccountId;
use crate::utils::SystemResult;
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};

/// Secure keypair management for GridTokenX accounts
#[derive(Clone)]
pub struct GridTokenXKeyPair {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
    account_id: String,
}

impl GridTokenXKeyPair {
    /// Generate a new keypair with AccountId
    pub fn generate() -> SystemResult<Self> {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();
        let account_id = derive_account_id(&verifying_key)?;
        
        Ok(Self {
            signing_key,
            verifying_key,
            account_id,
        })
    }
    
    /// Create keypair from existing private key bytes
    pub fn from_private_key_bytes(private_key_bytes: &[u8; 32]) -> SystemResult<Self> {
        let signing_key = SigningKey::from_bytes(private_key_bytes);
        let verifying_key = signing_key.verifying_key();
        let account_id = derive_account_id(&verifying_key)?;
        
        Ok(Self {
            signing_key,
            verifying_key,
            account_id,
        })
    }
    
    /// Get the AccountId
    pub fn account_id(&self) -> &str {
        &self.account_id
    }
    
    /// Get the verifying key (public key)
    pub fn verifying_key(&self) -> &VerifyingKey {
        &self.verifying_key
    }
    
    /// Sign a message
    pub fn sign(&self, message: &[u8]) -> Signature {
        self.signing_key.sign(message)
    }
    
    /// Verify a signature
    pub fn verify(&self, message: &[u8], signature: &Signature) -> Result<(), ed25519_dalek::SignatureError> {
        self.verifying_key.verify(message, signature)
    }
    
    /// Export private key bytes (use with extreme caution)
    pub fn export_private_key_bytes(&self) -> [u8; 32] {
        self.signing_key.to_bytes()
    }
    
    /// Export public key bytes
    pub fn export_public_key_bytes(&self) -> [u8; 32] {
        self.verifying_key.to_bytes()
    }
    
    /// Sign a transaction and include AccountId verification
    pub fn sign_transaction<T: Serialize>(&self, transaction: &T) -> SystemResult<TransactionSignature> {
        // Serialize transaction
        let transaction_json = serde_json::to_string(transaction)
            .map_err(|e| crate::SystemError::Internal(format!("Failed to serialize transaction: {}", e)))?;
        let transaction_bytes = transaction_json.as_bytes();
        
        // Create message hash
        let mut hasher = Sha256::new();
        hasher.update(transaction_bytes);
        hasher.update(self.account_id.as_bytes()); // Include AccountId
        let message_hash = hasher.finalize();
        
        // Sign the hash
        let signature = self.sign(&message_hash);
        
        Ok(TransactionSignature {
            signature: signature.to_bytes().to_vec(),
            public_key: self.verifying_key.to_bytes().to_vec(),
            account_id: self.account_id.clone(),
            timestamp: Utc::now(),
        })
    }
}

/// Transaction signature structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionSignature {
    pub signature: String, // Hex encoded signature
    pub public_key: String, // Hex encoded public key
    pub account_id: String,
    pub timestamp: DateTime<Utc>,
}

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
    pub async fn create_account(&self) -> SystemResult<String> {
        let keypair = GridTokenXKeyPair::generate()?;
        let account_id = keypair.account_id().to_string();
        
        // Add to lookup table
        let public_key_hash = account_id_to_public_key_hash(&account_id)?;
        
        let mut accounts = self.accounts.write().await;
        let mut lookup = self.account_lookup.write().await;
        
        accounts.insert(account_id.clone(), keypair);
        lookup.insert(public_key_hash, account_id.clone());
        
        crate::utils::logging::log_info(
            "AccountManager", 
            &format!("Created new account: {}", account_id)
        );
        
        Ok(account_id)
    }
    
    /// Import account from private key
    pub async fn import_account(&self, private_key: SecretKey) -> SystemResult<String> {
        let keypair = GridTokenXKeyPair::from_private_key(private_key)?;
        let account_id = keypair.account_id().to_string();
        
        // Validate AccountId
        if !validate_account_id(&account_id)? {
            return Err(crate::SystemError::InvalidInput("Generated AccountId is invalid".to_string()));
        }
        
        let public_key_hash = account_id_to_public_key_hash(&account_id)?;
        
        let mut accounts = self.accounts.write().await;
        let mut lookup = self.account_lookup.write().await;
        
        accounts.insert(account_id.clone(), keypair);
        lookup.insert(public_key_hash, account_id.clone());
        
        crate::utils::logging::log_info(
            "AccountManager", 
            &format!("Imported account: {}", account_id)
        );
        
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
    ) -> SystemResult<TransactionSignature> {
        let accounts = self.accounts.read().await;
        let keypair = accounts.get(account_id)
            .ok_or_else(|| crate::SystemError::NotFound(format!("Account not found: {}", account_id)))?;
            
        keypair.sign_transaction(transaction)
    }
    
    /// Verify transaction signature
    pub async fn verify_transaction_signature<T: Serialize>(
        &self,
        transaction: &T,
        signature: &TransactionSignature
    ) -> SystemResult<bool> {
        // Validate AccountId
        if !validate_account_id(&signature.account_id)? {
            return Ok(false);
        }
        
        // Recreate message hash
        let transaction_json = serde_json::to_string(transaction)
            .map_err(|e| crate::SystemError::Internal(format!("Failed to serialize transaction: {}", e)))?;
        let transaction_bytes = transaction_json.as_bytes();
        
        let mut hasher = Sha256::new();
        hasher.update(transaction_bytes);
        hasher.update(signature.account_id.as_bytes());
        let message_hash = hasher.finalize();
        
        // Verify signature
        let public_key = PublicKey::from_bytes(&signature.public_key)
            .map_err(|e| crate::SystemError::InvalidInput(format!("Invalid public key: {}", e)))?;
        let ed_signature = ed25519_dalek::Signature::from_bytes(&signature.signature)
            .map_err(|e| crate::SystemError::InvalidInput(format!("Invalid signature: {}", e)))?;
        
        match public_key.verify(&message_hash, &ed_signature) {
            Ok(()) => {
                // Additional check: verify AccountId matches public key
                let derived_account_id = derive_account_id(&public_key)?;
                Ok(derived_account_id == signature.account_id)
            }
            Err(_) => Ok(false)
        }
    }
    
    /// List all managed accounts
    pub async fn list_accounts(&self) -> Vec<String> {
        let accounts = self.accounts.read().await;
        accounts.keys().cloned().collect()
    }
    
    /// Get account count
    pub async fn account_count(&self) -> usize {
        let accounts = self.accounts.read().await;
        accounts.len()
    }
}

/// Derive AccountId from public key
/// 
/// For GridTokenX, we use a simplified approach:
/// 1. Take the public key bytes
/// 2. Hash with SHA256
/// 3. Take first 16 bytes
/// 4. Encode as hex
/// 5. Add "gx_" prefix
pub fn derive_account_id(public_key: &PublicKey) -> SystemResult<String> {
    let public_key_bytes = public_key.as_bytes();
    
    // Hash the public key
    let mut hasher = Sha256::new();
    hasher.update(public_key_bytes);
    let hash = hasher.finalize();
    
    // Take first 16 bytes and encode as hex
    let account_hash = &hash[..16];
    let account_hex = hex::encode(account_hash);
    
    // Add GridTokenX prefix
    Ok(format!("gx_{}", account_hex))
}

/// Validate AccountId format
pub fn validate_account_id(account_id: &str) -> SystemResult<bool> {
    // Check prefix
    if !account_id.starts_with("gx_") {
        return Ok(false);
    }
    
    // Extract hex part
    let hex_part = &account_id[3..];
    
    // Check hex format and length (should be 32 characters for 16 bytes)
    if hex_part.len() != 32 {
        return Ok(false);
    }
    
    // Verify it's valid hex
    match hex::decode(hex_part) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// Extract public key hash from AccountId
pub fn account_id_to_public_key_hash(account_id: &str) -> SystemResult<[u8; 20]> {
    if !validate_account_id(account_id)? {
        return Err(crate::SystemError::InvalidInput("Invalid AccountId format".to_string()));
    }
    
    let hex_part = &account_id[3..];
    let decoded = hex::decode(hex_part)
        .map_err(|e| crate::SystemError::InvalidInput(format!("Failed to decode AccountId hex: {}", e)))?;
    
    // For lookup purposes, we use first 20 bytes of the decoded hash
    // Since we only have 16 bytes, pad with zeros
    let mut hash = [0u8; 20];
    hash[..decoded.len().min(20)].copy_from_slice(&decoded[..decoded.len().min(20)]);
    
    Ok(hash)
}

/// Generate a random AccountId for testing purposes
pub fn generate_test_account_id() -> String {
    let keypair = GridTokenXKeyPair::generate().unwrap();
    keypair.account_id().to_string()
}

/// Verify that an AccountId corresponds to a given public key
pub fn verify_account_id_matches_public_key(account_id: &str, public_key: &PublicKey) -> SystemResult<bool> {
    let derived_account_id = derive_account_id(public_key)?;
    Ok(derived_account_id == account_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

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
    fn test_account_id_derivation() {
        let keypair = GridTokenXKeyPair::generate().unwrap();
        let public_key = keypair.public_key();
        
        // Derive AccountId manually
        let derived_account_id = derive_account_id(public_key).unwrap();
        
        // Should match keypair's AccountId
        assert_eq!(keypair.account_id(), &derived_account_id);
    }
    
    #[test]
    fn test_transaction_signing() {
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
        
        let signature = keypair.sign_transaction(&transaction).unwrap();
        
        // Signature should contain correct AccountId
        assert_eq!(signature.account_id, keypair.account_id());
        
        // Signature arrays should be correct length
        assert_eq!(signature.signature.len(), 64);
        assert_eq!(signature.public_key.len(), 32);
    }
    
    #[test]
    fn test_account_id_validation() {
        // Valid AccountId
        let keypair = GridTokenXKeyPair::generate().unwrap();
        assert!(validate_account_id(keypair.account_id()).unwrap());
        
        // Invalid AccountIds
        assert!(!validate_account_id("invalid").unwrap());
        assert!(!validate_account_id("gx").unwrap());
        assert!(!validate_account_id("notgx_something").unwrap());
        assert!(!validate_account_id("gx_toolong1234567890123456789012345678901234567890").unwrap());
        assert!(!validate_account_id("gx_invalid_hex_zzz123").unwrap());
    }
    
    #[test]
    fn test_verify_account_id_matches_public_key() {
        let keypair = GridTokenXKeyPair::generate().unwrap();
        
        // Should match
        assert!(verify_account_id_matches_public_key(keypair.account_id(), keypair.public_key()).unwrap());
        
        // Different keypair should not match
        let keypair2 = GridTokenXKeyPair::generate().unwrap();
        assert!(!verify_account_id_matches_public_key(keypair.account_id(), keypair2.public_key()).unwrap());
    }
    
    #[tokio::test]
    async fn test_account_manager() {
        let manager = AccountManager::new();
        
        // Create account
        let account_id = manager.create_account().await.unwrap();
        assert!(validate_account_id(&account_id).unwrap());
        
        // Retrieve account
        let account = manager.get_account(&account_id).await;
        assert!(account.is_some());
        assert_eq!(account.unwrap().account_id(), account_id);
        
        // Account count should be 1
        assert_eq!(manager.account_count().await, 1);
        
        // List accounts
        let accounts = manager.list_accounts().await;
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0], account_id);
    }
    
    #[tokio::test]
    async fn test_transaction_signature_verification() {
        let manager = AccountManager::new();
        
        #[derive(Serialize)]
        struct TestTransaction {
            from: String,
            to: String,
            amount: u64,
        }
        
        // Create account
        let account_id = manager.create_account().await.unwrap();
        
        let transaction = TestTransaction {
            from: account_id.clone(),
            to: "gx_recipient".to_string(),
            amount: 1000,
        };
        
        // Sign transaction
        let signature = manager.sign_transaction(&account_id, &transaction).await.unwrap();
        
        // Verify signature
        let is_valid = manager.verify_transaction_signature(&transaction, &signature).await.unwrap();
        assert!(is_valid);
        
        // Modify transaction and verify it fails
        let modified_transaction = TestTransaction {
            from: account_id.clone(),
            to: "gx_recipient".to_string(),
            amount: 2000, // Different amount
        };
        
        let is_valid_modified = manager.verify_transaction_signature(&modified_transaction, &signature).await.unwrap();
        assert!(!is_valid_modified);
    }
}
