//! 游戏核心模块
//! 
//! 实现游戏主逻辑和自动刷图功能

use anyhow::Result;
use crate::config::Config;
use crate::driver::{Driver, memory::ReadWriteMemory};
use crate::entity::{GlobalData, GlobalState, MapDataType};
use crate::helper::timer;
use crate::game::call_system::CallSystem;
use log::{info, debug, error, warn};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

/// 游戏状态枚举
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    /// 未初始化
    Uninitialized,
    /// 角色选择
    RoleSelect,
    /// 城镇中
    InTown,
    /// 选图中
    SelectingMap,
    /// 副本中
    InDungeon,
    /// 战斗状态
    Fighting,
    /// 捡取物品
    Picking,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Uninitialized
    }
}

/// 游戏主结构体
pub struct Game {
    driver: Driver,
    config: Config,
    global_state: GlobalState,
    game_state: GameState,
    call_system: Option<CallSystem<crate::driver::memory::Memory>>,
    running: AtomicBool,
}

impl Game {
    /// 创建新的游戏实例
    pub fn new(driver: Driver, config: Config) -> Self {
        Self {
            driver,
            config,
            global_state: GlobalState::new(),
            game_state: GameState::default(),
            call_system: None,
            running: AtomicBool::new(false),
        }
    }
    
    /// 初始化游戏
    pub fn initialize(&mut self) -> Result<()> {
        info!("正在初始化游戏模块...");
        
        // 验证驱动已初始化
        if !self.driver.is_installed() {
            error!("驱动未安装");
            return Err(anyhow::anyhow!("驱动未安装"));
        }
        
        // 初始化 CALL 系统
        let memory = self.driver.memory().clone();
        let mut call_system = CallSystem::new(memory);
        
        // 获取进程句柄
        #[cfg(windows)]
        {
            use windows::Win32::System::Threading::OpenProcess;
            use windows::Win32::System::Threading::{PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE};
            
            let pid = self.driver.get_process_id();
            unsafe {
                let handle = OpenProcess(
                    PROCESS_VM_OPERATION | PROCESS_VM_READ | PROCESS_VM_WRITE,
                    false,
                    pid,
                )?;
                call_system.set_process_handle(handle.0 as usize);
            }
        }
        
        call_system.initialize()?;
        self.call_system = Some(call_system);
        
        // 读取初始内存测试
        let person_ptr = self.read_person_ptr();
        debug!("人物指针：0x{:X}", person_ptr);
        
        self.game_state = GameState::RoleSelect;
        info!("游戏初始化完成");
        
        Ok(())
    }
    
    /// 读取人物指针
    fn read_person_ptr(&self) -> usize {
        use crate::game::address::Address;
        let memory = self.driver.memory();
        
        match memory.read_i64(Address::PERSON_PTR_ADDR as usize) {
            Ok(ptr) => ptr as usize,
            Err(e) => {
                error!("读取人物指针失败：{}", e);
                0
            }
        }
    }
    
    /// 获取当前游戏状态
    pub fn get_state(&self) -> GameState {
        self.game_state
    }
    
    /// 设置游戏状态
    fn set_state(&mut self, state: GameState) {
        debug!("游戏状态变更：{:?} -> {:?}", self.game_state, state);
        self.game_state = state;
    }
    
    /// 启动自动刷图线程
    pub fn start_auto_thread(&mut self) {
        info!("启动自动刷图线程...");
        
        let mut global = self.global_state.write();
        global.auto_switch = true;
        drop(global);
        
        // 克隆必要的引用
        let global_state = self.global_state.clone();
        self.running.store(true, Ordering::SeqCst);
        
        // 启动线程
        let running = self.running.clone();
        std::thread::spawn(move || {
            Self::auto_thread_loop(global_state, running);
        });
        
        info!("自动刷图线程已启动");
    }
    
    /// 停止自动刷图
    pub fn stop_auto_thread(&mut self) {
        info!("停止自动刷图...");
        self.running.store(false, Ordering::SeqCst);
        
        let mut global = self.global_state.write();
        global.auto_switch = false;
        drop(global);
    }
    
    /// 自动刷图主循环
    fn auto_thread_loop(global_state: GlobalState, running: AtomicBool) {
        info!("自动刷图线程运行中...");
        
        while running.load(Ordering::SeqCst) {
            // 检查开关
            {
                let global = global_state.read();
                if !global.auto_switch {
                    info!("自动刷图已停止");
                    break;
                }
            }
            
            // 执行自动逻辑
            if let Err(e) = Self::auto_step(&global_state) {
                error!("自动刷图步骤出错：{}", e);
            }
            
            // 短暂休眠
            timer::sleep(100);
        }
        
        info!("自动刷图线程退出");
    }
    
