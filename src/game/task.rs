//! 任务模块 - 实现主线任务处理、任务接取/提交等功能
//! 
//! 对应 Java: Task.java

use crate::driver::memory::Memory;
use crate::game::address::Address;
use crate::game::call_system::CallSystem;
use crate::game::send_pack::SendPack;
use crate::helper::strings;
use crate::helper::timer::Timer;
use log::{info, debug};

/// 任务管理器
pub struct Task {
    memory: Memory,
    call_system: CallSystem,
    send_pack: SendPack,
}

impl Task {
    pub fn new(memory: Memory, call_system: CallSystem, send_pack: SendPack) -> Self {
        Self {
            memory,
            call_system,
            send_pack,
        }
    }

    /// 处理任务，返回适合的地图 ID
    pub fn handle_task(&self, role_count: i32) -> i32 {
        let mut map_id = 0;
        let mut next_task_id = 0;
        let mut refresh_task = false;
        
        // 提交已有任务
        self.submit_task();

        loop {
            Timer::sleep(200);
            self.main_line_task();

            // 检测任务变化
            if Task::get_task_id() != next_task_id {
                next_task_id = Task::get_task_id();
                info!("任务名称 [{}]", Task::get_task_name());
            }

            // 无任务时刷新角色
            if Task::get_task_id() == 0 {
                if !refresh_task {
                    Timer::sleep(200);
                    info!("暂无任务或卡任务，重新选择角色");
                    // 返回角色
                    self.send_pack.return_role();
                    Timer::sleep(2000);
                    // 选择角色
                    self.send_pack.select_role(role_count - 1);
                    Timer::sleep(500);
                    refresh_task = true;
                    continue;
                } else {
                    map_id = self.highest_map();
                    info!("暂无任务，执行适应等级地图");
                    break;
                }
            }

            refresh_task = false;

            // 任务未接，执行接取任务
            if self.finish_status(Task::get_task_id()) == -1 {
                self.call_system.accept_task_call(Task::get_task_id());
            }

            // 跳过部分无法完成的任务
            // 任务名称 [返回赫顿玛尔], 任务条件 [[seek n meet npc]], 任务 ID[3509] 材料不足
            // 任务名称 [黑市的商人], 任务条件 [[seek n meet npc]], 任务 ID[5943] 蛇肉任务
            if Task::get_task_id() == 3509 || Task::get_task_id() == 5943 {
                map_id = self.highest_map();
                info!("无法完成任务，执行适应等级地图");
                break;
            }

            // 任务完成，执行提交任务
            if self.finish_status(Task::get_task_id()) == 0 {
                self.call_system.submit_task_call(Task::get_task_id());
                continue;
            }

            // 剧情条件判断
            match self.conditional(&Task::get_task_condition()) {
                1 => {
                    self.call_system.finish_task_call(Task::get_task_id());
                }
                2 => {
                    map_id = self.task_map(Task::get_task_id());
                    if map_id > 0 {
                        break;
                    }
                }
                3 => {
                    info!("材料任务无法自动完成，执行最高等级地图");
                }
                _ => {}
            }
        }

        map_id
    }

    /// 主线任务检测
    pub fn main_line_task(&self) {
        let task_address = self.memory.read_i64(Address::TASK_ADDR);
        let start = self.memory.read_i64(task_address + Address::QB_RW_START_ADDR);
        let end = self.memory.read_i64(task_address + Address::QB_RW_END_ADDR);
        let num = (end - start) / 8;

        for i in 0..num {
            let task_ptr = self.memory.read_i64(start + i * 8);
            let task_type = self.memory.read_i32(task_ptr + Address::RW_LX_ADDR);
            
            if task_type == 0 {
                let task_length = self.memory.read_i32(task_ptr + Address::RW_DX_ADDR);
                
                let task_name = if task_length > 7 {
                    let name_ptr = self.memory.read_i64(task_ptr + 16);
                    let name_bytes = self.memory.read_bytes(name_ptr, 100);
                    strings::unicode_to_ascii(&name_bytes)
                } else {
                    let name_bytes = self.memory.read_bytes(task_ptr + 16, 100);
                    strings::unicode_to_ascii(&name_bytes)
                };

                // 任务条件
                let condition_ptr = self.memory.read_i64(task_ptr + Address::RW_TJ_ADDR);
                let condition_bytes = self.memory.read_bytes(condition_ptr, 100);
                let task_condition = strings::unicode_to_ascii(&condition_bytes);

                // 任务编号
                let task_id = self.memory.read_i32(task_ptr);

                Task::set_task_name(task_name);
                Task::set_task_condition(task_condition);
                Task::set_task_id(task_id);
                break;
            }
        }
    }

    /// 提交任务
    pub fn submit_task(&self) {
        let task_address = self.memory.read_i64(Address::TASK_ADDR);
        let start = self.memory.read_i64(task_address + Address::QB_RW_START_ADDR);
        let end = self.memory.read_i64(task_address + Address::QB_RW_END_ADDR);
        let num = (end - start) / 8;

        for i in 0..num {
            let task_ptr = self.memory.read_i64(start + i * 8);
            let task_type = self.memory.read_i32(task_ptr + Address::RW_LX_ADDR);
            
            if task_type == 0 {
                let task_id = self.memory.read_i32(task_ptr);
                if self.finish_status(task_id) == 0 {
                    self.call_system.submit_task_call(task_id);
                }
            }
        }
    }

