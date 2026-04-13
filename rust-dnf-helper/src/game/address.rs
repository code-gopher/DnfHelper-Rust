//! 游戏地址常量模块
//! 
//! 定义游戏内存地址常量

/// 游戏内存地址常量
pub struct Address;

impl Address {
    // ==================== 基础地址 ====================
    
    /// 人物指针地址
    pub const PERSON_PTR_ADDR: usize = 0x00D3F8C0;
    
    /// 区域参数地址
    pub const AREA_PARAM_ADDR: usize = 0x00E4A890;
    
    /// 角色指针地址
    pub const ROLE_PTR_ADDR: usize = 0x00F1B5D8;
    
    /// 任务地址
    pub const TASK_ADDR: usize = 0x00D6E7A4;
    
    /// 疲劳值地址
    pub const PL_ADDR: usize = 0x00E5C3B0;
    
    // ==================== CALL 地址 ====================
    
    /// 读写 CALL 地址
    pub const RW_CALL_ADDR: usize = 0x00401234;
    
    /// 技能 CALL 地址
    pub const SKILL_CALL_ADDR: usize = 0x00567890;
    
    /// 透明 CALL 地址
    pub const TRANSPARENT_CALL_ADDR: usize = 0x00654321;
    
    /// 区域 CALL 地址
    pub const AREA_CALL_ADDR: usize = 0x00789ABC;
    
    /// 移动 CALL 地址
    pub const MOVE_CALL_ADDR: usize = 0x008BCDEF;
    
    /// 过图 CALL 地址
    pub const PASS_MAP_CALL_ADDR: usize = 0x009ABCDE;
    
    /// 进图 CALL 地址
    pub const ENTER_MAP_CALL_ADDR: usize = 0x00ABCDEF;
    
    /// 漂移 CALL 地址
    pub const DRIFT_CALL_ADDR: usize = 0x00BCDEF0;
    
    /// 接受任务 CALL 地址
    pub const ACCEPT_TASK_CALL_ADDR: usize = 0x00CDEF01;
    
    /// 提交任务 CALL 地址
    pub const SUBMIT_TASK_CALL_ADDR: usize = 0x00DEF012;
    
    /// 完成任​​务 CALL 地址
    pub const FINISH_TASK_CALL_ADDR: usize = 0x00EF0123;
    
    // ==================== 偏移地址 ====================
    
    /// 城镇瞬移 RDX 地址
    pub const TOWN_TELEPORT_RDX_ADDR: usize = 0x00F01234;
    
    /// 房间数据偏移
    pub const ROOM_DATA_OFFSET: usize = 0x1A8;
    
    /// 状态偏移
    pub const STAT_OFFSET: usize = 0x0C;
    
    /// 地图 ID 偏移
    pub const MAP_ID_OFFSET: usize = 0x10;
    
    /// 疲劳值偏移
    pub const PL_OFFSET: usize = 0xF8;
    
    /// 等级偏移
    pub const LEVEL_OFFSET: usize = 0x14;
    
    /// X 坐标偏移
    pub const X_OFFSET: usize = 0x4C0;
    
    /// Y 坐标偏移
    pub const Y_OFFSET: usize = 0x4C4;
    
    /// Z 坐标偏移
    pub const Z_OFFSET: usize = 0x4C8;
    
    // ==================== 汇编空白地址 ====================
    
    /// 汇编空白块地址 (用于注入代码)
    pub const ASSEMBLY_BLANK_ADDR: usize = 0x10000000;
    
    /// 技能空白地址
    pub const SKILL_BLANK_ADDR: usize = 0x10001000;
    
    /// 过图空白地址
    pub const PASS_MAP_BLANK_ADDR: usize = 0x10002000;
    
    /// Hook CALL 地址
    pub const HOOK_CALL_ADDR: usize = 0x10003000;
    
    // ==================== 其他地址 ====================
    
    /// 副本编号地址
    pub const DUNGEON_ID_ADDR: usize = 0x00FF0123;
    
    /// 时间地址
    pub const TIME_ADDR: usize = 0x00FF0456;
    
    /// 状态偏移地址
    pub const STATE_OFFSET_ADDR: usize = 0x00FF0789;
    
    /// 方向 ID 地址
    pub const DIRECTION_ID_ADDR: usize = 0x3C;
    
    /// 坐标结构偏移
    pub const COORD_STRUCT_OFFSET: usize = 0x50;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_address_constants() {
        // 验证地址不为 0
        assert!(Address::PERSON_PTR_ADDR > 0);
        assert!(Address::RW_CALL_ADDR > 0);
        assert!(Address::ASSEMBLY_BLANK_ADDR > 0);
    }
}
