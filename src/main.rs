use std::env;

use crate::input::run_app;
use crate::app::App;
use crate::api::ApiClient;
use ratatui::backend::CrosstermBackend;
use anyhow::{Context, Result};

mod app;
mod api;
mod crypto;
mod input;
mod models;
mod types;
mod ui;
mod utils;

// 主函数
fn main() -> Result<()> {
    // 读取环境变量（可选）
    let secret_key = env::var("HEILO_SECRET_KEY").ok();
    
    let api_url = env::var("HEILO_API_URL")
        .unwrap_or_else(|_| "http://localhost:8080".to_string());

    // 创建 API 客户端（可选，如果设置了环境变量才创建）
    let api_client = if secret_key.is_some() {
        Some(ApiClient::new(api_url, secret_key)
            .context("创建 API 客户端失败")?)
    } else {
        None
    };

    // 创建终端
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = ratatui::Terminal::new(backend)?;
    
    // 创建应用实例（支持本地模式）
    let mut app = App::new(api_client)
        .context("创建应用实例失败")?;
    
    // 运行应用
    run_app(&mut terminal, &mut app)?;
    
    Ok(())
}
