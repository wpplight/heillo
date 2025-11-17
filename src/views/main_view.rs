use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::app::App;

/// 渲染主视图（groups 列表）
pub fn render(f: &mut Frame, area: Rect, app: &mut App) {
    // 创建列表项（显示解密后的组名称）
    let items: Vec<ListItem> = app
        .groups
        .iter()
        .map(|group| {
            let name = if group.is_local {
                // 本地组：直接显示组名称
                group.name.clone()
            } else {
                // 远程组：使用该组的 API 客户端解密
                if let Some(api_client) = app.group_api_clients.get(&group.id) {
                    api_client
                        .decrypt_group_name(&group.name)
                        .unwrap_or_else(|_| "解密失败".to_string())
                } else {
                    // 如果没有找到 API 客户端，显示原始名称
                    group.name.clone()
                }
            };
            let line = Line::from(name);
            ListItem::new(line).style(Style::default().fg(Color::White))
        })
        .collect();

    // 创建列表组件
    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("订阅组"))
        .highlight_style(
            Style::default()
                .bg(Color::LightBlue)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    // 渲染列表
    f.render_stateful_widget(items, area, &mut app.state);
}

