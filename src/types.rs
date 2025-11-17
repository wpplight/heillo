// 重新导出模型类型（延迟导入，在 main.rs 中 models 模块已声明）
#[allow(unused_imports)]
pub use crate::models::{Group, Item};

/// 表示详情项目的结构体
#[derive(Clone)]
pub struct DetailItem {
    pub title: String,
    pub describe: String,
    pub text: String,
    /// Item ID，用于 API 更新
    pub id: String,
    /// Group ID，用于 API 更新
    pub group_id: String,
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

/// 组创建模式枚举
#[derive(Clone, Copy, PartialEq)]
pub enum GroupCreationMode {
    Local,         // 本地模式
    Remote,         // 远程模式
    InputHost,      // 输入主机地址
    InputPort,      // 输入端口
    InputSecretKey, // 输入密钥
}