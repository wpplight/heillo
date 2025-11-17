use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;

/// 渲染帮助文本视图
pub fn render(f: &mut Frame, area: Rect, app: &mut App) {
    let help_text = get_help_text(app);
    let help_paragraph = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title("操作说明"))
        .style(Style::default().fg(Color::Gray));

    f.render_widget(help_paragraph, area);
}

/// 生成帮助文本
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
                Span::raw(" - 查看项目详情  "),
                Span::styled("a", Style::default().fg(Color::Yellow)),
                Span::raw(" - 添加项目  "),
                Span::styled("d", Style::default().fg(Color::Yellow)),
                Span::raw(" - 删除项目"),
            ]),
        ]
    } else if app.in_group_creation {
        vec![
            Line::from(vec![
                Span::styled("↑/↓/j/k", Style::default().fg(Color::Yellow)),
                Span::raw(" - 切换输入字段  "),
                Span::styled("Enter/'a'", Style::default().fg(Color::Yellow)),
                Span::raw(" - 创建组  "),
                Span::styled("'q'", Style::default().fg(Color::Yellow)),
                Span::raw(" - 取消"),
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
                Span::raw(" - 选择组  "),
                Span::styled("r", Style::default().fg(Color::Yellow)),
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
                Span::raw(" - 添加订阅组  "),
                Span::styled("d", Style::default().fg(Color::Yellow)),
                Span::raw(" - 删除订阅组"),
            ]),
        ]
    }
}