    /// 自动刷图单步执行
    fn auto_step(global_state: &GlobalState) -> Result<()> {
        let global = global_state.read();
        
        // 对话处理 - 检测并关闭对话框
        Self::handle_dialog(global_state)?;
        
        match global.map_id {
            0 => {
                // 在城镇
                debug!("城镇状态");
                drop(global);
                Self::handle_town(global_state)?;
            }
            _ => {
                // 在副本
                debug!("副本状态");
                drop(global);
                Self::handle_dungeon(global_state)?;
            }
        }
        
        Ok(())
    }
    
    /// 处理对话框
    fn handle_dialog(global_state: &GlobalState) -> Result<()> {
        // TODO: 检测游戏对话框并关闭
        // 可以通过检测特定内存地址或窗口来实现
        Ok(())
    }
    
    /// 城镇处理
    fn handle_town(global_state: &GlobalState) -> Result<()> {
        let global_read = global_state.read();
        
        // 检查疲劳值
        let pl = Self::read_pl(global_state);
        if pl <= 0 {
            warn!("疲劳值为 0，需要切换角色");
            drop(global_read);
            // TODO: 切换角色逻辑
            return Ok(());
        }
        
        debug!("当前疲劳值：{}", pl);
        
        // 选择地图
        let target_map = Self::select_map(global_state);
        if target_map.is_none() {
            warn!("未找到可刷的地图");
            return Ok(());
        }
        
        let (map_id, difficulty) = target_map.unwrap();
        drop(global_read);
        
        info!("准备进入地图：ID={}, 难度={}", map_id, difficulty);
        
        // 进入副本
        Self::enter_dungeon_internal(global_state, map_id, difficulty)?;
        
        Ok(())
    }
    
    /// 读取疲劳值
    fn read_pl(global_state: &GlobalState) -> i32 {
        use crate::game::address::Address;
        let global = global_state.read();
        let person_ptr = global.person_ptr;
        
        if person_ptr == 0 {
            return 0;
        }
        
        let pl_addr = person_ptr + Address::PL_OFFSET;
        // TODO: 实际读取内存
        // let memory = ...;
        // memory.read_i32(pl_addr).unwrap_or(0)
        
        156 // 模拟值
    }
    
    /// 选择地图
    fn select_map(global_state: &GlobalState) -> Option<(i32, i32)> {
        let global = global_state.read();
        
        // 从配置获取地图列表
        let auto_mode = global.map_id;
        
        if auto_mode == 0 {
            // 随机模式 - 返回默认地图
            Some((10086, 3)) // 模拟地图 ID 和难度
        } else {
            // 指定模式
            Some((auto_mode, 3))
        }
    }
    
    /// 进入副本内部实现
    fn enter_dungeon_internal(global_state: &GlobalState, map_id: i32, difficulty: i32) -> Result<()> {
        // 使用 CALL 系统进图
        // TODO: 获取 call_system 引用
        // call_system.enter_map_call(map_id, difficulty)?;
        
        info!("调用进图 CALL: map={}, diff={}", map_id, difficulty);
        
        // 更新状态
        let mut global = global_state.write();
        global.current_map_id = map_id;
        drop(global);
        
        Ok(())
    }
    
    /// 副本处理
    fn handle_dungeon(global_state: &GlobalState) -> Result<()> {
        // 1. 检测怪物
        let monsters = Self::detect_monsters(global_state)?;
        
        if monsters.is_empty() {
            // 没有怪物，可能已经通关或者需要过图
            info!("未检测到怪物，检查是否通关");
            return Self::check_dungeon_clear(global_state);
        }
        
        // 2. 选择最近怪物
        if let Some(target) = Self::find_nearest_monster(global_state, &monsters) {
            // 3. 移动到怪物位置
            Self::move_to_target(global_state, target.x, target.y, target.z)?;
            
            // 4. 释放技能
            Self::cast_skills(global_state, target.id)?;
        }
        
        // 5. 捡取物品
        Self::pick_items(global_state)?;
        
        Ok(())
    }
    
    /// 检测怪物
    fn detect_monsters(global_state: &GlobalState) -> Result<Vec<MonsterInfo>> {
        use crate::game::address::Address;
        
        // TODO: 从内存读取怪物列表
        // 这里实现模拟逻辑
        
        let mut monsters = Vec::new();
        
        // 模拟添加几个怪物
        monsters.push(MonsterInfo {
            id: 1001,
            x: 100,
            y: 200,
            z: 0,
            name: "哥布林".to_string(),
        });
        
        Ok(monsters)
    }
    
    /// 查找最近的怪物
    fn find_nearest_monster(global_state: &GlobalState, monsters: &[MonsterInfo]) -> Option<&MonsterInfo> {
        if monsters.is_empty() {
            return None;
        }
        
        // 获取玩家位置
        let player_pos = Self::get_player_position(global_state);
        
        // 计算距离并排序
        monsters.iter().min_by(|a, b| {
            let dist_a = ((a.x - player_pos.0).pow(2) + (a.y - player_pos.1).pow(2)) as f32;
            let dist_b = ((b.x - player_pos.0).pow(2) + (b.y - player_pos.1).pow(2)) as f32;
            dist_a.partial_cmp(&dist_b).unwrap()
        })
    }
    
