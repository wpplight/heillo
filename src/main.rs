use std::io;

use crate::input::run_app;
use crate::app::App;
use ratatui::backend::CrosstermBackend;

mod app;
mod input;
mod types;
mod ui;
mod utils;

// 主函数
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建终端
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = ratatui::Terminal::new(backend)?;
    
    // 创建应用实例
    let mut app = App::new();
    
    // 运行应用
    run_app(&mut terminal, &mut app)?;
    
    Ok(())
}
