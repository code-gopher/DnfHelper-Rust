//! 全局数据模块
//! 
//! 存储全局状态和共享数据

use parking_lot::RwLock;
use std::sync::Arc;

/// 全局数据结构体
#[derive(Debug, Clone)]
pub struct GlobalData {
    /// 自动刷图开关
    pub auto_switch: bool,
    /// 当前地图 ID
    pub map_id: i32,
    /// 当前地图难度
    pub map_level: i32,
    /// 当前角色数量
    pub role_count: i32,
    /// 是否首次进图
    pub first_enter_map: bool,
    /// 完成次数
    pub completed_count: i32,
}

impl Default for GlobalData {
    fn default() -> Self {
        Self {
            auto_switch: false,
            map_id: 0,
            map_level: 0,
            role_count: 0,
            first_enter_map: false,
            completed_count: 0,
        }
    }
}

impl GlobalData {
    /// 创建新的全局数据实例
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 重置状态
    pub fn reset(&mut self) {
        self.auto_switch = false;
        self.first_enter_map = false;
        self.completed_count = 0;
    }
}

/// 线程安全的全局数据包装器
#[derive(Clone)]
pub struct GlobalState {
    inner: Arc<RwLock<GlobalData>>,
}

impl GlobalState {
    /// 创建新的全局状态
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(GlobalData::new())),
        }
    }
    
    /// 获取读锁
    pub fn read(&self) -> parking_lot::RwLockReadGuard<GlobalData> {
        self.inner.read()
    }
    
    /// 获取写锁
    pub fn write(&self) -> parking_lot::RwLockWriteGuard<GlobalData> {
        self.inner.write()
    }
}

impl Default for GlobalState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_global_data() {
        let mut data = GlobalData::new();
        assert!(!data.auto_switch);
        
        data.auto_switch = true;
        assert!(data.auto_switch);
        
        data.reset();
        assert!(!data.auto_switch);
    }
    
    #[test]
    fn test_thread_safety() {
        let state = GlobalState::new();
        
        {
            let mut data = state.write();
            data.auto_switch = true;
        }
        
        {
            let data = state.read();
            assert!(data.auto_switch);
        }
    }
}
