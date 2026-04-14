//! 屏幕技能模块 - 实现全屏技能和秒杀功能
//! 
//! 对应 Java: Screen.java

use crate::config::Config;
use crate::driver::memory::Memory;
use crate::game::address::Address;
use crate::game::call_system::CallSystem;
use crate::helper::timer::Timer;
use log::{info, debug};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

/// 屏幕技能管理器
pub struct Screen {
    memory: Memory,
    call_system: CallSystem,
    config: Config,
    is_running: AtomicBool,
}

impl Screen {
    pub fn new(memory: Memory, call_system: CallSystem, config: Config) -> Self {
        Self {
            memory,
            call_system,
            config,
            is_running: AtomicBool::new(false),
        }
    }

    /// 切换全屏技能开关
    pub fn toggle_screen(&self) {
        let current = self.is_running.load(Ordering::SeqCst);
        self.is_running.store(!current, Ordering::SeqCst);
        
        if !current {
            let thread = thread::spawn({
                let this = Self {
                    memory: self.memory.clone(),
                    call_system: self.call_system.clone(),
                    config: self.config.clone(),
                    is_running: AtomicBool::new(true),
                };
                move || {
                    while this.is_running.load(Ordering::SeqCst) {
                        Timer::sleep(300);
                        this.full_screen();
                    }
                }
            });
            info!("技能全屏 - [ √ ]");
            std::mem::forget(thread);
        } else {
            info!("技能全屏 - [ x ]");
        }
    }

    /// 全屏技能逻辑
    fn full_screen(&self) {
        let game_state = self.memory.read_i32(Address::YXZT_ADDR);
        if game_state != 3 {
            return;
        }

        let screen_code = self.config.read::<i32>("自动配置", "技能代码");
        let screen_harm = self.config.read::<i32>("自动配置", "技能伤害");
        let screen_size = self.config.read::<i32>("自动配置", "技能大小");
        let screen_number = self.config.read::<i32>("自动配置", "技能个数");

        let mut num = 0;
        let map_data = self.get_map_traversal_data();
        let person_ptr = map_data.rw_addr;

        for i in 1..map_data.obj_num {
            let obj_ptr = self.get_traversal_ptr(map_data.start, i as i64, 2);
            if obj_ptr <= 0 {
                continue;
            }

            let obj_type_a = self.memory.read_i32(obj_ptr + Address::LX_PY_ADDR);
            let obj_camp = self.memory.read_i32(obj_ptr + Address::ZY_PY_ADDR);
            let obj_code = self.memory.read_i32(obj_ptr + Address::DM_PY_ADDR);

            // 怪物类型判断 (529, 545, 273, 61440)
            if ![529, 545, 273, 61440].contains(&obj_type_a) {
                continue;
            }

            let obj_blood = self.memory.read_i64(obj_ptr + Address::GW_XL_ADDR);

            if obj_camp > 0 && obj_code > 0 && obj_blood > 0 && obj_ptr != person_ptr {
                let coordinate = self.read_coordinate(obj_ptr);
                
                self.call_system.skill_call(
                    person_ptr,
                    screen_code,
                    screen_harm,
                    coordinate.0,
                    coordinate.1,
                    0,
                    screen_size as f32,
                );

                num += 1;
                if num >= screen_number {
                    break;
                }
            }
        }
    }

    /// 秒杀所有怪物
    pub fn screen_kill(&self) {
        // 使用固定技能代码 54141 进行秒杀
        self.call_system.skill_call(0, 54141, 0, 0, 0, 0, 1.0);
        info!("秒杀完毕 - [ √ ]");
    }

    // ==================== 工具方法 ====================

    /// 获取地图遍历数据
    fn get_map_traversal_data(&self) -> MapTraversalData {
        let rw_addr = self.call_system.person_ptr();
        let dt_addr = self.memory.read_i64(rw_addr + Address::DT_PY_ADDR - 8);
        let map_data = self.memory.read_i64(dt_addr + 16);
        let start = self.memory.read_i64(map_data + Address::DT_KS2);
        let end = self.memory.read_i64(map_data + Address::DT_JS2);
        let obj_num = ((end - start) / 24) as usize;

        MapTraversalData {
            rw_addr,
            start,
            end,
            obj_num,
        }
    }

    /// 获取遍历指针
    fn get_traversal_ptr(&self, ptr: i64, offset: i64, t: i32) -> i64 {
        match t {
            1 => {
                let one = self.memory.read_i64(ptr + (offset - 1) * 8);
                let two = self.memory.read_i64(one - 72);
                self.memory.read_i64(two + 16)
            }
            2 => {
                let one = self.memory.read_i64(ptr + (offset - 1) * 24);
                self.memory.read_i64(one + 16) - 48
            }
            _ => 0,
        }
    }

    /// 读取坐标
    fn read_coordinate(&self, param: i64) -> (i32, i32, i32) {
        let obj_type = self.memory.read_i32(param + Address::LX_PY_ADDR);
        
        if obj_type == 273 {
            let ptr = self.memory.read_i64(param + Address::DQ_ZB_ADDR);
            let x = self.memory.read_f32(ptr) as i32;
            let y = self.memory.read_f32(ptr + 4) as i32;
            let z = self.memory.read_f32(ptr + 8) as i32;
            (x, y, z)
        } else {
            let ptr = self.memory.read_i64(param + Address::FX_PY_ADDR);
            let x = self.memory.read_f32(ptr + 32) as i32;
            let y = self.memory.read_f32(ptr + 36) as i32;
            let z = self.memory.read_f32(ptr + 40) as i32;
            (x, y, z)
        }
    }
}

/// 地图遍历数据结构
#[derive(Debug)]
struct MapTraversalData {
    rw_addr: i64,
    start: i64,
    end: i64,
    obj_num: usize,
}
