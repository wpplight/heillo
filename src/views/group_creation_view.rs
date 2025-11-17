use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;
use crate::types::GroupCreationField;

/// 渲染组创建页面
pub fn render(f: &mut Frame, area: Rect, app: &mut App) {
    let creation_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(area);

    // 显示标题
    let title_paragraph = Paragraph::new("创建新订阅组 (不输入 host 和 port 则创建本地组)")
        .block(Block::default().borders(Borders::ALL).title("创建组"))
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(title_paragraph, creation_chunks[0]);

    // 显示输入字段（高亮当前选中的字段）
    let host_text = format!("主机: {}", app.group_creation_host);
    let host_style = if app.group_creation_field == GroupCreationField::Host {
        Style::default().fg(Color::White).bg(Color::Blue)
    } else {
        Style::default().fg(Color::White)
    };
    let host_paragraph = Paragraph::new(host_text)
        .block(Block::default().borders(Borders::ALL).title("主机地址"))
        .style(host_style);
    f.render_widget(host_paragraph, creation_chunks[1]);

    let port_text = format!("端口: {}", app.group_creation_port);
    let port_style = if app.group_creation_field == GroupCreationField::Port {
        Style::default().fg(Color::White).bg(Color::Blue)
    } else {
        Style::default().fg(Color::White)
    };
    let port_paragraph = Paragraph::new(port_text)
        .block(Block::default().borders(Borders::ALL).title("端口"))
        .style(port_style);
    f.render_widget(port_paragraph, creation_chunks[2]);

    let key_text = format!("密钥: {}", app.group_creation_secret_key);
    let key_style = if app.group_creation_field == GroupCreationField::SecretKey {
        Style::default().fg(Color::White).bg(Color::Blue)
    } else {
        Style::default().fg(Color::White)
    };
    let key_paragraph = Paragraph::new(key_text)
        .block(Block::default().borders(Borders::ALL).title("密钥"))
        .style(key_style);
    f.render_widget(key_paragraph, creation_chunks[3]);

    // 显示操作说明
    let instruction_text = "↑/↓/j/k: 切换输入字段 | Enter/'a': 创建组 | 'q': 取消";
    let instruction_paragraph = Paragraph::new(instruction_text)
        .block(Block::default().borders(Borders::ALL).title("操作"))
        .style(Style::default().fg(Color::Green));
    f.render_widget(instruction_paragraph, creation_chunks[4]);
}

