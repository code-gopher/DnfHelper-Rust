//! CALL 调用系统模块
//! 
//! 实现游戏内部函数调用和汇编注入

use anyhow::Result;
use log::{info, debug, error};
use crate::driver::memory::ReadWriteMemory;

#[cfg(windows)]
use windows::{
    Win32::Foundation::{HANDLE, FALSE},
    Win32::System::Threading::{CreateRemoteThread, WaitForSingleObject, INFINITE},
    Win32::System::Memory::{WriteProcessMemory, VirtualAllocEx, VirtualFreeEx, MEM_COMMIT, MEM_RESERVE, MEM_RELEASE, PAGE_EXECUTE_READWRITE},
};

/// CALL 调用器结构体
pub struct CallSystem<M: ReadWriteMemory> {
    memory: M,
    /// 汇编空白地址
    blank_addr: usize,
    /// 是否已初始化
    initialized: bool,
    /// 进程句柄 (Windows)
    #[cfg(windows)]
    process_handle: Option<HANDLE>,
}

impl<M: ReadWriteMemory> CallSystem<M> {
    /// 创建新的 CALL 系统实例
    pub fn new(memory: M) -> Self {
        Self {
            memory,
            blank_addr: 0,
            initialized: false,
            #[cfg(windows)]
            process_handle: None,
        }
    }
    
    /// 设置进程句柄
    pub fn set_process_handle(&mut self, handle: usize) {
        #[cfg(windows)]
        {
            self.process_handle = Some(HANDLE(handle as *mut _));
        }
    }
    
    /// 初始化 CALL 系统
    pub fn initialize(&mut self) -> Result<()> {
        info!("初始化 CALL 系统...");
        
        // 分配内存空间用于注入代码
        self.blank_addr = self.memory.allocate(0x2000)?;
        info!("分配汇编空白地址：0x{:X}", self.blank_addr);
        
        self.initialized = true;
        Ok(())
    }
    
    /// 写入汇编指令到指定地址
    pub fn write_assembly(&self, address: usize, code: &[u8]) -> Result<()> {
        debug!("写入汇编指令到 0x{:X}, 长度：{}", address, code.len());
        self.memory.write_bytes(address, code)
    }
    
    /// 执行远程 CALL (Windows 完整实现)
    pub fn remote_call(&self, call_address: usize, params: &[usize]) -> Result<i32> {
        if !self.initialized {
            return Err(anyhow::anyhow!("CALL 系统未初始化"));
        }
        
        debug!("执行远程 CALL: 0x{:X}, 参数数量：{}", call_address, params.len());
        
        #[cfg(windows)]
        {
            use std::ptr;
            
            let h_process = self.process_handle.context("进程句柄未设置")?;
            
            // 在空白地址构建汇编代码
            // x86 调用约定：参数从右向左压栈
            let mut asm_code: Vec<u8> = Vec::new();
            
            // 保存现场
            asm_code.extend_from_slice(&[0x60]); // pushad
            asm_code.extend_from_slice(&[0x9C]); // pushfd
            
            // 压入参数 (从右向左)
            for &param in params.iter().rev() {
                asm_code.extend_from_slice(&[0x68]); // push imm32
                asm_code.extend_from_slice(&(param as u32).to_le_bytes());
            }
            
            // CALL 目标地址
            asm_code.extend_from_slice(&[0xE8]); // call rel32
            let rel_addr = (call_address as i32) - ((self.blank_addr + asm_code.len() + 4) as i32);
            asm_code.extend_from_slice(&rel_addr.to_le_bytes());
            
            // 清理堆栈 (参数个数 * 4)
            if !params.is_empty() {
                let stack_clean = (params.len() * 4) as u16;
                asm_code.extend_from_slice(&[0x81, 0xC4]); // esp += xxx
                asm_code.extend_from_slice(&stack_clean.to_le_bytes());
            }
            
            // 保存返回值
            asm_code.extend_from_slice(&[0xA3]); // mov [addr], eax
            let result_addr = self.blank_addr + 0x1000; // 使用空白地址的后半部分存储结果
            asm_code.extend_from_slice(&(result_addr as u32).to_le_bytes());
            
            // 恢复现场
            asm_code.extend_from_slice(&[0x9D]); // popfd
            asm_code.extend_from_slice(&[0x61]); // popad
            
            // 返回
            asm_code.extend_from_slice(&[0xC3]); // ret
            
            // 写入汇编代码
            self.memory.write_bytes(self.blank_addr, &asm_code)?;
            
            // 创建远程线程执行
            unsafe {
                let thread_handle = CreateRemoteThread(
                    h_process,
                    None,
                    0,
                    Some(std::mem::transmute(self.blank_addr)),
                    None,
                    0,
                    None,
                )?;
                
                if thread_handle.is_invalid() {
                    return Err(anyhow::anyhow!("创建远程线程失败"));
                }
                
                // 等待线程执行完成
                WaitForSingleObject(thread_handle, INFINITE);
                
                // 读取返回值
                let result_bytes = self.memory.read_bytes(result_addr, 4)?;
                let result = i32::from_le_bytes([result_bytes[0], result_bytes[1], result_bytes[2], result_bytes[3]]);
                
                debug!("远程 CALL 完成，返回值：{}", result);
                Ok(result)
            }
        }
        
        #[cfg(not(windows))]
        {
            log::warn!("非 Windows 平台，返回模拟值");
            Ok(0)
        }
    }
    
