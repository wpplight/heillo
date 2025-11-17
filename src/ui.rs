use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::app::App;
use crate::views::{
    detail_page_view, error_view, group_creation_view, help_view, items_view, main_view,
};

/// UI 渲染函数（视图路由器）
pub fn draw(f: &mut Frame, app: &mut App) {
    // 创建主布局
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(f.size());

    // 根据应用状态路由到对应的视图
    if app.in_detail_page && !app.detail_items.is_empty() {
        // 详情页面
        detail_page_view::render(f, chunks[0], app);
    } else if app.in_group_creation {
        // 组创建页面
        group_creation_view::render(f, chunks[0], app);
    } else if app.in_detail_view {
        // Items 列表视图
        items_view::render(f, chunks[0], app);
    } else {
        // 主视图（groups 列表）
        main_view::render(f, chunks[0], app);
    }

    // 渲染底部区域（帮助文本和错误消息）
    if app.error_message.is_some() {
        // 如果有错误，分割区域显示错误和帮助
        let (help_area, error_area) = error_view::get_error_area(chunks[1], app);
        help_view::render(f, help_area, app);
        error_view::render(f, error_area, app);
    } else {
        // 只显示帮助
        help_view::render(f, chunks[1], app);
    }
}
