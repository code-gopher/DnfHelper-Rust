//! 热键监听模块
//! 
//! 实现全局热键注册和监听功能

use anyhow::Result;
use log::{info, debug, error};

#[cfg(windows)]
use windows::{
    Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM},
    Win32::System::LibraryLoader::GetModuleHandleW,
    Win32::UI::Input::KeyboardAndMouse::{
        RegisterHotKey, UnregisterHotKey, GetAsyncKeyState,
        VK_F1, VK_F2, VK_F3, VK_F4, VK_F5, VK_F6, VK_F7, VK_F8, VK_F9, VK_F10, VK_F11, VK_F12,
        MOD_ALT, MOD_CONTROL, MOD_SHIFT, MOD_WIN,
    },
    Win32::UI::WindowsAndMessaging::{
        GetMessageW, TranslateMessage, DispatchMessageW, MSG,
        WM_HOTKEY, CreateWindowExW, DefWindowProcW,
        CW_USEDEFAULT, WS_OVERLAPPEDWINDOW,
    },
};

/// 热键动作枚举
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HotKeyAction {
    /// 启动/停止自动刷图
    ToggleAuto,
    /// 紧急停止
    EmergencyStop,
    /// 切换角色
    SwitchRole,
    /// 回城
    ReturnTown,
    /// 自定义动作 1
    Custom1,
    /// 自定义动作 2
    Custom2,
    /// 未知动作
    Unknown,
}

/// 热键配置结构体
#[derive(Debug, Clone)]
pub struct HotKeyConfig {
    /// 虚拟键码
    pub vk_code: i32,
    /// 修饰键 (Ctrl, Alt, Shift, Win)
    pub modifiers: u32,
    /// 关联动作
    pub action: HotKeyAction,
}

impl HotKeyConfig {
    pub fn new(vk_code: i32, modifiers: u32, action: HotKeyAction) -> Self {
        Self { vk_code, modifiers, action }
    }
}

/// 热键管理器
pub struct HotKeyManager {
    /// 已注册的热键列表
    hotkeys: Vec<HotKeyConfig>,
    /// 是否正在运行
    running: bool,
    /// 窗口句柄 (Windows)
    #[cfg(windows)]
    hwnd: Option<HWND>,
}

impl HotKeyManager {
    /// 创建新的热键管理器
    pub fn new() -> Self {
        Self {
            hotkeys: Vec::new(),
            running: false,
            #[cfg(windows)]
            hwnd: None,
        }
    }
    
    /// 注册热键
    pub fn register(&mut self, config: HotKeyConfig) -> Result<()> {
        info!("注册热键：VK={}, Modifiers={:#X}, Action={:?}", 
              config.vk_code, config.modifiers, config.action);
        
        #[cfg(windows)]
        {
            unsafe {
                // 确保有窗口句柄
                if self.hwnd.is_none() {
                    self.create_hidden_window()?;
                }
                
                let hotkey_id = self.hotkeys.len() as u32 + 1;
                let result = RegisterHotKey(
                    self.hwnd.unwrap(),
                    hotkey_id,
                    config.modifiers,
                    config.vk_code as u32,
                );
                
                if result.0 == 0 {
                    return Err(anyhow::anyhow!("注册热键失败"));
                }
            }
        }
        
        #[cfg(not(windows))]
        {
            log::warn!("非 Windows 平台，热键注册被跳过");
        }
        
        self.hotkeys.push(config);
        Ok(())
    }
    
    /// 注销所有热键
    pub fn unregister_all(&mut self) -> Result<()> {
        info!("注销所有热键");
        
        #[cfg(windows)]
        {
            for (i, _) in self.hotkeys.iter().enumerate() {
                unsafe {
                    let hotkey_id = (i + 1) as u32;
                    UnregisterHotKey(self.hwnd.unwrap(), hotkey_id);
                }
            }
        }
        
        self.hotkeys.clear();
        Ok(())
    }
    
