use crate::common::*;
use ::aes_gcm::{
    Aes256Gcm, Key, Nonce,
    aead::{Aead, AeadCore, KeyInit},
};
use ::argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use ::base64::{Engine as _, engine::general_purpose};
use ::serde::{Deserialize, Serialize};

pub struct JsonCrypto {
    cipher: Aes256Gcm,
}

impl JsonCrypto {
    pub fn init() -> Result<Self> {
        let key = Aes256Gcm::generate_key().map_err(map_server_err)?;
        let cipher = Aes256Gcm::new(&key);
        Ok(Self { cipher })
    }

    pub fn init_with_key(key: impl AsRef<str>) -> Result<Self> {
        let key = Key::<Aes256Gcm>::try_from(key.as_ref().as_bytes()).map_err(map_server_err)?;
        let cipher = Aes256Gcm::new(&key);
        Ok(Self { cipher })
    }
    pub fn init_with_bytes_key(key: &[u8]) -> Result<Self> {
        let key = Key::<Aes256Gcm>::try_from(key).map_err(map_server_err)?;
        let cipher = Aes256Gcm::new(&key);
        Ok(Self { cipher })
    }

    pub fn init_with_password(password: impl AsRef<str>, salt: impl AsRef<str>) -> Result<Self> {
        use pbkdf2::pbkdf2_hmac;
        use sha2::Sha256;

        let salt = salt.as_ref().as_bytes();

        if salt.len() < 16 {
            Err("Salt must be 16 bytes at least")?
        }

        let mut key = [0u8; 32];
        pbkdf2_hmac::<Sha256>(password.as_ref().as_bytes(), salt, 100_000, &mut key);

        Self::init_with_bytes_key(&key)
    }

    pub fn encrypt_binary<T>(&self, data: &T) -> Result<Vec<u8>>
    where
        T: Serialize,
    {
        let json_string = serde_json::to_string(data).map_err(map_server_err)?;

        let nonce = Aes256Gcm::generate_nonce().map_err(map_server_err)?;

        let ciphertext = self
            .cipher
            .encrypt(&nonce, json_string.as_bytes())
            .map_err(map_server_err)?;

        let mut result = Vec::with_capacity(nonce.len() + ciphertext.len());
        result.extend_from_slice(&nonce);
        result.extend_from_slice(&ciphertext);

        Ok(lz4_flex::compress_prepend_size(&result))
    }

    pub fn encrypt_json<T>(&self, data: &T) -> Result<String>
    where
        T: Serialize,
    {
        let json_string = serde_json::to_string(data).map_err(map_server_err)?;

        let nonce = Aes256Gcm::generate_nonce().map_err(map_server_err)?;

        let ciphertext = self
            .cipher
            .encrypt(&nonce, json_string.as_bytes())
            .map_err(map_server_err)?;

        let mut result = Vec::with_capacity(nonce.len() + ciphertext.len());
        result.extend_from_slice(&nonce);
        result.extend_from_slice(&ciphertext);

        Ok(general_purpose::STANDARD.encode(result))
    }

    pub fn decrypt_json<T>(&self, encrypted_data: impl AsRef<str>) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        if encrypted_data.as_ref().is_empty() {
            Err("Data is empty")?
        }

        let encrypted_bytes = general_purpose::STANDARD
            .decode(encrypted_data.as_ref().trim())
            .map_err(map_server_err)?;

        const NONCE_SIZE: usize = 12;
        if encrypted_bytes.len() < NONCE_SIZE + 1 {
            Err("Data is corrupted")?
        }

        let (nonce_bytes, ciphertext) = encrypted_bytes.split_at(NONCE_SIZE);
        let nonce = Nonce::try_from(nonce_bytes).map_err(map_server_err)?;

        let plaintext = self
            .cipher
            .decrypt(&nonce, ciphertext)
            .map_err(map_server_err)?;

        let json_string = String::from_utf8(plaintext).map_err(map_server_err)?;

        let result: T = serde_json::from_str(&json_string).map_err(map_server_err)?;
        Ok(result)
    }

    pub fn decrypt_binary<T>(&self, encrypted_data: &[u8]) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        if encrypted_data.is_empty() {
            Err(Error::from("Data is empty"))?
        }

        let encrypted_bytes =
            lz4_flex::decompress_size_prepended(encrypted_data).map_err(map_server_err)?;

        const NONCE_SIZE: usize = 12;
        if encrypted_bytes.len() < NONCE_SIZE + 1 {
            Err("Data is corrupted")?
        }

        let (nonce_bytes, ciphertext) = encrypted_bytes.split_at(NONCE_SIZE);
        let nonce = Nonce::try_from(nonce_bytes).map_err(map_server_err)?;

        let plaintext = self
            .cipher
            .decrypt(&nonce, ciphertext)
            .map_err(map_server_err)?;

        let json_string = String::from_utf8(plaintext).map_err(map_server_err)?;

        let result: T = serde_json::from_str(&json_string).map_err(map_server_err)?;
        Ok(result)
    }

    pub fn key_info(&self) -> String {
        "AES-256-GCM (32 byte key)".to_string()
    }
}

pub fn encrypt_json_string<T>(data: &T, key: impl AsRef<str>) -> Result<String>
where
    T: Serialize,
{
    JsonCrypto::init_with_key(key)?.encrypt_json(data)
}

pub fn decrypt_json_string<T>(encrypted_data: impl AsRef<str>, key: impl AsRef<str>) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    JsonCrypto::init_with_key(key)?.decrypt_json(encrypted_data)
}

pub fn generate_key() -> [u8; 32] {
    Aes256Gcm::generate_key().unwrap().into()
}

pub fn generate_salt() -> [u8; 32] {
    Aes256Gcm::generate_key().unwrap().into()
}

pub fn keys_equal(key1: &[u8], key2: &[u8]) -> bool {
    use subtle::ConstantTimeEq;
    key1.ct_eq(key2).into()
}

pub fn hash_password(password: impl AsRef<str>) -> Result<String> {
    let mut salt_bytes = [0u8; 16];
    getrandom::fill(&mut salt_bytes).map_err(map_server_err)?;

    let salt_b64 = general_purpose::STANDARD_NO_PAD.encode(salt_bytes);

    let salt = SaltString::from_b64(&salt_b64).map_err(map_server_err)?;

    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_ref().as_bytes(), &salt)
        .map_err(map_server_err)?;
    Ok(password_hash.to_string())
}

pub fn verify_password(password: impl AsRef<str>, hash: impl AsRef<str>) -> Result<()> {
    let parsed_hash = PasswordHash::new(hash.as_ref()).map_err(map_server_err)?;
    let argon2 = Argon2::default();

    argon2
        .verify_password(password.as_ref().as_bytes(), &parsed_hash)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "invalid-credentials"))
        .map(Ok)?
}