    /// 技能 CALL
    pub fn skill_call(&self, skill_id: i32, target_x: i32, target_y: i32, target_z: i32) -> Result<()> {
        info!("释放技能：ID={}, 坐标=({}, {}, {})", skill_id, target_x, target_y, target_z);
        
        use crate::game::address::Address;
        let skill_call_addr = Address::SKILL_CALL_ADDR as usize;
        
        let params = vec![
            skill_id as usize,
            target_x as usize,
            target_y as usize,
            target_z as usize,
        ];
        
        self.remote_call(skill_call_addr, &params)?;
        
        Ok(())
    }
    
    /// 移动 CALL
    pub fn move_call(&self, x: i32, y: i32, z: i32) -> Result<()> {
        info!("移动到坐标：({}, {}, {})", x, y, z);
        
        use crate::game::address::Address;
        let move_call_addr = Address::MOVE_CALL_ADDR as usize;
        
        let params = vec![
            x as usize,
            y as usize,
            z as usize,
        ];
        
        self.remote_call(move_call_addr, &params)?;
        
        Ok(())
    }
    
    /// 过图 CALL
    pub fn pass_map_call(&self, direction: i32) -> Result<()> {
        info!("过图，方向：{}", direction);
        
        use crate::game::address::Address;
        let pass_map_addr = Address::PASS_MAP_CALL_ADDR as usize;
        
        let params = vec![direction as usize];
        self.remote_call(pass_map_addr, &params)?;
        
        Ok(())
    }
    
    /// 漂移过图
    pub fn drift_pass_map(&self) -> Result<()> {
        info!("执行漂移过图");
        
        use crate::game::address::Address;
        let drift_call_addr = Address::DRIFT_PASS_MAP_ADDR as usize;
        self.remote_call(drift_call_addr, &[])?;
        
        Ok(())
    }
    
    /// 进图 CALL
    pub fn enter_map_call(&self, map_id: i32, difficulty: i32) -> Result<()> {
        info!("进入地图：ID={}, 难度={}", map_id, difficulty);
        
        use crate::game::address::Address;
        let enter_map_addr = Address::ENTER_MAP_CALL_ADDR as usize;
        
        let params = vec![
            map_id as usize,
            difficulty as usize,
        ];
        
        self.remote_call(enter_map_addr, &params)?;
        
        Ok(())
    }
    
    /// 接受任务 CALL
    pub fn accept_task_call(&self, task_id: i32) -> Result<()> {
        info!("接受任务：ID={}", task_id);
        
        use crate::game::address::Address;
        let accept_task_addr = Address::ACCEPT_TASK_CALL_ADDR as usize;
        
        let params = vec![task_id as usize];
        self.remote_call(accept_task_addr, &params)?;
        
        Ok(())
    }
    
    /// 提交任务 CALL
    pub fn submit_task_call(&self, task_id: i32) -> Result<()> {
        info!("提交任务：ID={}", task_id);
        
        use crate::game::address::Address;
        let submit_task_addr = Address::SUBMIT_TASK_CALL_ADDR as usize;
        
        let params = vec![task_id as usize];
        self.remote_call(submit_task_addr, &params)?;
        
        Ok(())
    }
    
    /// 完成任务 CALL
    pub fn finish_task_call(&self, task_id: i32) -> Result<()> {
        info!("完成任务：ID={}", task_id);
        
        use crate::game::address::Address;
        let finish_task_addr = Address::FINISH_TASK_CALL_ADDR as usize;
        
        let params = vec![task_id as usize];
        self.remote_call(finish_task_addr, &params)?;
        
        Ok(())
    }
    
    /// 城镇瞬移
    pub fn town_teleport(&self, npc_id: i32) -> Result<()> {
        info!("城镇瞬移到 NPC: ID={}", npc_id);
        
        use crate::game::address::Address;
        let teleport_addr = Address::TOWN_TELEPORT_ADDR as usize;
        
        let params = vec![npc_id as usize];
        self.remote_call(teleport_addr, &params)?;
        
        Ok(())
    }
    
    /// 普通攻击
    pub fn normal_attack(&self, target_id: i32) -> Result<()> {
        info!("普通攻击，目标 ID: {}", target_id);
        
        use crate::game::address::Address;
        let attack_addr = Address::NORMAL_ATTACK_ADDR as usize;
        
        let params = vec![target_id as usize];
        self.remote_call(attack_addr, &params)?;
        
        Ok(())
    }
    
    /// 捡取物品
    pub fn pick_item(&self, item_id: i32) -> Result<()> {
        info!("捡取物品，ID: {}", item_id);
        
        use crate::game::address::Address;
        let pick_addr = Address::PICK_ITEM_ADDR as usize;
        
        let params = vec![item_id as usize];
        self.remote_call(pick_addr, &params)?;
        
        Ok(())
    }
    
    /// 清理资源
    pub fn cleanup(&mut self) -> Result<()> {
        if self.initialized && self.blank_addr > 0 {
            info!("清理 CALL 系统资源");
            self.memory.free(self.blank_addr)?;
            self.blank_addr = 0;
            self.initialized = false;
        }
        Ok(())
    }
}

impl<M: ReadWriteMemory> Drop for CallSystem<M> {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::driver::memory::Memory;
    
    #[test]
    #[ignore]
    fn test_call_system_init() {
        // 需要实际运行环境
        let memory = Memory::new();
        let mut call_system = CallSystem::new(memory);
        assert!(!call_system.initialized);
    }
}