    /// 创建隐藏窗口用于接收热键消息
    #[cfg(windows)]
    fn create_hidden_window(&mut self) -> Result<()> {
        unsafe {
            let hinstance = GetModuleHandleW(None)?;
            
            let hwnd = CreateWindowExW(
                Default::default(),
                widestring("DNFHelperHotKey"),
                widestring("DNF Helper HotKey Window"),
                WS_OVERLAPPEDWINDOW,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                None,
                None,
                hinstance,
                None,
            );
            
            if hwnd.0 == 0 {
                return Err(anyhow::anyhow!("创建窗口失败"));
            }
            
            self.hwnd = Some(hwnd);
            info!("创建隐藏窗口：{:?}", hwnd);
        }
        
        Ok(())
    }
    
    /// 启动热键监听循环
    pub fn start_listening(&mut self, callback: Box<dyn Fn(HotKeyAction) + Send + 'static>) -> Result<()> {
        info!("启动热键监听...");
        self.running = true;
        
        #[cfg(windows)]
        {
            unsafe {
                let mut msg: MSG = std::mem::zeroed();
                
                while self.running {
                    if GetMessageW(&mut msg, None, 0, 0).0 > 0 {
                        if msg.message == WM_HOTKEY {
                            let hotkey_id = msg.wParam.0 as usize;
                            if hotkey_id > 0 && hotkey_id <= self.hotkeys.len() {
                                let action = self.hotkeys[hotkey_id - 1].action;
                                debug!("热键触发：{:?}", action);
                                callback(action);
                            }
                        } else {
                            TranslateMessage(&msg);
                            DispatchMessageW(&msg);
                        }
                    }
                }
            }
        }
        
        #[cfg(not(windows))]
        {
            log::warn!("非 Windows 平台，使用轮询模式模拟热键");
            // 简单的轮询实现
            while self.running {
                std::thread::sleep(std::time::Duration::from_millis(100));
                // TODO: 实现跨平台键盘监听
            }
        }
        
        Ok(())
    }
    
    /// 停止监听
    pub fn stop(&mut self) {
        info!("停止热键监听");
        self.running = false;
    }
    
    /// 检查按键状态
    pub fn is_key_pressed(&self, vk_code: i32) -> bool {
        #[cfg(windows)]
        {
            unsafe {
                let state = GetAsyncKeyState(vk_code);
                (state & 0x8000) != 0
            }
        }
        
        #[cfg(not(windows))]
        {
            false
        }
    }
}

impl Default for HotKeyManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(windows)]
impl Drop for HotKeyManager {
    fn drop(&mut self) {
        let _ = self.unregister_all();
    }
}

/// Windows 宽字符串辅助函数
#[cfg(windows)]
fn widestring(s: &str) -> Vec<u16> {
    use std::os::windows::prelude::OsStrExt;
    use std::ffi::OsStr;
    OsStr::new(s).encode_wide().chain(Some(0)).collect()
}

/// 默认热键配置
pub fn default_hotkeys() -> Vec<HotKeyConfig> {
    vec![
        // F1: 启动/停止自动刷图
        HotKeyConfig::new(VK_F1.0 as i32, MOD_CONTROL, HotKeyAction::ToggleAuto),
        // F2: 紧急停止
        HotKeyConfig::new(VK_F2.0 as i32, MOD_CONTROL, HotKeyAction::EmergencyStop),
        // F3: 切换角色
        HotKeyConfig::new(VK_F3.0 as i32, MOD_CONTROL, HotKeyAction::SwitchRole),
        // F4: 回城
        HotKeyConfig::new(VK_F4.0 as i32, MOD_CONTROL, HotKeyAction::ReturnTown),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hotkey_config() {
        let config = HotKeyConfig::new(VK_F1.0 as i32, MOD_CONTROL, HotKeyAction::ToggleAuto);
        assert_eq!(config.vk_code, VK_F1.0 as i32);
        assert_eq!(config.modifiers, MOD_CONTROL);
        assert_eq!(config.action, HotKeyAction::ToggleAuto);
    }
    
    #[test]
    fn test_default_hotkeys() {
        let hotkeys = default_hotkeys();
        assert!(!hotkeys.is_empty());
        assert_eq!(hotkeys.len(), 4);
    }
}
