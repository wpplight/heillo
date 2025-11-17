use crate::crypto::{decrypt, encrypt, derive_key};
use crate::models::{ErrorResponse, Group, Item};
use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde_json::json;
use std::time::Duration;

/// API 客户端，封装 HTTP 请求和加密/解密功能
pub struct ApiClient {
    client: Client,
    base_url: String,
    secret_key: Option<[u8; 32]>,
}

impl ApiClient {
    /// 创建新的 API 客户端
    /// 
    /// # 参数
    /// - `base_url`: API 服务器基础 URL
    /// - `secret_key`: 可选的加密密钥字符串
    pub fn new(base_url: String, secret_key: Option<String>) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .context("创建 HTTP 客户端失败")?;

        let secret_key = secret_key.map(|key: String| derive_key(&key));

        Ok(Self {
            client,
            base_url,
            secret_key,
        })
    }

    /// 列出所有组
    pub fn list_groups(&self) -> Result<Vec<Group>> {
        let url = format!("{}/api/groups", self.base_url);
        let response = self
            .client
            .get(&url)
            .send()
            .context("请求失败")?;

        let status = response.status();
        if !status.is_success() {
            let error: ErrorResponse = response.json().unwrap_or(ErrorResponse {
                error: format!("HTTP {}", status),
            });
            anyhow::bail!("{}", error.error);
        }

        let groups: Vec<Group> = response.json().context("解析响应失败")?;
        Ok(groups)
    }

    /// 创建组
    pub fn create_group(&self, id: &str, name: &str, use_encryption: bool) -> Result<Group> {
        let name_value = if use_encryption {
            if let Some(secret_key) = &self.secret_key {
                encrypt(secret_key, name.as_bytes())?
            } else {
                anyhow::bail!("没有设置加密密钥")
            }
        } else {
            name.to_string()
        };

        let url = format!("{}/api/groups", self.base_url);
        let body = json!({
            "id": id,
            "name": name_value
        });

        let response = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .context("请求失败")?;

        let status = response.status();
        if !status.is_success() {
            let error: ErrorResponse = response.json().unwrap_or(ErrorResponse {
                error: format!("HTTP {}", status),
            });
            anyhow::bail!("{}", error.error);
        }

        let group: Group = response.json().context("解析响应失败")?;
        Ok(group)
    }

    /// 获取组
    pub fn get_group(&self, id: &str) -> Result<Group> {
        let url = format!("{}/api/groups/{}", self.base_url, id);
        let response = self
            .client
            .get(&url)
            .send()
            .context("请求失败")?;

        let status = response.status();
        if !status.is_success() {
            let error: ErrorResponse = response.json().unwrap_or(ErrorResponse {
                error: format!("HTTP {}", status),
            });
            anyhow::bail!("{}", error.error);
        }

        let group: Group = response.json().context("解析响应失败")?;
        Ok(group)
    }

    /// 更新组
    pub fn update_group(&self, id: &str, name: &str, use_encryption: bool) -> Result<()> {
        let name_value = if use_encryption {
            if let Some(secret_key) = &self.secret_key {
                encrypt(secret_key, name.as_bytes())?
            } else {
                anyhow::bail!("没有设置加密密钥")
            }
        } else {
            name.to_string()
        };

        let url = format!("{}/api/groups/{}", self.base_url, id);
        let body = json!({
            "id": id,
            "name": name_value
        });

        let response = self
            .client
            .put(&url)
            .json(&body)
            .send()
            .context("请求失败")?;

        let status = response.status();
        if !status.is_success() {
            let error: ErrorResponse = response.json().unwrap_or(ErrorResponse {
                error: format!("HTTP {}", status),
            });
            anyhow::bail!("{}", error.error);
        }

        Ok(())
    }

    /// 删除组
    pub fn delete_group(&self, id: &str) -> Result<()> {
        let url = format!("{}/api/groups/{}", self.base_url, id);
        let response = self
            .client
            .delete(&url)
            .send()
            .context("请求失败")?;

        let status = response.status();
        if !status.is_success() {
            let error: ErrorResponse = response.json().unwrap_or(ErrorResponse {
                error: format!("HTTP {}", status),
            });
            anyhow::bail!("{}", error.error);
        }

        Ok(())
    }

    /// 列出组内所有 items
    pub fn list_items(&self, group_id: &str) -> Result<Vec<Item>> {
        let url = format!("{}/api/groups/{}/items", self.base_url, group_id);
        let response = self
            .client
            .get(&url)
            .send()
            .context("请求失败")?;

        let status = response.status();
        if !status.is_success() {
            let error: ErrorResponse = response.json().unwrap_or(ErrorResponse {
                error: format!("HTTP {}", status),
            });
            anyhow::bail!("{}", error.error);
        }

        let items: Vec<Item> = response.json().context("解析响应失败")?;
        Ok(items)
    }

    /// 创建 item
    pub fn create_item(
        &self,
        group_id: &str,
        id: &str,
        title: &str,
        describe: &str,
        text: &str,
        use_encryption: bool
    ) -> Result<Item> {
        let encrypt_field = |field: &str| -> Result<String> {
            if use_encryption {
                if let Some(secret_key) = &self.secret_key {
                    encrypt(secret_key, field.as_bytes())
                } else {
                    anyhow::bail!("没有设置加密密钥")
                }
            } else {
                Ok(field.to_string())
            }
        };

        let encrypted_title = encrypt_field(title)?;
        let encrypted_describe = encrypt_field(describe)?;
        let encrypted_text = encrypt_field(text)?;

        let url = format!("{}/api/groups/{}/items", self.base_url, group_id);
        let body = json!({
            "id": id,
            "title": encrypted_title,
            "describe": encrypted_describe,
            "text": encrypted_text
        });

        let response = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .context("请求失败")?;

        let status = response.status();
        if !status.is_success() {
            let error: ErrorResponse = response.json().unwrap_or(ErrorResponse {
                error: format!("HTTP {}", status),
            });
            anyhow::bail!("{}", error.error);
        }

        let item: Item = response.json().context("解析响应失败")?;
        Ok(item)
    }

    /// 获取 item
    pub fn get_item(&self, group_id: &str, item_id: &str) -> Result<Item> {
        let url = format!("{}/api/groups/{}/items/{}", self.base_url, group_id, item_id);
        let response = self
            .client
            .get(&url)
            .send()
            .context("请求失败")?;

        let status = response.status();
        if !status.is_success() {
            let error: ErrorResponse = response.json().unwrap_or(ErrorResponse {
                error: format!("HTTP {}", status),
            });
            anyhow::bail!("{}", error.error);
        }

        let item: Item = response.json().context("解析响应失败")?;
        Ok(item)
    }

    /// 更新 item
    pub fn update_item(
        &self,
        group_id: &str,
        item_id: &str,
        title: &str,
        describe: &str,
        text: &str,
        use_encryption: bool
    ) -> Result<()> {
        let encrypt_field = |field: &str| -> Result<String> {
            if use_encryption {
                if let Some(secret_key) = &self.secret_key {
                    encrypt(secret_key, field.as_bytes())
                } else {
                    anyhow::bail!("没有设置加密密钥")
                }
            } else {
                Ok(field.to_string())
            }
        };

        let encrypted_title = encrypt_field(title)?;
        let encrypted_describe = encrypt_field(describe)?;
        let encrypted_text = encrypt_field(text)?;

        let url = format!("{}/api/groups/{}/items/{}", self.base_url, group_id, item_id);
        let body = json!({
            "id": item_id,
            "title": encrypted_title,
            "describe": encrypted_describe,
            "text": encrypted_text
        });

        let response = self
            .client
            .put(&url)
            .json(&body)
            .send()
            .context("请求失败")?;

        let status = response.status();
        if !status.is_success() {
            let error: ErrorResponse = response.json().unwrap_or(ErrorResponse {
                error: format!("HTTP {}", status),
            });
            anyhow::bail!("{}", error.error);
        }

        Ok(())
    }

    /// 删除 item
    pub fn delete_item(&self, group_id: &str, item_id: &str) -> Result<()> {
        let url = format!("{}/api/groups/{}/items/{}", self.base_url, group_id, item_id);
        let response = self
            .client
            .delete(&url)
            .send()
            .context("请求失败")?;

        let status = response.status();
        if !status.is_success() {
            let error: ErrorResponse = response.json().unwrap_or(ErrorResponse {
                error: format!("HTTP {}", status),
            });
            anyhow::bail!("{}", error.error);
        }

        Ok(())
    }

    /// 解密组名称
    pub fn decrypt_group_name(&self, encrypted_name: &str) -> Result<String> {
        if let Some(secret_key) = &self.secret_key {
            let decrypted = decrypt(secret_key, encrypted_name)
                .context("解密组名称失败")?;
            String::from_utf8(decrypted).context("转换为字符串失败")
        } else {
            // 如果没有密钥，直接返回原始字符串
            Ok(encrypted_name.to_string())
        }
    }

    /// 解密 item 字段
    pub fn decrypt_item_field(&self, encrypted_field: &str) -> Result<String> {
        if let Some(secret_key) = &self.secret_key {
            let decrypted = decrypt(secret_key, encrypted_field)
                .context("解密字段失败")?;
            String::from_utf8(decrypted).context("转换为字符串失败")
        } else {
            // 如果没有密钥，直接返回原始字符串
            Ok(encrypted_field.to_string())
        }
    }
}

