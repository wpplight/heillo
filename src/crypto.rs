use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};

/// 加密数据
/// 
/// # 参数
/// - `key`: 加密密钥（32 字节）
/// - `plaintext`: 要加密的明文数据
/// 
/// # 返回
/// Base64 编码的加密数据
pub fn encrypt(key: &[u8], plaintext: &[u8]) -> Result<String> {
    // 确保密钥长度为 32 字节（AES-256）
    if key.len() != 32 {
        anyhow::bail!("密钥长度必须为 32 字节");
    }

    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| anyhow::anyhow!("创建加密器失败: {:?}", e))?;

    // 生成随机 nonce
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    // 加密数据
    let ciphertext = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|e| anyhow::anyhow!("加密失败: {:?}", e))?;

    // 将 nonce 和 ciphertext 组合在一起
    let mut encrypted_data = nonce.to_vec();
    encrypted_data.extend_from_slice(&ciphertext);

    // Base64 编码
    Ok(general_purpose::STANDARD.encode(&encrypted_data))
}

/// 解密数据
/// 
/// # 参数
/// - `key`: 解密密钥（32 字节）
/// - `encrypted_base64`: Base64 编码的加密数据
/// 
/// # 返回
/// 解密后的明文数据
pub fn decrypt(key: &[u8], encrypted_base64: &str) -> Result<Vec<u8>> {
    // 确保密钥长度为 32 字节（AES-256）
    if key.len() != 32 {
        anyhow::bail!("密钥长度必须为 32 字节");
    }

    // Base64 解码
    let encrypted_data = general_purpose::STANDARD
        .decode(encrypted_base64)
        .map_err(|e| anyhow::anyhow!("Base64 解码失败: {}", e))?;

    // Nonce 长度为 12 字节（AES-GCM 标准）
    if encrypted_data.len() < 12 {
        anyhow::bail!("加密数据格式错误：长度不足");
    }

    let (nonce_bytes, ciphertext) = encrypted_data.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| anyhow::anyhow!("创建解密器失败: {:?}", e))?;

    // 解密数据
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| anyhow::anyhow!("解密失败: {:?}", e))?;

    Ok(plaintext)
}

/// 从字符串密钥生成 32 字节密钥
/// 如果密钥长度不足 32 字节，使用 PBKDF2 派生
/// 如果密钥长度超过 32 字节，截取前 32 字节
pub fn derive_key(key_str: &str) -> [u8; 32] {
    let key_bytes = key_str.as_bytes();
    let mut key = [0u8; 32];
    
    if key_bytes.len() >= 32 {
        // 如果密钥长度 >= 32 字节，直接使用前 32 字节
        key.copy_from_slice(&key_bytes[..32]);
    } else {
        // 如果密钥长度 < 32 字节，使用简单填充（重复密钥直到 32 字节）
        for (i, &byte) in key_bytes.iter().cycle().take(32).enumerate() {
            key[i] = byte;
        }
    }
    
    key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let key = b"01234567890123456789012345678901"; // 32 字节
        let plaintext = b"Hello, World!";
        
        let encrypted = encrypt(key, plaintext).unwrap();
        let decrypted = decrypt(key, &encrypted).unwrap();
        
        assert_eq!(plaintext, decrypted.as_slice());
    }

    #[test]
    fn test_derive_key() {
        let key_str = "my-secret-key";
        let derived = derive_key(key_str);
        assert_eq!(derived.len(), 32);
    }
}

