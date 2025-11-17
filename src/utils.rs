
/// 设置窗口是否置顶
#[cfg(windows)]
pub fn set_window_topmost(topmost: bool) {
    use winapi::um::winuser::{SetWindowPos, HWND_TOPMOST, HWND_NOTOPMOST, SWP_NOMOVE, SWP_NOSIZE, GetForegroundWindow};
    
    unsafe {
        let window = GetForegroundWindow();
        if !window.is_null() {
            let hwnd_topmost = if topmost { HWND_TOPMOST } else { HWND_NOTOPMOST };
            SetWindowPos(
                window,
                hwnd_topmost,
                0, 0, 0, 0,
                SWP_NOMOVE | SWP_NOSIZE
            );
        }
    }
}

/// 设置窗口是否置顶 (非Windows平台的空实现)
#[cfg(not(windows))]
pub fn set_window_topmost(_topmost: bool) {
    // 非Windows平台不支持此功能
}

/// 模拟键盘流输出文本 (Windows平台)
#[cfg(windows)]
pub fn simulate_keyboard_output(text: &str) {
    use winapi::um::winuser::{keybd_event, VkKeyScanW, KEYEVENTF_KEYUP};
    use std::thread;
    use std::time::Duration;
    
    unsafe {
        // 我们不需要检查控制台窗口，而是直接向当前焦点窗口发送键盘事件
        for c in text.chars() {
            // 将Unicode字符转换为虚拟键码
            let vk = VkKeyScanW(c as u16);
            if vk != -1 {
                let vk_code = (vk & 0xFF) as u8;
                let shift = (vk >> 8) & 0x01 != 0;
                
                // 如果需要按下Shift键
                if shift {
                    keybd_event(0x10, 0, 0, 0); // 按下Shift
                }
                
                // 按下并释放字符键
                keybd_event(vk_code, 0, 0, 0); // 按下键
                keybd_event(vk_code, 0, KEYEVENTF_KEYUP, 0); // 释放键
                
                // 如果按下了Shift键，释放它
                if shift {
                    keybd_event(0x10, 0, KEYEVENTF_KEYUP, 0); // 释放Shift
                }
                
                // 添加小延迟以确保输入被正确处理
                thread::sleep(Duration::from_millis(10));
            }
        }
    }
}

/// 模拟键盘流输出文本 (非Windows平台)
#[cfg(not(windows))]
pub fn simulate_keyboard_output(text: &str) {
    // 在非Windows平台上简单地打印文本
    println!("{}", text);
}