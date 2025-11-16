use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::types::{DetailItem, DetailSelection};
use crate::app::App;

// UI渲染函数
pub fn draw(f: &mut Frame, app: &mut App) {
    // 创建主布局
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(f.size());

    // 如果在详情页面中
    if app.in_detail_page && !app.detail_items.is_empty() {
        // 显示详细内容
        if app.current_detail_index < app.detail_items.len() {
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
                .split(chunks[0]);

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
    } else if app.in_detail_view {
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
        f.render_stateful_widget(items, chunks[0], &mut app.detail_state);
    } else {
        // 显示主订阅列表
        // 创建列表项
        let items: Vec<ListItem> = app
            .items
            .iter()
            .map(|i| {
                let line = Line::from(i.as_str());
                ListItem::new(line).style(Style::default().fg(Color::White))
            })
            .collect();

        // 创建列表组件
        let items = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("订阅"))
            .highlight_style(
                Style::default()
                    .bg(Color::LightBlue)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        // 渲染列表
        f.render_stateful_widget(items, chunks[0], &mut app.state);
    }

    // 创建说明栏文本
    let help_text = get_help_text(app);

    // 创建说明栏组件
    let help_paragraph = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title("操作说明"))
        .style(Style::default().fg(Color::Gray));

    // 渲染说明栏
    f.render_widget(help_paragraph, chunks[1]);
}

// 生成帮助文本
fn get_help_text(app: &App) -> Vec<Line> {
    if app.in_detail_page {
        if app.in_save_mode {
            vec![
                Line::from(vec![
                    Span::styled("选择保存方式:", Style::default().fg(Color::Green)),
                    Span::raw("  "),
                ]),
                Line::from(vec![
                    Span::styled("q", Style::default().fg(Color::Yellow)),
                    Span::raw(" - 不保存退出  "),
                    Span::styled("w", Style::default().fg(Color::Yellow)),
                    Span::raw(" - 保存并退出"),
                ]),
                Line::from(vec![
                    Span::styled("Esc", Style::default().fg(Color::Yellow)),
                    Span::raw(" - 返回编辑模式"),
                ]),
            ]
        } else if app.in_edit_mode {
            vec![
                Line::from(vec![
                    Span::styled("Esc", Style::default().fg(Color::Yellow)),
                    Span::raw(" - 完成编辑"),
                ]),
                Line::from(vec![
                    Span::styled("字符输入", Style::default().fg(Color::Yellow)),
                    Span::raw(" - 输入文本  "),
                    Span::styled("Backspace", Style::default().fg(Color::Yellow)),
                    Span::raw(" - 删除字符  "),
                    Span::styled("Enter", Style::default().fg(Color::Yellow)),
                    Span::raw(" - 换行"),
                ]),
            ]
        } else {
            vec![
                Line::from(vec![
                    Span::styled("↑/↓/j/k", Style::default().fg(Color::Yellow)),
                    Span::raw(" - 切换选择项  "),
                    Span::styled("v", Style::default().fg(Color::Yellow)),
                    Span::raw(" - 编辑选中项"),
                ]),
                Line::from(vec![
                    Span::styled("d", Style::default().fg(Color::Yellow)),
                    Span::raw(" - 清空选中项内容  "),
                    Span::styled("q/Esc", Style::default().fg(Color::Yellow)),
                    Span::raw(" - 返回items列表  "),
                    Span::styled("b", Style::default().fg(Color::Yellow)),
                    Span::raw(" - 模拟键盘输出(2秒后)"),
                ]),
            ]
        }
    } else if app.in_detail_view {
        vec![
            Line::from(vec![
                Span::styled("q/Esc", Style::default().fg(Color::Yellow)),
                Span::raw(" - 返回主列表  "),
                Span::styled("↑/↓/j/k", Style::default().fg(Color::Yellow)),
                Span::raw(" - 上下导航  "),
                Span::styled("Enter", Style::default().fg(Color::Yellow)),
                Span::raw(" - 查看项目详情"),
            ]),
        ]
    } else {
        vec![
            Line::from(vec![
                Span::styled("q/Esc", Style::default().fg(Color::Yellow)),
                Span::raw(" - 退出程序  "),
                Span::styled("↑/↓/j/k", Style::default().fg(Color::Yellow)),
                Span::raw(" - 上下导航  "),
                Span::styled("Enter", Style::default().fg(Color::Yellow)),
                Span::raw(" - 选择项目"),
                Span::raw(" - 刷新列表  "),
                Span::styled("t", Style::default().fg(Color::Yellow)),
                Span::raw(" - "),
                Span::styled(
                    if app.is_pinned { "取消置顶" } else { "窗口置顶" },
                    Style::default().fg(if app.is_pinned { Color::Red } else { Color::Green })
                ),
            ]),
            Line::from(vec![
                Span::styled("a", Style::default().fg(Color::Yellow)),
                Span::raw(" - 添加订阅  "),
                Span::styled("d", Style::default().fg(Color::Yellow)),
                Span::raw(" - 删除订阅  "),
                Span::styled("r", Style::default().fg(Color::Yellow)),
                Span::raw(" - 刷新列表"),
            ]),
        ]
    }
}