    /// 获取玩家位置
    fn get_player_position(global_state: &GlobalState) -> (i32, i32, i32) {
        use crate::game::address::Address;
        let global = global_state.read();
        let person_ptr = global.person_ptr;
        
        if person_ptr == 0 {
            return (0, 0, 0);
        }
        
        // TODO: 从内存读取坐标
        // let x = memory.read_i32(person_ptr + Address::X_OFFSET).unwrap_or(0);
        // let y = memory.read_i32(person_ptr + Address::Y_OFFSET).unwrap_or(0);
        // let z = memory.read_i32(person_ptr + Address::Z_OFFSET).unwrap_or(0);
        
        (0, 0, 0)
    }
    
    /// 移动到目标位置
    fn move_to_target(global_state: &GlobalState, x: i32, y: i32, z: i32) -> Result<()> {
        info!("移动到目标：({}, {}, {})", x, y, z);
        
        // 使用移动 CALL
        // TODO: 获取 call_system
        // call_system.move_call(x, y, z)?;
        
        Ok(())
    }
    
    /// 释放技能
    fn cast_skills(global_state: &GlobalState, target_id: i32) -> Result<()> {
        let config = &global_state.read().config;
        
        // 从配置获取技能列表
        let skills = vec![
            config.skill_1_code,
            config.skill_2_code,
            config.skill_3_code,
        ];
        
        for skill_id in skills {
            if skill_id > 0 {
                let pos = Self::get_player_position(global_state);
                // TODO: call_system.skill_call(skill_id, pos.0, pos.1, pos.2)?;
                info!("释放技能：{}", skill_id);
                
                // 技能 CD
                timer::sleep(300);
            }
        }
        
        Ok(())
    }
    
    /// 捡取物品
    fn pick_items(global_state: &GlobalState) -> Result<()> {
        // TODO: 检测地面物品并捡取
        // 1. 读取物品列表
        // 2. 移动到物品位置
        // 3. 调用捡取 CALL
        
        Ok(())
    }
    
    /// 检查副本是否通关
    fn check_dungeon_clear(global_state: &GlobalState) -> Result<()> {
        // TODO: 检测 Boss 房间是否清空
        // 如果通关，调用过图或翻牌
        
        // 模拟判断：如果在最后一个房间，调用过图
        let global_read = global_state.read();
        let current_room = global_read.current_room;
        drop(global_read);
        
        if current_room >= 5 {
            // 假设第 5 个房间是 Boss 房
            info!("Boss 房间已清空，准备过图");
            Self::pass_to_next_room(global_state)?;
        }
        
        Ok(())
    }
    
    /// 过图到下一个房间
    fn pass_to_next_room(global_state: &GlobalState) -> Result<()> {
        info!("执行过图");
        
        // 确定过图方向
        let direction = Self::get_pass_direction(global_state);
        
        // 调用过图 CALL
        // TODO: call_system.pass_map_call(direction)?;
        
        // 更新房间计数
        let mut global = global_state.write();
        global.current_room += 1;
        drop(global);
        
        Ok(())
    }
    
    /// 获取过图方向
    fn get_pass_direction(global_state: &GlobalState) -> i32 {
        // TODO: 根据地图数据确定方向
        // 右：1, 左：2, 上：3, 下：4
        
        1 // 默认向右
    }
    
    /// 进入城镇
    pub fn enter_town(&mut self) -> Result<()> {
        info!("进入城镇");
        self.set_state(GameState::InTown);
        Ok(())
    }
    
    /// 进入副本
    pub fn enter_dungeon(&mut self, map_id: i32, difficulty: i32) -> Result<()> {
        info!("进入副本，地图 ID: {}, 难度：{}", map_id, difficulty);
        self.set_state(GameState::InDungeon);
        Ok(())
    }
    
    /// 退出副本
    pub fn quit_dungeon(&mut self) -> Result<()> {
        info!("退出副本");
        self.set_state(GameState::SelectingMap);
        Ok(())
    }
    
    /// 切换角色
    pub fn switch_role(&mut self, role_index: i32) -> Result<()> {
        info!("切换角色：{}", role_index);
        self.set_state(GameState::RoleSelect);
        Ok(())
    }
    
    /// 回城
    pub fn return_town(&mut self) -> Result<()> {
        info!("执行回城");
        
        // 调用城镇瞬移 CALL
        if let Some(ref mut cs) = self.call_system {
            cs.town_teleport(0)?; // 0 表示赫顿玛尔广场
        }
        
        self.set_state(GameState::InTown);
        Ok(())
    }
}

/// 怪物信息结构体
#[derive(Debug, Clone)]
struct MonsterInfo {
    id: i32,
    x: i32,
    y: i32,
    z: i32,
    name: String,
}
