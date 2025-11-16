use ratatui::widgets::ListState;

/// 表示详情项目的结构体
#[derive(Clone)]
pub struct DetailItem {
    pub title: String,
    pub describe: String,
    pub text: String,
}

/// 详情页面中的可选项枚举
#[derive(Clone, Copy, PartialEq)]
pub enum DetailSelection {
    Title,
    Describe,
    Text,
}

/// 键盘输入事件类型
#[derive(Clone, Copy, PartialEq)]
pub enum InputEvent {
    Quit,
    Help,
    TogglePinned,
    AddItem,
    DeleteItem,
    Refresh,
    Enter,
    Escape,
    Up,
    Down,
    Left,
    Right,
    EditMode,
    SaveSelection,
    CharacterInput(char),
    Backspace,
    OtherKey,
}

/// 应用程序状态枚举
#[derive(Clone, Copy, PartialEq)]
pub enum AppState {
    MainView,      // 主视图
    DetailView,    // 详细视图
    DetailPage,    // 详情页面
    EditMode,      // 编辑模式
    SaveMode,      // 保存选择模式
}