use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;
use crate::types::DetailSelection;

/// 渲染 item 详情页面
pub fn render(f: &mut Frame, area: Rect, app: &mut App) {
    if app.current_detail_index >= app.detail_items.len() {
        return;
    }

    let current_item = &app.detail_items[app.current_detail_index];
    
    // 创建垂直布局用于显示详细信息
    let detail_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(50),
        ])
        .margin(1)
        .split(area);

    // 创建标题区块
    let title_block = Block::default()
        .borders(Borders::ALL)
        .title("标题");
    let title_paragraph = Paragraph::new(
        if (app.in_edit_mode || app.in_save_mode) && app.current_detail_selection == DetailSelection::Title {
            app.edit_buffer.clone()
        } else {
            current_item.title.clone()
        }
    )
    .block(title_block)
    .style(
        if app.current_detail_selection == DetailSelection::Title {
            if app.in_edit_mode {
                Style::default().fg(Color::White).bg(Color::DarkGray)
            } else {
                Style::default().fg(Color::White).bg(Color::Blue)
            }
        } else {
            Style::default().fg(Color::White)
        }
    );

    // 创建描述区块
    let describe_block = Block::default()
        .borders(Borders::ALL)
        .title("描述");
    let describe_paragraph = Paragraph::new(
        if (app.in_edit_mode || app.in_save_mode) && app.current_detail_selection == DetailSelection::Describe {
            app.edit_buffer.clone()
        } else {
            current_item.describe.clone()
        }
    )
    .block(describe_block)
    .style(
        if app.current_detail_selection == DetailSelection::Describe {
            if app.in_edit_mode {
                Style::default().fg(Color::White).bg(Color::DarkGray)
            } else {
                Style::default().fg(Color::White).bg(Color::Blue)
            }
        } else {
            Style::default().fg(Color::White)
        }
    );

    // 创建文本区块
    let text_block = Block::default()
        .borders(Borders::ALL)
        .title("文本");
    let text_paragraph = Paragraph::new(
        if (app.in_edit_mode || app.in_save_mode) && app.current_detail_selection == DetailSelection::Text {
            app.edit_buffer.clone()
        } else {
            current_item.text.clone()
        }
    )
    .block(text_block)
    .style(
        if app.current_detail_selection == DetailSelection::Text {
            if app.in_edit_mode {
                Style::default().fg(Color::White).bg(Color::DarkGray)
            } else {
                Style::default().fg(Color::White).bg(Color::Blue)
            }
        } else {
            Style::default().fg(Color::White)
        }
    );
        
    // 渲染三个区块
    f.render_widget(title_paragraph, detail_chunks[0]);
    f.render_widget(describe_paragraph, detail_chunks[1]);
    f.render_widget(text_paragraph, detail_chunks[2]);
}

