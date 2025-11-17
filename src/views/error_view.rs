use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;

/// 渲染错误消息视图
/// 如果有错误消息，在指定区域显示；否则不渲染
pub fn render(f: &mut Frame, area: Rect, app: &mut App) {
    if let Some(error) = &app.error_message {
        let error_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(area);
        
        let error_paragraph = Paragraph::new(error.as_str())
            .block(Block::default().borders(Borders::ALL).title("错误").border_style(Style::default().fg(Color::Red)))
            .style(Style::default().fg(Color::Red));
        
        f.render_widget(error_paragraph, error_chunks[1]);
    }
}

/// 获取错误消息占用的区域（如果有错误）
pub fn get_error_area(area: Rect, _app: &App) -> (Rect, Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(area);
    (chunks[0], chunks[1])
}

