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
    // 读取环境变量
    let secret_key = env::var("HEILO_SECRET_KEY")
        .context("请设置环境变量 HEILO_SECRET_KEY")?;
    
    let api_url = env::var("HEILO_API_URL")
        .unwrap_or_else(|_| "http://localhost:8080".to_string());

    // 创建 API 客户端
    let api_client = ApiClient::new(api_url, secret_key)
        .context("创建 API 客户端失败")?;

    // 创建终端
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = ratatui::Terminal::new(backend)?;
    
    // 创建应用实例
    let mut app = App::new(api_client)
        .context("创建应用实例失败")?;
    
    // 运行应用
    run_app(&mut terminal, &mut app)?;
    
    Ok(())
}
