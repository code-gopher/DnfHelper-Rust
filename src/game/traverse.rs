//! 遍历模块 - 实现物品拾取、装备处理、跟随打怪等功能
//! 
//! 对应 Java: Traverse.java

use crate::config::Config;
use crate::driver::memory::Memory;
use crate::game::address::Address;
use crate::game::call_system::CallSystem;
use crate::game::send_pack::SendPack;
use crate::helper::timer::Timer;
use log::{debug, info};
use std::collections::HashSet;

/// 遍历管理器
pub struct Traverse {
    memory: Memory,
    call_system: CallSystem,
    send_pack: SendPack,
    config: Config,
}

impl Traverse {
    pub fn new(memory: Memory, call_system: CallSystem, send_pack: SendPack, config: Config) -> Self {
        Self {
            memory,
            call_system,
            send_pack,
            config,
        }
    }

    /// 读取过滤物品配置
    fn get_item_filter(&self) -> HashSet<String> {
        let item_str = self.config.read::<String>("自动配置", "过滤物品");
        item_str.split(',')
            .map(|s| s.trim().to_string())
            .collect()
    }

    /// 组包拾取
    pub fn pack_pickup(&self) {
        let game_state = self.memory.read_i32(Address::YXZT_ADDR);
        if game_state != 3 {
            return;
        }

        let item_filter = self.get_item_filter();
        let map_data = self.get_map_traversal_data();
        
        for i in 1..map_data.obj_num {
            let obj_ptr = self.get_traversal_ptr(map_data.start, i as i64, 2);
            if obj_ptr <= 0 {
                continue;
            }

            let obj_type_a = self.memory.read_i32(obj_ptr + Address::LX_PY_ADDR);
            let obj_type_b = self.memory.read_i32(obj_ptr + Address::LX_PY_ADDR + 4);
            let obj_camp = self.memory.read_i32(obj_ptr + Address::ZY_PY_ADDR);

            // 检查是否为物品 (类型 289) 且阵营为 200
            if (obj_type_a == 289 || obj_type_b == 289) && obj_camp == 200 {
                // 读取物品名称
                let dm_wp_addr = self.memory.read_i64(obj_ptr + Address::DM_WP_ADDR);
                let wp_mc_addr = self.memory.read_i64(dm_wp_addr + Address::WP_MC_ADDR);
                let name_bytes = self.memory.read_bytes(wp_mc_addr, 100);
                let obj_name = String::from_utf8_lossy(&name_bytes)
                    .trim_end_matches('\0')
                    .to_string();

                // 检查是否在过滤列表中
                if item_filter.contains(&obj_name) {
                    continue;
                }

                // 避免重复拾取
                if obj_ptr != map_data.rw_addr {
                    let res_address = self.decode(obj_ptr + Address::FB_SQ_ADDR);
                    self.send_pack.pick_up(res_address as i64);
                    debug!("拾取物品：{}", obj_name);
                }
            }
        }
    }

    /// 处理装备 (分解/出售)
    pub fn handle_equip(&self) {
        let handle_mode = self.config.read::<i32>("自动配置", "处理装备");
        if handle_mode == 0 {
            return;
        }

        let backpack_weight = self.backpack_weight();
        if backpack_weight < 60 {
            return;
        }

        let mut num = 0;
        let person_ptr = self.call_system.person_ptr();
        let back_pack_ptr = self.memory.read_i64(person_ptr + Address::WPL_ADDR);
        let mut address = self.memory.read_i64(back_pack_ptr + Address::WPL_PY_ADDR) + 0x48;

        for i in 1..=56 {
            let equip = self.get_traversal_ptr(address, i as i64, 1);
            if equip > 0 {
                let equip_level = self.memory.read_i32(equip + Address::ZB_PJ_ADDR);
                let name_address = self.memory.read_i32(equip + Address::WP_MC_ADDR);
                let name_bytes = self.memory.read_bytes(name_address as i64, 100);
                let equip_name = String::from_utf8_lossy(&name_bytes)
                    .trim_end_matches('\0')
                    .to_string();

                if equip_name.is_empty() {
                    break;
                }

                // 0=白装，1=蓝装 -> 分解
                if equip_level == 0 || equip_level == 1 {
                    info!("分解装备 [{}]", equip_name);
                    self.send_pack.decomposition((i + 8) as i32);
                    Timer::sleep(200);
                    num += 1;
                }

                // 2=紫装，3=粉装 -> 出售
                if equip_level == 2 || equip_level == 3 {
                    info!("出售装备 [{}]", equip_name);
                    self.send_pack.sell_equip((i + 8) as i32);
                    Timer::sleep(200);
                    num += 1;
                }
            }
        }

        self.send_pack.tidy_backpack(0, 0);
        info!("处理装备 [{}] 件", num);
    }

