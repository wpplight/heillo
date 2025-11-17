use serde::{Deserialize, Serialize};

/// Group 结构体，表示订阅组
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: String,
    /// 组名称（base64 编码的加密字符串，或本地组的明文）
    pub name: String,
    /// 是否为本地组（不连接远程服务器）
    #[serde(default)]
    pub is_local: bool,
    /// 远程 API 主机地址（仅远程组使用）
    #[serde(default)]
    pub api_host: Option<String>,
    /// 远程 API 端口（仅远程组使用）
    #[serde(default)]
    pub api_port: Option<String>,
    /// 远程 API 密钥（仅远程组使用，不序列化到 JSON）
    #[serde(skip)]
    pub api_secret_key: Option<String>,
}

/// Item 结构体，表示订阅项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    /// 标题（base64 编码的加密字符串）
    pub title: String,
    /// 描述（base64 编码的加密字符串）
    pub describe: String,
    /// 文本内容（base64 编码的加密字符串）
    pub text: String,
}

/// API 错误响应
#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// API 成功响应（用于更新/删除操作）
#[derive(Debug, Deserialize)]
pub struct MessageResponse {
    pub message: String,
}

