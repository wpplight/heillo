use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::io;
use std::thread;
use std::time::Duration;
use crate::types::{DetailItem, DetailSelection, Group};
use crate::ui;
use crate::utils;
use crate::api::ApiClient;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::widgets::ListState;
use anyhow::Result;

pub struct App {
    pub groups: Vec<Group>,
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
    pub api_client: Option<ApiClient>,  // 改为可选，支持本地模式
    pub current_group_id: Option<String>,
    pub error_message: Option<String>,
    pub in_group_creation: bool,
    pub group_creation_mode: GroupCreationMode,
    pub group_creation_host: String,
    pub group_creation_port: String,
    pub group_creation_secret_key: String,
}

impl App {
    pub fn new(api_client: Option<ApiClient>) -> Result<Self> {
        let mut state = ListState::default();
        if false {  // 占位，避免警告
            state.select(Some(0));
        }
        
        let mut detail_state = ListState::default();
        if false {  // 占位，避免警告
            detail_state.select(Some(0));
        }
        
        let mut app = Self {
            groups: Vec::new(),
            state,
            is_pinned: false,
            detail_items: Vec::new(),
            in_detail_view: false,
            in_detail_page: false,
            detail_state,
            current_detail_index: 0,
            current_detail_selection: DetailSelection::Title,
            in_edit_mode: false,
            in_save_mode: false,
            edit_buffer: String::new(),
            api_client,
            current_group_id: None,
            error_message: None,
            in_group_creation: false,
            group_creation_mode: GroupCreationMode::Local,
            group_creation_host: String::new(),
            group_creation_port: String::new(),
            group_creation_secret_key: String::new(),
        };

        // 只有在有API客户端时才加载远程groups
        if app.api_client.is_some() {
            app.load_groups()?;
        }
        
        Ok(app)
    }

    /// 加载所有 groups（本地或远程）
    pub fn load_groups(&mut self) -> Result<()> {
        match &self.api_client {
            Some(api_client) => {
                // 远程模式：从API加载
                match api_client.list_groups() {
                    Ok(groups) => {
                        self.groups = groups;
                        self.error_message = None;
                        // 更新选中状态
                        if !self.groups.is_empty() {
                            self.state.select(Some(0));
                        } else {
                            self.state.select(None);
                        }
                        Ok(())
                    }
                    Err(e) => {
                        self.error_message = Some(format!("加载组失败: {}", e));
                        Err(e)
                    }
                }
            }
            None => {
                // 本地模式：清空groups列表
                self.groups.clear();
                self.error_message = None;
                self.state.select(None);
                Ok(())
            }
        }
    }

    /// 加载当前组的 items（本地或远程）
    pub fn load_items(&mut self) -> Result<()> {
        let group_id = match &self.current_group_id {
            Some(id) => id.clone(),
            None => {
                self.error_message = Some("未选择组".to_string());
                return Ok(());
            }
        };

        match &self.api_client {
            Some(api_client) => {
                // 远程模式：从API加载
                match api_client.list_items(&group_id) {
                    Ok(items) => {
                        // 解密 items 并转换为 DetailItem
                        self.detail_items = items
                            .into_iter()
                            .map(|item| {
                                let title = api_client
                                    .decrypt_item_field(&item.title)
                                    .unwrap_or_else(|_| "解密失败".to_string());
                                let describe = api_client
                                    .decrypt_item_field(&item.describe)
                                    .unwrap_or_else(|_| "解密失败".to_string());
                                let text = api_client
                                    .decrypt_item_field(&item.text)
                                    .unwrap_or_else(|_| "解密失败".to_string());

                                DetailItem {
                                    title,
                                    describe,
                                    text,
                                    id: item.id,
                                    group_id: group_id.clone(),
                                }
                            })
                            .collect();

                        self.error_message = None;
                        // 更新选中状态
                        if !self.detail_items.is_empty() {
                            self.detail_state.select(Some(0));
                        } else {
                            self.detail_state.select(None);
                        }
                        Ok(())
                    }
                    Err(e) => {
                        self.error_message = Some(format!("加载 items 失败: {}", e));
                        Err(e)
                    }
                }
            }
            None => {
                // 本地模式：清空items列表
                self.detail_items.clear();
                self.error_message = None;
                self.detail_state.select(None);
                Ok(())
            }
        }
    }

