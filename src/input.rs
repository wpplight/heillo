use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::io;
use std::thread;
use std::time::Duration;
use crate::types::DetailSelection;
use crate::app::App;
use crate::utils;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

/// 处理键盘输入事件
pub fn handle_key_event(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App
) -> io::Result<bool> {
    let mut should_exit = false;

    if let Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Press {
            match key.code {
                // 退出键处理
                KeyCode::Char('q') => {
                    if app.in_save_mode {
                        app.in_save_mode = false;
                    } else if app.in_edit_mode {
                        app.edit_buffer.push('q');
                    } else if app.in_detail_page {
                        app.in_detail_page = false;
                    } else if app.in_detail_view {
                        app.in_detail_view = false;
                    } else {
                        should_exit = true;
                    }
                }
                
                // Esc键处理
                KeyCode::Esc => {
                    if app.in_edit_mode {
                        app.in_edit_mode = false;
                        app.in_save_mode = true;
                    } else if app.in_save_mode {
                        app.in_save_mode = false;
                        app.in_edit_mode = true;
                    } else if app.in_detail_page {
                        app.in_detail_page = false;
                    } else if app.in_detail_view {
                        app.in_detail_view = false;
                    } else {
                        should_exit = true;
                    }
                }
                
                // 向下/向下移动键处理
                KeyCode::Down | KeyCode::Char('j') => {
                    if app.in_edit_mode {
                        app.edit_buffer.push('j');
                    } else {
                        if app.in_detail_page {
                            match app.current_detail_selection {
                                DetailSelection::Title => app.current_detail_selection = DetailSelection::Describe,
                                DetailSelection::Describe => app.current_detail_selection = DetailSelection::Text,
                                DetailSelection::Text => app.current_detail_selection = DetailSelection::Title,
                            }
                        } else if app.in_detail_view {
                            app.next_detail();
                        } else {
                            app.next();
                        }
                    }
                }
                
                // 向上/向上移动键处理
                KeyCode::Up | KeyCode::Char('k') => {
                    if app.in_edit_mode {
                        app.edit_buffer.push('k');
                    } else {
                        if app.in_detail_page {
                            match app.current_detail_selection {
                                DetailSelection::Title => app.current_detail_selection = DetailSelection::Text,
                                DetailSelection::Describe => app.current_detail_selection = DetailSelection::Title,
                                DetailSelection::Text => app.current_detail_selection = DetailSelection::Describe,
                            }
                        } else if app.in_detail_view {
                            app.previous_detail();
                        } else {
                            app.previous();
                        }
                    }
                }
                
                // 添加新项（a键）
                KeyCode::Char('a') => {
                    if app.in_edit_mode {
                        app.edit_buffer.push('a');
                    } else {
                        app.items.push(format!("新订阅项 {}", app.items.len() + 1));
                    }
                }
                
                // 删除当前项（d键）
                KeyCode::Char('d') => {
                    if app.in_edit_mode {
                        app.edit_buffer.push('d');
                    } else if app.in_detail_page && !app.in_save_mode {
                        // 在详情页面中，清空当前选中区块的内容
                        if app.current_detail_index < app.detail_items.len() {
                            match app.current_detail_selection {
                                DetailSelection::Title => {
                                    app.detail_items[app.current_detail_index].title.clear();
                                },
                                DetailSelection::Describe => {
                                    app.detail_items[app.current_detail_index].describe.clear();
                                },
                                DetailSelection::Text => {
                                    app.detail_items[app.current_detail_index].text.clear();
                                },
                            }
                        }
                    } else {
                        if !app.items.is_empty() {
                            let selected = app.state.selected().unwrap_or(0);
                            app.items.remove(selected);
                            
                            if app.items.is_empty() {
                                app.state.select(None);
                            } else {
                                let new_selected = if selected >= app.items.len() {
                                    app.items.len() - 1
                                } else {
                                    selected
                                };
                                app.state.select(Some(new_selected));
                            }
                        }
                    }
                }
                
                // 刷新排序（r键）
                KeyCode::Char('r') => {
                    if app.in_edit_mode {
                        app.edit_buffer.push('r');
                    } else {
                        app.items.sort();
                    }
                }
                
                // 切换置顶状态（t键）
                KeyCode::Char('t') => {
                    if app.in_edit_mode {
                        app.edit_buffer.push('t');
                    } else {
                        app.toggle_pin();
                    }
                }
                
                // 回车键处理
                KeyCode::Enter => {
                    if app.in_edit_mode {
                        app.edit_buffer.push('\n');
                    } else {
                        if !app.in_detail_view {
                            app.in_detail_view = true;
                            if !app.detail_items.is_empty() {
                                app.detail_state.select(Some(0));
                            }
                        } else if !app.in_detail_page {
                            if let Some(selected) = app.detail_state.selected() {
                                app.in_detail_page = true;
                                app.current_detail_index = selected;
                            }
                        }
                    }
                }
                
                // 编辑模式（v键）
                KeyCode::Char('v') => {
                    if app.in_edit_mode {
                        app.edit_buffer.push('v');
                    } else {
                        if app.in_detail_page && !app.in_edit_mode && !app.in_save_mode {
                            if app.current_detail_index < app.detail_items.len() {
                                let current_item = &app.detail_items[app.current_detail_index];
                                app.edit_buffer = match app.current_detail_selection {
                                    DetailSelection::Title => current_item.title.clone(),
                                    DetailSelection::Describe => current_item.describe.clone(),
                                    DetailSelection::Text => current_item.text.clone(),
                                };
                                app.in_edit_mode = true;
                            }
                        }
                    }
                }
                
                // 模拟键盘输出（b键）
                KeyCode::Char('b') => {
                    if app.in_edit_mode {
                        app.edit_buffer.push('b');
                    } else {
                        if app.in_detail_page && !app.detail_items.is_empty() {
                            if app.current_detail_index < app.detail_items.len() {
                                let text = app.detail_items[app.current_detail_index].text.clone();
                                thread::spawn(move || {
                                    thread::sleep(Duration::from_secs(2));
                                    utils::simulate_keyboard_output(&text);
                                });
                            }
                        } else if app.in_detail_view && !app.detail_items.is_empty() {
                            if let Some(selected) = app.detail_state.selected() {
                                if selected < app.detail_items.len() {
                                    let text = app.detail_items[selected].text.clone();
                                    thread::spawn(move || {
                                        thread::sleep(Duration::from_secs(2));
                                        utils::simulate_keyboard_output(&text);
                                    });
                                }
                            }
                        }
                    }
                }
                
                // 保存并前进（w键）
                KeyCode::Char('w') => {
                    if app.in_save_mode {
                        if app.current_detail_index < app.detail_items.len() {
                            let buffer = app.edit_buffer.clone();
                            match app.current_detail_selection {
                                DetailSelection::Title => {
                                    app.detail_items[app.current_detail_index].title = buffer;
                                },
                                DetailSelection::Describe => {
                                    app.detail_items[app.current_detail_index].describe = buffer;
                                },
                                DetailSelection::Text => {
                                    app.detail_items[app.current_detail_index].text = buffer;
                                },
                            }
                        }
                        app.in_save_mode = false;
                        app.edit_buffer.clear();
                    } else if app.in_edit_mode {
                        app.edit_buffer.push('w');
                    } else if app.in_detail_page {
                        if !app.detail_items.is_empty() {
                            let old_index = app.current_detail_index;
                            app.current_detail_index = (app.current_detail_index + 1) % app.detail_items.len();
                            if old_index != app.current_detail_index {
                                app.edit_buffer.clear();
                            }
                        }
                    } else if app.in_detail_view {
                        app.next_detail();
                    } else {
                        app.next();
                    }
                }
                
                // 字符输入
                KeyCode::Char(c) => {
                    if app.in_edit_mode {
                        app.edit_buffer.push(c);
                    }
                }
                
                // 退格键处理
                KeyCode::Backspace => {
                    if app.in_edit_mode {
                        app.edit_buffer.pop();
                    }
                }
                
                // 忽略其他按键
                _ => {}
            }
        }
    }
    
    Ok(!should_exit)
}

/// 运行应用程序主循环
pub fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| crate::ui::draw(f, app))?;

        let should_continue = handle_key_event(terminal, app)?;
        if !should_continue {
            break;
        }
    }
    
    Ok(())
}