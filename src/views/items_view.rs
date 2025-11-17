use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::app::App;

/// 渲染 items 列表视图
pub fn render(f: &mut Frame, area: Rect, app: &mut App) {
    // 显示详细内容列表
    let items: Vec<ListItem> = app
        .detail_items
        .iter()
        .map(|item| {
            let line = Line::from(vec![
                Span::styled(&item.title, Style::default().fg(Color::Cyan)),
                Span::raw(" - "),
                Span::styled(&item.describe, Style::default().fg(Color::Yellow)),
            ]);
            ListItem::new(line).style(Style::default().fg(Color::White))
        })
        .collect();

    // 创建列表组件
    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("items"))
        .highlight_style(
            Style::default()
                .bg(Color::LightBlue)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    // 渲染列表
    f.render_stateful_widget(items, area, &mut app.detail_state);
}

