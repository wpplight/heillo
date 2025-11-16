use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::io;
use std::thread;
use std::time::Duration;
use crate::types::{DetailItem, DetailSelection};
use crate::ui;
use crate::utils;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::widgets::ListState;

pub struct App {
    pub items: Vec<String>,
    pub state: ListState,
    pub is_pinned: bool,
    pub detail_items: Vec<DetailItem>,
    pub in_detail_view: bool,
    pub in_detail_page: bool,
    pub detail_state: ListState,
    pub current_detail_index: usize,
    pub current_detail_selection: DetailSelection,
    pub in_edit_mode: bool,
    pub in_save_mode: bool,
    pub edit_buffer: String,
}

impl App {
    pub fn new() -> App {
        let mut state = ListState::default();
        state.select(Some(0));
        
        let mut detail_state = ListState::default();
        detail_state.select(Some(0));
        
        let detail_items = vec![
            DetailItem {
                title: "学习Rust".to_string(),
                describe: "掌握Rust编程语言的基础知识".to_string(),
                text: "println!(\"Hello, Rust!\");".to_string(),
            },
            DetailItem {
                title: "构建TUI应用".to_string(),
                describe: "使用Ratatui库创建终端用户界面".to_string(),
                text: "let app = App::new();".to_string(),
            },
            DetailItem {
                title: "探索Ratatui".to_string(),
                describe: "深入了解Ratatui的各种组件和功能".to_string(),
                text: "terminal.draw(|f| ui(f, &mut app))?;".to_string(),
            },
        ];
        
        App {
            items: vec![
                "Item 1: Learn Rust".to_string(),
                "Item 2: Build TUI applications".to_string(),
                "Item 3: Explore Ratatui".to_string(),
                "Item 4: Create smart lists".to_string(),
                "Item 5: Handle user input".to_string(),
                "Item 6: Navigate with keyboard".to_string(),
                "Item 7: Style with colors".to_string(),
                "Item 8: Manage state".to_string(),
                "Item 9: Build interactive UIs".to_string(),
                "Item 10: Deploy applications".to_string(),
                "Item 11: Test with various inputs".to_string(),
                "Item 12: Document the code".to_string(),
                "Item 13: Share with community".to_string(),
                "Item 14: Get feedback".to_string(),
                "Item 15: Improve based on feedback".to_string(),
            ],
            state,
            is_pinned: false,
            detail_items,
            in_detail_view: false,
            in_detail_page: false,
            detail_state,
            current_detail_index: 0,
            current_detail_selection: DetailSelection::Title,
            in_edit_mode: false,
            in_save_mode: false,
            edit_buffer: String::new(),
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn next_detail(&mut self) {
        if self.detail_items.is_empty() {
            return;
        }
        
        let i = match self.detail_state.selected() {
            Some(i) => {
                if i >= self.detail_items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.detail_state.select(Some(i));
    }

    pub fn previous_detail(&mut self) {
        if self.detail_items.is_empty() {
            return;
        }
        
        let i = match self.detail_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.detail_items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.detail_state.select(Some(i));
    }

    pub fn toggle_pin(&mut self) {
        self.is_pinned = !self.is_pinned;
        utils::set_window_topmost(self.is_pinned);
    }

    pub fn simulate_keyboard_output(&self, text: &str) {
        utils::simulate_keyboard_output(text);
    }

    pub fn handle_key_event(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Char('q') => {
                if self.in_save_mode {
                    self.in_save_mode = false;
                } else if self.in_edit_mode {
                    self.edit_buffer.push('q');
                } else if self.in_detail_page {
                    self.in_detail_page = false;
                } else if self.in_detail_view {
                    self.in_detail_view = false;
                } else {
                    // 程序将在这里结束
                }
            }
            KeyCode::Esc => {
                if self.in_edit_mode {
                    self.in_edit_mode = false;
                    self.in_save_mode = true;
                } else if self.in_save_mode {
                    self.in_save_mode = false;
                    self.in_edit_mode = true;
                } else if self.in_detail_page {
                    self.in_detail_page = false;
                } else if self.in_detail_view {
                    self.in_detail_view = false;
                } else {
                    // 程序将在这里结束
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.in_detail_page {
                    match self.current_detail_selection {
                        DetailSelection::Title => self.current_detail_selection = DetailSelection::Describe,
                        DetailSelection::Describe => self.current_detail_selection = DetailSelection::Text,
                        DetailSelection::Text => self.current_detail_selection = DetailSelection::Title,
                    }
                } else if self.in_detail_view {
                    self.next_detail();
                } else {
                    self.next();
                }
            },
            KeyCode::Up | KeyCode::Char('k') => {
                if self.in_detail_page {
                    match self.current_detail_selection {
                        DetailSelection::Title => self.current_detail_selection = DetailSelection::Text,
                        DetailSelection::Describe => self.current_detail_selection = DetailSelection::Title,
                        DetailSelection::Text => self.current_detail_selection = DetailSelection::Describe,
                    }
                } else if self.in_detail_view {
                    self.previous_detail();
                } else {
                    self.previous();
                }
            },
            KeyCode::Char('a') => {
                self.items.push(format!("新订阅项 {}", self.items.len() + 1));
            },
            KeyCode::Char('d') => {
                if !self.items.is_empty() {
                    let selected = self.state.selected().unwrap_or(0);
                    self.items.remove(selected);
                    
                    if self.items.is_empty() {
                        self.state.select(None);
                    } else {
                        let new_selected = if selected >= self.items.len() {
                            self.items.len() - 1
                        } else {
                            selected
                        };
                        self.state.select(Some(new_selected));
                    }
                }
            },
            KeyCode::Char('r') => {
                self.items.sort();
            },
            KeyCode::Char('t') => {
                self.toggle_pin();
            },
            KeyCode::Enter => {
                if self.in_edit_mode {
                    self.edit_buffer.push('\n');
                } else {
                    if !self.in_detail_view {
                        self.in_detail_view = true;
                        if !self.detail_items.is_empty() {
                            self.detail_state.select(Some(0));
                        }
                    } else if !self.in_detail_page {
                        if let Some(selected) = self.detail_state.selected() {
                            self.in_detail_page = true;
                            self.current_detail_index = selected;
                        }
                    }
                }
            }
            KeyCode::Char('v') => {
                if self.in_detail_page && !self.in_edit_mode && !self.in_save_mode {
                    if self.current_detail_index < self.detail_items.len() {
                        let current_item = &self.detail_items[self.current_detail_index];
                        self.edit_buffer = match self.current_detail_selection {
                            DetailSelection::Title => current_item.title.clone(),
                            DetailSelection::Describe => current_item.describe.clone(),
                            DetailSelection::Text => current_item.text.clone(),
                        };
                        self.in_edit_mode = true;
                    }
                }
            },
            KeyCode::Char('b') => {
                if self.in_detail_page && !self.detail_items.is_empty() {
                    if self.current_detail_index < self.detail_items.len() {
                        let text = self.detail_items[self.current_detail_index].text.clone();
                        thread::spawn(move || {
                            thread::sleep(Duration::from_secs(2));
                            // 需要一个方式来获取应用的模拟键盘输出方法
                            // 这里直接使用utils模块中的函数
                            utils::simulate_keyboard_output(&text);
                        });
                    }
                } else if self.in_detail_view && !self.detail_items.is_empty() {
                    if let Some(selected) = self.detail_state.selected() {
                        if selected < self.detail_items.len() {
                            let text = self.detail_items[selected].text.clone();
                            thread::spawn(move || {
                                thread::sleep(Duration::from_secs(2));
                                utils::simulate_keyboard_output(&text);
                            });
                        }
                    }
                }
            }
            KeyCode::Char('w') => {
                if self.in_save_mode {
                    if self.current_detail_index < self.detail_items.len() {
                        let buffer = self.edit_buffer.clone();
                        match self.current_detail_selection {
                            DetailSelection::Title => {
                                self.detail_items[self.current_detail_index].title = buffer;
                            },
                            DetailSelection::Describe => {
                                self.detail_items[self.current_detail_index].describe = buffer;
                            },
                            DetailSelection::Text => {
                                self.detail_items[self.current_detail_index].text = buffer;
                            },
                        }
                    }
                    self.in_save_mode = false;
                    self.edit_buffer.clear();
                } else if self.in_edit_mode {
                    self.edit_buffer.push('w');
                } else if self.in_detail_page {
                    if !self.detail_items.is_empty() {
                        let old_index = self.current_detail_index;
                        self.current_detail_index = (self.current_detail_index + 1) % self.detail_items.len();
                        if old_index != self.current_detail_index {
                            self.edit_buffer.clear();
                        }
                    }
                } else if self.in_detail_view {
                    self.next_detail();
                } else {
                    self.next();
                }
            },
            KeyCode::Char(c) => {
                if self.in_edit_mode {
                    // 编辑模式下，只允许特定功能键执行功能，其他字符正常输入
                    match c {
                        // 在编辑模式下，Esc键退出编辑模式
                        'q' => {
                            self.in_edit_mode = false;
                            self.in_save_mode = true;
                        },
                        // 在编辑模式下，Ctrl+S保存并退出
                        'w' => {
                            if self.current_detail_index < self.detail_items.len() {
                                let buffer = self.edit_buffer.clone();
                                match self.current_detail_selection {
                                    DetailSelection::Title => {
                                        self.detail_items[self.current_detail_index].title = buffer;
                                    },
                                    DetailSelection::Describe => {
                                        self.detail_items[self.current_detail_index].describe = buffer;
                                    },
                                    DetailSelection::Text => {
                                        self.detail_items[self.current_detail_index].text = buffer;
                                    },
                                }
                            }
                            self.in_edit_mode = false;
                            self.edit_buffer.clear();
                        },
                        // 其他字符正常输入到编辑缓冲区
                        _ => {
                            self.edit_buffer.push(c);
                        }
                    }
                } else {
                    // 非编辑模式下，所有功能键正常执行
                    match c {
                        'a' => {
                            self.items.push(format!("新订阅项 {}", self.items.len() + 1));
                        },
                        'd' => {
                            if !self.items.is_empty() {
                                let selected = self.state.selected().unwrap_or(0);
                                self.items.remove(selected);
                                
                                if self.items.is_empty() {
                                    self.state.select(None);
                                } else {
                                    let new_selected = if selected >= self.items.len() {
                                        self.items.len() - 1
                                    } else {
                                        selected
                                    };
                                    self.state.select(Some(new_selected));
                                }
                            }
                        },
                        'r' => {
                            self.items.sort();
                        },
                        't' => {
                            self.toggle_pin();
                        },
                        'v' => {
                            if self.in_detail_page && !self.in_edit_mode && !self.in_save_mode {
                                if self.current_detail_index < self.detail_items.len() {
                                    let current_item = &self.detail_items[self.current_detail_index];
                                    self.edit_buffer = match self.current_detail_selection {
                                        DetailSelection::Title => current_item.title.clone(),
                                        DetailSelection::Describe => current_item.describe.clone(),
                                        DetailSelection::Text => current_item.text.clone(),
                                    };
                                    self.in_edit_mode = true;
                                }
                            }
                        },
                        'b' => {
                            if self.in_detail_page && !self.detail_items.is_empty() {
                                if self.current_detail_index < self.detail_items.len() {
                                    let text = self.detail_items[self.current_detail_index].text.clone();
                                    thread::spawn(move || {
                                        thread::sleep(Duration::from_secs(2));
                                        utils::simulate_keyboard_output(&text);
                                    });
                                }
                            } else if self.in_detail_view && !self.detail_items.is_empty() {
                                if let Some(selected) = self.detail_state.selected() {
                                    if selected < self.detail_items.len() {
                                        let text = self.detail_items[selected].text.clone();
                                        thread::spawn(move || {
                                            thread::sleep(Duration::from_secs(2));
                                            utils::simulate_keyboard_output(&text);
                                        });
                                    }
                                }
                            }
                        },
                        'w' => {
                            if self.in_save_mode {
                                if self.current_detail_index < self.detail_items.len() {
                                    let buffer = self.edit_buffer.clone();
                                    match self.current_detail_selection {
                                        DetailSelection::Title => {
                                            self.detail_items[self.current_detail_index].title = buffer;
                                        },
                                        DetailSelection::Describe => {
                                            self.detail_items[self.current_detail_index].describe = buffer;
                                        },
                                        DetailSelection::Text => {
                                            self.detail_items[self.current_detail_index].text = buffer;
                                        },
                                    }
                                }
                                self.in_save_mode = false;
                                self.edit_buffer.clear();
                            } else if self.in_detail_page {
                                if !self.detail_items.is_empty() {
                                    let old_index = self.current_detail_index;
                                    self.current_detail_index = (self.current_detail_index + 1) % self.detail_items.len();
                                    if old_index != self.current_detail_index {
                                        self.edit_buffer.clear();
                                    }
                                }
                            } else if self.in_detail_view {
                                self.next_detail();
                            } else {
                                self.next();
                            }
                        },
                        _ => {}
                    }
                }
            },
            KeyCode::Backspace => {
                if self.in_edit_mode {
                    self.edit_buffer.pop();
                }
            },
            _ => {}
        }
    }

    pub fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> io::Result<()> {
        loop {
            terminal.draw(|f| ui::draw(f, self))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => {
                            if self.in_save_mode {
                                self.in_save_mode = false;
                            } else if self.in_edit_mode {
                                self.edit_buffer.push('q');
                            } else if self.in_detail_page {
                                self.in_detail_page = false;
                            } else if self.in_detail_view {
                                self.in_detail_view = false;
                            } else {
                                return Ok(());
                            }
                        }
                        _ => {
                            self.handle_key_event(key.code);
                        }
                    }
                }
            }
        }
    }
}