//! 时间工具模块
//! 
//! 提供睡眠和计时功能

use std::time::{Duration, Instant};

/// 线程睡眠指定毫秒数
pub fn sleep(ms: u64) {
    std::thread::sleep(Duration::from_millis(ms));
}

/// 高精度睡眠 (微秒级)
#[cfg(windows)]
pub fn sleep_precise(us: u64) {
    use windows::Win32::System::Threading::Sleep;
    unsafe {
        Sleep((us / 1000) as u32);
    }
}

#[cfg(not(windows))]
pub fn sleep_precise(us: u64) {
    std::thread::sleep(Duration::from_micros(us));
}

/// 计时器结构体
pub struct Timer {
    start: Instant,
}

impl Timer {
    /// 创建并启动计时器
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }
    
    /// 重置计时器
    pub fn reset(&mut self) {
        self.start = Instant::now();
    }
    
    /// 获取经过的毫秒数
    pub fn elapsed_ms(&self) -> u64 {
        self.start.elapsed().as_millis() as u64
    }
    
    /// 获取经过的微秒数
    pub fn elapsed_us(&self) -> u64 {
        self.start.elapsed().as_micros() as u64
    }
    
    /// 检查是否超过指定毫秒数
    pub fn has_elapsed(&self, ms: u64) -> bool {
        self.elapsed_ms() >= ms
    }
    
    /// 等待直到超过指定时间
    pub fn wait_until(&self, ms: u64) {
        let elapsed = self.elapsed_ms();
        if elapsed < ms {
            sleep(ms - elapsed);
        }
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

/// 延迟执行宏
#[macro_export]
macro_rules! delay {
    ($ms:expr) => {
        $crate::helper::timer::sleep($ms)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_timer() {
        let mut timer = Timer::new();
        sleep(10);
        assert!(timer.elapsed_ms() >= 10);
        
        timer.reset();
        assert!(timer.elapsed_ms() < 10);
    }
}