    /// 跟随怪物
    pub fn follow_monster(&self) {
        let game_state = self.memory.read_i32(Address::YXZT_ADDR);
        if game_state != 3 {
            return;
        }

        let follow_mode = self.config.read::<i32>("自动配置", "跟随打怪");
        let skill_code = self.config.read::<i32>("自动配置", "技能代码");
        let skill_harm = self.config.read::<i32>("自动配置", "技能伤害");
        let skill_size = self.config.read::<i32>("自动配置", "技能大小");

        let map_data = self.get_map_traversal_data();
        let person_ptr = map_data.rw_addr;

        for i in 1..map_data.obj_num {
            let obj_ptr = self.get_traversal_ptr(map_data.start, i as i64, 2);
            if obj_ptr <= 0 {
                continue;
            }

            let obj_type_a = self.memory.read_i32(obj_ptr + Address::LX_PY_ADDR);
            
            // 怪物类型判断 (529, 545, 273, 61440, 1057)
            if ![529, 545, 273, 61440, 1057].contains(&obj_type_a) {
                continue;
            }

            let obj_camp = self.memory.read_i32(obj_ptr + Address::ZY_PY_ADDR);
            let obj_code = self.memory.read_i32(obj_ptr + Address::DM_PY_ADDR);
            let obj_blood = self.memory.read_i64(obj_ptr + Address::GW_XL_ADDR);

            if obj_camp > 0 && obj_ptr != person_ptr && obj_blood > 0 {
                let coordinate = self.read_coordinate(obj_ptr);
                let name_bytes = self.memory.read_bytes(
                    self.memory.read_i64(obj_ptr + Address::MC_PY_ADDR), 
                    200
                );
                let obj_name = String::from_utf8_lossy(&name_bytes)
                    .trim_end_matches('\0')
                    .to_string();

                debug!(
                    "对象名称:[{}], 类型:[{}], 阵营:[{}], 代码:[{}], 血量:[{}], X:[{}], Y:[{}]",
                    obj_name, obj_type_a, obj_camp, obj_code, obj_blood, coordinate.0, coordinate.1
                );

                // 漂移 CALL 到怪物位置
                self.call_system.drift_call(person_ptr, coordinate.0, coordinate.1, 0, 0);

                // 跟随打怪模式 2: 按键攻击
                if follow_mode == 2 {
                    use winapi::um::winuser::{VK_A, VK_D, VK_E, VK_F, VK_G, VK_H, VK_Q, VK_R, VK_S, VK_T, VK_W, VK_X, VK_Y};
                    
                    let vk_codes = [
                        VK_A as u32, VK_S as u32, VK_D as u32, VK_F as u32,
                        VK_G as u32, VK_H as u32, VK_Q as u32, VK_W as u32,
                        VK_E as u32, VK_R as u32, VK_T as u32, VK_Y as u32,
                        VK_X as u32,
                    ];

                    // 按下 X 键
                    crate::helper::process::Process::send_key(VK_X as u32);
                    Timer::sleep(800);
                    // 释放 X 键
                    crate::helper::process::Process::release_key(VK_X as u32);
                    Timer::sleep(100);

                    // 随机按一个技能键
                    use rand::Rng;
                    let mut rng = rand::thread_rng();
                    let random_index = rng.gen_range(0..vk_codes.len());
                    crate::helper::process::Process::send_key(vk_codes[random_index]);
                } else {
                    // 使用技能 CALL 攻击
                    self.call_system.skill_call(
                        person_ptr,
                        skill_code,
                        skill_harm,
                        coordinate.0,
                        coordinate.1,
                        0,
                        skill_size as f32,
                    );
                }

                // 找到一个目标后退出循环
                break;
            }
        }
    }

    /// 检查是否存在可拾取物品
    pub fn is_exists_item(&self) -> bool {
        let game_state = self.memory.read_i32(Address::YXZT_ADDR);
        if game_state != 3 {
            return false;
        }

        let map_data = self.get_map_traversal_data();
        
        for i in 1..map_data.obj_num {
            let obj_ptr = self.get_traversal_ptr(map_data.start, i as i64, 2);
            if obj_ptr <= 0 {
                continue;
            }

            let obj_type_a = self.memory.read_i32(obj_ptr + Address::LX_PY_ADDR);
            let obj_type_b = self.memory.read_i32(obj_ptr + Address::LX_PY_ADDR + 4);
            let obj_camp = self.memory.read_i32(obj_ptr + Address::ZY_PY_ADDR);

            if (obj_type_a == 289 || obj_type_b == 289) && obj_camp == 200 {
                return true;
            }
        }
        
        false
    }

    // ==================== 工具方法 ====================

    /// 解密
    fn decode(&self, address: i64) -> i32 {
        self.memory.read_i32(address)
    }

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

    /// 背包负重百分比
    fn backpack_weight(&self) -> i32 {
        let person_ptr = self.call_system.person_ptr();
        let back_pack_ptr = self.memory.read_i64(person_ptr + Address::WPL_ADDR);
        let cut_weight = self.decode(back_pack_ptr + 0x58);
        let max_weight = self.decode(person_ptr + Address::ZD_FZ_ADDR);
        
        if max_weight == 0 {
            return 0;
        }
        
        let result = (cut_weight as f32 / max_weight as f32) * 100.0;
        debug!("背包负重：{:.2}%", result);
        result as i32
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
