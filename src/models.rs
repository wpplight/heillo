use serde::{Deserialize, Serialize};

/// Group 结构体，表示订阅组
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: String,
    /// 组名称（base64 编码的加密字符串）
    pub name: String,
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

