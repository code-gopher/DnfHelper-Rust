//! 封包发送模块
//! 
//! 实现与 Java SendPack.java 相同的功能
//! 包含缓冲 CALL、加密 CALL、发包 CALL 以及各种游戏协议包的组装和发送

use crate::game::address::Address;
use crate::game::call_system::CallSystem;
use crate::helper::bytes::{Bytes, add_bytes};

/// 封包发送器
pub struct SendPack {
    call_system: CallSystem,
    data: Vec<i32>,
}

impl SendPack {
    pub fn new(call_system: CallSystem) -> Self {
        Self {
            call_system,
            data: Vec::new(),
        }
    }

    /// 缓冲 CALL
    fn hc_call(&mut self, param: i32) {
        let mut shell = self.call_system.sub_rsp(256);
        shell = add_bytes(shell, vec![0x48, 0xB9]);
        shell = add_bytes(shell, Bytes::int_to_bytes(Address::FB_ADDR as i64));
        shell = add_bytes(shell, vec![0xBA]);
        shell = add_bytes(shell, Bytes::int_to_bytes(param));
        shell = add_bytes(shell, vec![0x48, 0xB8]);
        shell = add_bytes(shell, Bytes::int_to_bytes(Address::HC_CALL_ADDR as i64));
        shell = add_bytes(shell, vec![0xFF, 0xD0]);
        shell = add_bytes(shell, self.call_system.add_rsp(256));
        
        self.data = shell;
    }

    /// 加密 CALL
    fn jm_call(&mut self, param: i64, length: i32) {
        self.data = add_bytes(self.data.clone(), self.call_system.sub_rsp(256));
        self.data = add_bytes(self.data.clone(), vec![0x48, 0xB9]);
        self.data = add_bytes(self.data.clone(), Bytes::int_to_bytes(Address::FB_ADDR as i64));
        self.data = add_bytes(self.data.clone(), vec![0x48, 0xBA]);
        self.data = add_bytes(self.data.clone(), Bytes::int_to_bytes(param));
        
        match length {
            1 => {
                self.data = add_bytes(self.data.clone(), vec![0x48, 0xB8]);
                self.data = add_bytes(self.data.clone(), Bytes::int_to_bytes(Address::JM_B1_CALL_ADDR as i64));
            }
            2 => {
                self.data = add_bytes(self.data.clone(), vec![0x48, 0xB8]);
                self.data = add_bytes(self.data.clone(), Bytes::int_to_bytes(Address::JM_B2_CALL_ADDR as i64));
            }
            4 => {
                self.data = add_bytes(self.data.clone(), vec![0x48, 0xB8]);
                self.data = add_bytes(self.data.clone(), Bytes::int_to_bytes(Address::JM_B3_CALL_ADDR as i64));
            }
            8 => {
                self.data = add_bytes(self.data.clone(), vec![0x48, 0xB8]);
                self.data = add_bytes(self.data.clone(), Bytes::int_to_bytes(Address::JM_B4_CALL_ADDR as i64));
            }
            _ => {}
        }
        
        self.data = add_bytes(self.data.clone(), vec![0xFF, 0xD0]);
        self.data = add_bytes(self.data.clone(), self.call_system.add_rsp(256));
    }

    /// 发包 CALL
    fn fb_call(&mut self) {
        self.data = add_bytes(self.data.clone(), self.call_system.sub_rsp(256));
        self.data = add_bytes(self.data.clone(), vec![0x48, 0xB8]);
        self.data = add_bytes(self.data.clone(), Bytes::int_to_bytes(Address::FB_CALL_ADDR as i64));
        self.data = add_bytes(self.data.clone(), vec![0xFF, 0xD0]);
        self.data = add_bytes(self.data.clone(), self.call_system.add_rsp(256));
        
        // 执行编译调用
        self.call_system.compile_call(&self.data);
        self.data.clear();
    }

    /// 组包返回角色
    pub fn return_role(&mut self) {
        self.hc_call(7);
        self.fb_call();
    }

    /// 组包选择角色
    pub fn select_role(&mut self, index: i32) {
        if index == 0 {
            return;
        }
        self.hc_call(4);
        self.jm_call(index as i64, 2);
        self.fb_call();
    }

    /// 组包选图
    pub fn select_map(&mut self) {
        self.hc_call(15);
        self.jm_call(0, 4);
        self.fb_call();
    }

    /// 组包进图
    pub fn go_map(&mut self, bh: i64, nd: i64, sy: i32, lx: i32) {
        self.hc_call(16);
        self.jm_call(bh, 4);
        self.jm_call(nd, 1);
        self.jm_call(0, 2);
        self.jm_call(sy, 1);
        self.jm_call(lx, 1);
        self.jm_call(65535, 2);
        self.jm_call(0, 4);
        self.jm_call(0, 1);
        self.jm_call(0, 4);
        self.jm_call(0, 1);
        self.jm_call(0, 4);
        self.fb_call();
    }