    /// 刷新当前视图的数据
    pub fn refresh(&mut self) {
        if self.in_detail_view {
            let _ = self.load_items();
        } else {
            let _ = self.load_groups();
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.groups.len().saturating_sub(1) {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        if !self.groups.is_empty() {
            self.state.select(Some(i));
        }
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.groups.len().saturating_sub(1)
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        if !self.groups.is_empty() {
            self.state.select(Some(i));
        }
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

    /// 获取当前选中组的解密名称
    pub fn get_current_group_name(&self) -> String {
        if let Some(selected) = self.state.selected() {
            if selected < self.groups.len() {
                let group = &self.groups[selected];
                return match &self.api_client {
                    Some(api_client) => {
                        api_client
                            .decrypt_group_name(&group.name)
                            .unwrap_or_else(|_| "解密失败".to_string())
                    }
                    None => {
                        // 本地模式：直接显示组名称
                        group.name.clone()
                    }
                };
            }
        }
        "".to_string()
    }

    /// 开始创建新组流程
    fn create_group(&mut self) {
        self.in_group_creation = true;
        self.group_creation_mode = GroupCreationMode::Local;
        self.group_creation_host.clear();
        self.group_creation_port.clear();
        self.group_creation_secret_key.clear();
        self.error_message = None;
    }
    
    /// 完成组创建
    fn finish_group_creation(&mut self) {
        // 简单实现：使用时间戳作为 ID
        use std::time::{SystemTime, UNIX_EPOCH};
        let id = format!("group_{}", 
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs());
        
        let name = "新订阅组";
        // 默认不使用加密，后续可以添加用户选择界面
        let use_encryption = false;
        
        match self.group_creation_mode {
            GroupCreationMode::Local => {
                // 本地模式：直接在本地创建组
                let new_group = Group {
                    id: id.clone(),
                    name: name.to_string(),
                };
                self.groups.push(new_group);
                
                // 更新选中状态
                if !self.groups.is_empty() {
                    self.state.select(Some(self.groups.len() - 1));
                }
                self.error_message = None;
            }
            GroupCreationMode::Remote => {
                // 远程模式：通过API创建组
                if let Some(api_client) = &self.api_client {
                    match api_client.create_group(&id, name, use_encryption) {
                        Ok(_) => {
                            let _ = self.load_groups();
                            self.error_message = None;
                        }
                        Err(e) => {
                            self.error_message = Some(format!("创建组失败: {}", e));
                        }
                    }
                } else {
                    // 如果没有API客户端，创建本地组
                    let new_group = Group {
                        id: id.clone(),
                        name: name.to_string(),
                    };
                    self.groups.push(new_group);
                    
                    // 更新选中状态
                    if !self.groups.is_empty() {
                        self.state.select(Some(self.groups.len() - 1));
                    }
                    self.error_message = None;
                }
            }
            _ => {
                // 其他模式不应该到达这里
                self.error_message = Some("组创建模式错误".to_string());
            }
        }
        
        // 重置组创建状态
        self.in_group_creation = false;
        self.group_creation_mode = GroupCreationMode::Local;
        self.group_creation_host.clear();
        self.group_creation_port.clear();
        self.group_creation_secret_key.clear();
    }

    /// 删除当前组
    fn delete_group(&mut self) {
        if let Some(selected) = self.state.selected() {
            if selected < self.groups.len() {
                let group_id = self.groups[selected].id.clone();
                
                match &self.api_client {
                    Some(api_client) => {
                        // 远程模式：通过API删除组
                        match api_client.delete_group(&group_id) {
                            Ok(_) => {
                                let _ = self.load_groups();
                                self.error_message = None;
                                // 如果删除的是当前查看的组，返回主视图
                                if self.current_group_id.as_ref() == Some(&group_id) {
                                    self.in_detail_view = false;
                                    self.in_detail_page = false;
                                    self.current_group_id = None;
                                }
                            }
                            Err(e) => {
                                self.error_message = Some(format!("删除组失败: {}", e));
                            }
                        }
                    }
                    None => {
                        // 本地模式：直接在本地删除组
                        self.groups.remove(selected);
                        
                        // 更新选中状态
                        if self.groups.is_empty() {
                            self.state.select(None);
                        } else if selected >= self.groups.len() {
                            self.state.select(Some(self.groups.len() - 1));
                        }
                        
                        // 如果删除的是当前查看的组，返回主视图
                        if self.current_group_id.as_ref() == Some(&group_id) {
                            self.in_detail_view = false;
                            self.in_detail_page = false;
                            self.current_group_id = None;
                        }
                        self.error_message = None;
                    }
                }
            }
        }
    }

    /// 创建新 item
    fn create_item(&mut self) {
        let group_id = match &self.current_group_id {
            Some(id) => id.clone(),
            None => {
                self.error_message = Some("未选择组".to_string());
                return;
            }
        };

        use std::time::{SystemTime, UNIX_EPOCH};
        let id = format!("item_{}", 
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs());
        
        // 默认不使用加密，后续可以添加用户选择界面
        let use_encryption = false;
        
        match &self.api_client {
            Some(api_client) => {
                // 远程模式：通过API创建item
                match api_client.create_item(&group_id, &id, "新标题", "新描述", "新文本", use_encryption) {
                    Ok(_) => {
                        let _ = self.load_items();
                        self.error_message = None;
                    }
                    Err(e) => {
                        self.error_message = Some(format!("创建 item 失败: {}", e));
                    }
                }
            }
            None => {
                // 本地模式：直接在本地创建item
                let new_item = DetailItem {
                    id: id.clone(),
                    group_id: group_id.clone(),
                    title: "新标题".to_string(),
                    describe: "新描述".to_string(),
                    text: "新文本".to_string(),
                };
                
                self.detail_items.push(new_item);
                
                // 更新选中状态
                if !self.detail_items.is_empty() {
                    self.detail_state.select(Some(self.detail_items.len() - 1));
                }
                self.error_message = None;
            }
        }
    }

    /// 删除当前 item
    fn delete_item(&mut self) {
        if let Some(selected) = self.detail_state.selected() {
            if selected < self.detail_items.len() {
                let item = &self.detail_items[selected];
                let group_id = item.group_id.clone();
                let item_id = item.id.clone();
                
                match &self.api_client {
                    Some(api_client) => {
                        // 远程模式：通过API删除item
                        match api_client.delete_item(&group_id, &item_id) {
                            Ok(_) => {
                                let _ = self.load_items();
                                self.error_message = None;
                            }
                            Err(e) => {
                                self.error_message = Some(format!("删除 item 失败: {}", e));
                            }
                        }
                    }
                    None => {
                        // 本地模式：直接在本地删除item
                        self.detail_items.remove(selected);
                        
                        // 更新选中状态
                        if self.detail_items.is_empty() {
                            self.detail_state.select(None);
                        } else if selected >= self.detail_items.len() {
                            self.detail_state.select(Some(self.detail_items.len() - 1));
                        }
                        self.error_message = None;
                    }
                }
            }
        }
    }

    /// 保存编辑的 item
    fn save_item_edit(&mut self) {
        if self.current_detail_index >= self.detail_items.len() {
            return;
        }

        let item = &self.detail_items[self.current_detail_index];
        let group_id = item.group_id.clone();
        let item_id = item.id.clone();

        // 根据当前选中的字段更新
        let (title, describe, text) = match self.current_detail_selection {
            DetailSelection::Title => {
                (self.edit_buffer.clone(), item.describe.clone(), item.text.clone())
            }
            DetailSelection::Describe => {
                (item.title.clone(), self.edit_buffer.clone(), item.text.clone())
            }
            DetailSelection::Text => {
                (item.title.clone(), item.describe.clone(), self.edit_buffer.clone())
            }
        };

        // 默认不使用加密，后续可以添加用户选择界面
        let use_encryption = false;
        
        match &self.api_client {
            Some(api_client) => {
                // 远程模式：通过API更新item
                match api_client.update_item(&group_id, &item_id, &title, &describe, &text, use_encryption) {
                    Ok(_) => {
                        // 更新本地数据
                        self.detail_items[self.current_detail_index].title = title;
                        self.detail_items[self.current_detail_index].describe = describe;
                        self.detail_items[self.current_detail_index].text = text;
                        self.error_message = None;
                    }
                    Err(e) => {
                        self.error_message = Some(format!("更新 item 失败: {}", e));
                    }
                }
            }
            None => {
                // 本地模式：直接在本地更新item
                self.detail_items[self.current_detail_index].title = title;
                self.detail_items[self.current_detail_index].describe = describe;
                self.detail_items[self.current_detail_index].text = text;
                self.error_message = None;
            }
        }
    }

    pub fn handle_key_event(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Char('q') => {
                if self.in_save_mode {
                    self.in_save_mode = false;
                    self.in_edit_mode = false;
                    self.edit_buffer.clear();
                } else if self.in_edit_mode {
                    self.edit_buffer.push('q');
                } else if self.in_detail_page {
                    self.in_detail_page = false;
                } else if self.in_detail_view {
                    self.in_detail_view = false;
                    self.in_detail_page = false;
                    self.current_group_id = None;
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
                    self.in_detail_page = false;
                    self.current_group_id = None;
                } else {
                    // 程序将在这里结束
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.in_edit_mode {
                    self.edit_buffer.push('j');
                } else if self.in_detail_page {
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
                if self.in_edit_mode {
                    self.edit_buffer.push('k');
                } else if self.in_detail_page {
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
                if self.in_edit_mode {
                    self.edit_buffer.push('a');
                } else if self.in_group_creation {
                    // 在组创建流程中，处理模式选择
                    match self.group_creation_mode {
                        GroupCreationMode::Local => {
                            // 选择本地模式，直接完成创建
                            self.finish_group_creation();
                        }
                        GroupCreationMode::Remote => {
                            // 选择远程模式，开始输入主机地址
                            self.group_creation_mode = GroupCreationMode::InputHost;
                        }
                        GroupCreationMode::InputHost => {
                            // 完成主机地址输入，开始输入端口
                            self.group_creation_mode = GroupCreationMode::InputPort;
                        }
                        GroupCreationMode::InputPort => {
                            // 完成端口输入，开始输入密钥
                            self.group_creation_mode = GroupCreationMode::InputSecretKey;
                        }
                        GroupCreationMode::InputSecretKey => {
                            // 完成所有输入，创建组
                            self.finish_group_creation();
                        }
                    }
                } else if self.in_detail_view {
                    self.create_item();
                } else {
                    self.create_group();
                }
            },
            KeyCode::Char('d') => {
                if self.in_edit_mode {
                    self.edit_buffer.push('d');
                } else if self.in_detail_page && !self.in_save_mode {
                    // 在详情页面中，清空当前选中区块的内容
                    if self.current_detail_index < self.detail_items.len() {
                        match self.current_detail_selection {
                            DetailSelection::Title => {
                                self.edit_buffer.clear();
                            },
                            DetailSelection::Describe => {
                                self.edit_buffer.clear();
                            },
                            DetailSelection::Text => {
                                self.edit_buffer.clear();
                            },
                        }
                    }
                } else if self.in_detail_view {
                    self.delete_item();
                } else {
                    self.delete_group();
                }
            },
            KeyCode::Char('r') => {
                if self.in_edit_mode {
                    self.edit_buffer.push('r');
                } else {
                    self.refresh();
                }
            },
            KeyCode::Char('t') => {
                if self.in_edit_mode {
                    self.edit_buffer.push('t');
                } else {
                    self.toggle_pin();
                }
            },
            KeyCode::Enter => {
                if self.in_edit_mode {
                    self.edit_buffer.push('\n');
                } else {
                    if !self.in_detail_view {
                        // 进入选中组的 items 列表
                        if let Some(selected) = self.state.selected() {
                            if selected < self.groups.len() {
                                self.current_group_id = Some(self.groups[selected].id.clone());
                                let _ = self.load_items();
                                self.in_detail_view = true;
                                if !self.detail_items.is_empty() {
                                    self.detail_state.select(Some(0));
                                }
                            }
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
                if self.in_edit_mode {
                    self.edit_buffer.push('v');
                } else if self.in_detail_page && !self.in_edit_mode && !self.in_save_mode {
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
                if self.in_edit_mode {
                    self.edit_buffer.push('b');
                } else if self.in_detail_page && !self.detail_items.is_empty() {
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
            }
            KeyCode::Char('w') => {
                if self.in_save_mode {
                    self.save_item_edit();
                    self.in_save_mode = false;
                    self.in_edit_mode = false;
                    self.edit_buffer.clear();
                } else if self.in_edit_mode {
                    // 在编辑模式下，w 键直接保存并退出
                    self.save_item_edit();
                    self.in_edit_mode = false;
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
            KeyCode::Char(c) => {
                if self.in_group_creation {
                    // 在组创建流程中，处理文本输入
                    match self.group_creation_mode {
                        GroupCreationMode::InputHost => {
                            self.group_creation_host.push(c);
                        }
                        GroupCreationMode::InputPort => {
                            self.group_creation_port.push(c);
                        }
                        GroupCreationMode::InputSecretKey => {
                            self.group_creation_secret_key.push(c);
                        }
                        _ => {
                            // 在其他模式下，处理模式切换
                            match c {
                                'l' => {
                                    // 切换到本地模式
                                    self.group_creation_mode = GroupCreationMode::Local;
                                }
                                'r' => {
                                    // 切换到远程模式
                                    self.group_creation_mode = GroupCreationMode::Remote;
                                }
                                _ => {}
                            }
                        }
                    }
                } else if self.in_edit_mode {
                    self.edit_buffer.push(c);
                }
            },
            KeyCode::Backspace => {
                if self.in_group_creation {
                    // 在组创建流程中，处理退格删除
                    match self.group_creation_mode {
                        GroupCreationMode::InputHost => {
                            self.group_creation_host.pop();
                        }
                        GroupCreationMode::InputPort => {
                            self.group_creation_port.pop();
                        }
                        GroupCreationMode::InputSecretKey => {
                            self.group_creation_secret_key.pop();
                        }
                        _ => {}
                    }
                } else if self.in_edit_mode {
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
                if self.in_group_creation {
                    // 取消组创建流程
                    self.in_group_creation = false;
                    self.group_creation_mode = GroupCreationMode::Local;
                    self.group_creation_host.clear();
                    self.group_creation_port.clear();
                    self.group_creation_secret_key.clear();
                } else if self.in_save_mode {
                    self.in_save_mode = false;
                    self.in_edit_mode = false;
                    self.edit_buffer.clear();
                } else if self.in_edit_mode {
                    self.edit_buffer.push('q');
                } else if self.in_detail_page {
                    self.in_detail_page = false;
                } else if self.in_detail_view {
                    self.in_detail_view = false;
                    self.in_detail_page = false;
                    self.current_group_id = None;
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
