//! 进程管理模块
//! 
//! 负责查找和管理游戏进程

use anyhow::{Result, Context};

#[cfg(windows)]
use windows::{
    Win32::Foundation::CloseHandle,
    Win32::System::Threading::{
        CreateToolhelp32Snapshot, Process32FirstW, Process32NextW,
        PROCESSENTRY32W, TH32CS_SNAPPROCESS,
    },
};

/// 查找指定名称的进程
pub fn find_process(process_name: &str) -> Result<u32> {
    log::info!("正在查找进程：{}", process_name);
    
    #[cfg(windows)]
    {
        unsafe {
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, None)?;
            
            let mut entry = PROCESSENTRY32W {
                dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
                ..Default::default()
            };
            
            if Process32FirstW(snapshot, &mut entry).is_ok() {
                loop {
                    let exe_name = wide_string_to_string(&entry.szExeFile);
                    if exe_name.eq_ignore_ascii_case(process_name) {
                        let pid = entry.th32ProcessID;
                        let _ = CloseHandle(snapshot);
                        log::info!("找到进程 {}，PID: {}", process_name, pid);
                        return Ok(pid);
                    }
                    
                    if Process32NextW(snapshot, &mut entry).is_err() {
                        break;
                    }
                }
            }
            
            let _ = CloseHandle(snapshot);
        }
    }
    
    #[cfg(not(windows))]
    {
        log::warn!("非 Windows 平台，返回模拟进程 ID");
        return Ok(12345);
    }
    
    Err(anyhow::anyhow!("未找到进程：{}", process_name))
}

/// 检查进程是否存在
pub fn process_exists(pid: u32) -> bool {
    #[cfg(windows)]
    {
        use windows::Win32::System::Threading::OpenProcess;
        use windows::Win32::Foundation::CloseHandle;
        
        unsafe {
            let handle = OpenProcess(0x0010, false, pid);
            if let Ok(h) = handle {
                let _ = CloseHandle(h);
                return true;
            }
        }
    }
    
    #[cfg(not(windows))]
    {
        log::warn!("非 Windows 平台，始终返回 true");
        return true;
    }
    
    false
}

/// 获取进程列表
pub fn list_processes() -> Vec<(u32, String)> {
    let mut processes = Vec::new();
    
    #[cfg(windows)]
    {
        unsafe {
            let snapshot = match CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, None) {
                Ok(s) => s,
                Err(_) => return processes,
            };
            
            let mut entry = PROCESSENTRY32W {
                dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
                ..Default::default()
            };
            
            if Process32FirstW(snapshot, &mut entry).is_ok() {
                loop {
                    let pid = entry.th32ProcessID;
                    let name = wide_string_to_string(&entry.szExeFile);
                    processes.push((pid, name));
                    
                    if Process32NextW(snapshot, &mut entry).is_err() {
                        break;
                    }
                }
            }
            
            let _ = CloseHandle(snapshot);
        }
    }
    
    processes
}

/// 将 Windows 宽字符串转换为 Rust String
#[cfg(windows)]
fn wide_string_to_string(wide: &[u16]) -> String {
    if let Some(null_pos) = wide.iter().position(|&c| c == 0) {
        String::from_utf16_lossy(&wide[..null_pos])
    } else {
        String::from_utf16_lossy(wide)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    #[ignore] // 需要实际运行环境
    fn test_find_process() {
        let result = find_process("notepad.exe");
        assert!(result.is_ok());
    }
}