    /// 组包翻牌
    pub fn get_income(&mut self, h: i32, l: i32) {
        self.hc_call(69);
        self.fb_call();
        self.hc_call(70);
        self.fb_call();
        self.hc_call(71);
        self.jm_call(h as i64, 1);
        self.jm_call(l as i64, 1);
        self.fb_call();
        self.hc_call(1426);
        self.fb_call();
    }

    /// 组包出图
    pub fn leave_map(&mut self) {
        self.hc_call(42);
        self.fb_call();
    }

    /// 组包移动
    pub fn move_map(&mut self, max_map: i64, mix_map: i64, x: i64, y: i64) {
        if max_map < 0 || mix_map < 0 || x < 0 || y < 0 {
            return;
        }
        self.hc_call(36);
        self.jm_call(max_map, 4);
        self.jm_call(mix_map, 4);
        self.jm_call(x, 2);
        self.jm_call(y, 2);
        self.jm_call(5, 1);
        self.jm_call(38, 4);
        self.jm_call(1, 2);
        self.jm_call(0, 4);
        self.jm_call(0, 1);
        self.jm_call(5, 1);
        self.fb_call();
    }

    /// 组包拾取
    pub fn pick_up(&mut self, addr: i64) {
        if addr < 0 {
            return;
        }
        self.hc_call(43);
        self.jm_call(addr, 4);
        self.jm_call(0, 1);
        self.jm_call(1, 1);
        self.jm_call(420, 2);
        self.jm_call(254, 2);
        self.jm_call(4501, 2);
        self.jm_call(435, 2);
        self.jm_call(271, 2);
        self.jm_call(22624, 2);
        self.jm_call(28402, 2);
        self.jm_call(0, 1);
        self.fb_call();
    }

    /// 组包分解
    pub fn decomposition(&mut self, address: i32) {
        if address < 0 {
            return;
        }
        self.hc_call(26);
        self.jm_call(0, 1);
        self.jm_call(65535, 2);
        self.jm_call(317, 4);
        self.jm_call(1, 1);
        self.jm_call(address as i64, 2);
        self.fb_call();
    }

    /// 组包出售
    pub fn sell_equip(&mut self, index: i64) {
        if index < 0 {
            return;
        }
        self.hc_call(22);
        self.jm_call(317, 4);
        self.jm_call(95, 4);
        self.jm_call(1, 1);
        self.jm_call(0, 1);
        self.jm_call(index, 2);
        self.jm_call(1, 4);
        self.jm_call(index * 2 + 2, 4);
        self.fb_call();
    }

    /// 整理背包
    pub fn tidy_backpack(&mut self, pack_type: i32, pack_address: i32) {
        self.hc_call(20);
        self.jm_call(6, 4);
        self.jm_call(16, 1);
        self.jm_call(pack_type as i64, 1);    // 背包类型:1 装备;2 消耗品;3 材料;4 任务;10 副职业
        self.jm_call(pack_address as i64, 1); // 背包地址:0 背包;2 个人仓库;12 账号金库
        self.jm_call(pack_address as i64, 1); // 排序方式:0 栏位排序;1 品级排序;2Lv 排序;3 部位排序
        self.fb_call();
    }

    /// 接受任务
    pub fn accept_task(&mut self, task_id: i32) {
        self.hc_call(31);
        self.jm_call(31, 2);
        self.jm_call(task_id as i64, 2);
        self.fb_call();
    }

    /// 放弃任务
    pub fn give_up_task(&mut self, task_id: i32) {
        self.hc_call(32);
        self.jm_call(32, 2);
        self.jm_call(task_id as i64, 2);
        self.fb_call();
    }

    /// 完成任务
    pub fn finish_task(&mut self, task_id: i32) {
        self.hc_call(33);
        self.jm_call(33, 2);
        self.jm_call(task_id as i64, 2);
        self.jm_call(0, 1);
        self.jm_call(0, 1);
        self.fb_call();
    }

    /// 提交任务
    pub fn submit_task(&mut self, task_id: i32) {
        self.hc_call(34);
        self.jm_call(34, 2);
        self.jm_call(task_id as i64, 2);
        self.jm_call(65535, 2);
        self.jm_call(1, 2);
        self.jm_call(65535, 2);
        self.fb_call();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_pack_creation() {
        // 测试代码，实际需要初始化 CallSystem
        // let call_system = CallSystem::new();
        // let pack = SendPack::new(call_system);
        // assert!(true);
    }
}