    /// 检查任务完成状态
    /// 返回：-1=未接，0=已完成可提交，1=进行中
    fn finish_status(&self, task_id: i32) -> i32 {
        let task_address = self.memory.read_i64(Address::TASK_ADDR);
        let completed_start = self.memory.read_i64(task_address + Address::WC_RW_START_ADDR);
        let completed_end = self.memory.read_i64(task_address + Address::WC_RW_END_ADDR);
        let completed_num = (completed_end - completed_start) / 8;

        for i in 0..completed_num {
            let completed_ptr = self.memory.read_i64(completed_start + i * 8);
            let completed_id = self.memory.read_i32(completed_ptr);
            
            if completed_id == task_id {
                return 0; // 已完成
            }
        }

        // 检查是否已接取
        let accepting_start = self.memory.read_i64(task_address + Address::QS_RW_START_ADDR);
        let accepting_end = self.memory.read_i64(task_address + Address::QS_RW_END_ADDR);
        let accepting_num = (accepting_end - accepting_start) / 8;

        for i in 0..accepting_num {
            let accepting_ptr = self.memory.read_i64(accepting_start + i * 8);
            let accepting_id = self.memory.read_i32(accepting_ptr);
            
            if accepting_id == task_id {
                return 1; // 进行中
            }
        }

        -1 // 未接
    }

    /// 条件判断
    /// 返回：1=剧情条件，2=刷图任务，3=材料任务，0=未知
    fn conditional(&self, condition: &str) -> i32 {
        if condition.is_empty() {
            return 0;
        }

        // 剧情条件判断
        if condition.contains("seek") && condition.contains("meet") && condition.contains("npc") {
            return 1;
        }

        // 刷图任务判断
        if condition.contains("kill") || condition.contains("dungeon") || condition.contains("hunt") {
            return 2;
        }

        // 材料任务判断
        if condition.contains("item") || condition.contains("collect") || condition.contains("material") {
            return 3;
        }

        0
    }

    /// 获取适合等级的地图
    fn highest_map(&self) -> i32 {
        let role_level = self.memory.read_i32(Address::JS_DJ_ADDR);
        
        // 根据等级返回推荐地图（简化实现）
        match role_level {
            0..=10 => 100,
            11..=20 => 200,
            21..=30 => 300,
            31..=40 => 400,
            41..=50 => 500,
            51..=60 => 600,
            61..=70 => 700,
            71..=80 => 800,
            81..=90 => 900,
            91..=100 => 1000,
            101..=110 => 1100,
            _ => 1100,
        }
    }

    /// 根据任务 ID 获取地图
    fn task_map(&self, task_id: i32) -> i32 {
        // 简化实现：根据任务 ID 映射到地图
        // 实际需要根据游戏数据配置
        match task_id {
            1..=100 => 100,
            101..=200 => 200,
            201..=300 => 300,
            301..=400 => 400,
            401..=500 => 500,
            _ => 0,
        }
    }

    // ==================== 静态方法用于存储当前任务信息 ====================
    
    fn get_task_name() -> String {
        use std::sync::{Mutex, OnceLock};
        static TASK_NAME: OnceLock<Mutex<String>> = OnceLock::new();
        let name = TASK_NAME.get_or_init(|| Mutex::new(String::new()));
        name.lock().unwrap().clone()
    }

    fn set_task_name(name: String) {
        use std::sync::{Mutex, OnceLock};
        static TASK_NAME: OnceLock<Mutex<String>> = OnceLock::new();
        let task_name = TASK_NAME.get_or_init(|| Mutex::new(String::new()));
        *task_name.lock().unwrap() = name;
    }

    fn get_task_condition() -> String {
        use std::sync::{Mutex, OnceLock};
        static TASK_CONDITION: OnceLock<Mutex<String>> = OnceLock::new();
        let condition = TASK_CONDITION.get_or_init(|| Mutex::new(String::new()));
        condition.lock().unwrap().clone()
    }

    fn set_task_condition(condition: String) {
        use std::sync::{Mutex, OnceLock};
        static TASK_CONDITION: OnceLock<Mutex<String>> = OnceLock::new();
        let task_condition = TASK_CONDITION.get_or_init(|| Mutex::new(String::new()));
        *task_condition.lock().unwrap() = condition;
    }

    fn get_task_id() -> i32 {
        use std::sync::{AtomicI32, OnceLock};
        static TASK_ID: OnceLock<AtomicI32> = OnceLock::new();
        let id = TASK_ID.get_or_init(|| AtomicI32::new(0));
        id.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn set_task_id(id: i32) {
        use std::sync::{AtomicI32, OnceLock};
        static TASK_ID: OnceLock<AtomicI32> = OnceLock::new();
        let task_id = TASK_ID.get_or_init(|| AtomicI32::new(0));
        task_id.store(id, std::sync::atomic::Ordering::SeqCst);
    }
}
