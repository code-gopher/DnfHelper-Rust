//! 内存读写模块
//! 
//! 提供底层内存读写功能

use anyhow::{Result, Context};
use std::ptr;

#[cfg(windows)]
use windows::{
    Win32::Foundation::HANDLE,
    Win32::System::Memory::{
        VirtualAllocEx, VirtualFreeEx, MEM_COMMIT, MEM_RELEASE, MEM_RESERVE,
        PAGE_EXECUTE_READWRITE,
    },
    Win32::System::Threading::{
        OpenProcess, PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE,
    },
};

/// 内存操作 Trait
pub trait ReadWriteMemory {
    fn set_process_id(&mut self, process_id: u32);
    fn read_bytes(&self, address: usize, size: usize) -> Result<Vec<u8>>;
    fn write_bytes(&self, address: usize, data: &[u8]) -> Result<()>;
    fn read_i32(&self, address: usize) -> Result<i32>;
    fn write_i32(&self, address: usize, value: i32) -> Result<()>;
    fn read_i64(&self, address: usize) -> Result<i64>;
    fn write_i64(&self, address: usize, value: i64) -> Result<()>;
    fn read_f32(&self, address: usize) -> Result<f32>;
    fn write_f32(&self, address: usize, value: f32) -> Result<()>;
    fn allocate(&self, size: usize) -> Result<usize>;
    fn free(&self, address: usize) -> Result<()>;
}

/// Windows 内存实现
#[cfg(windows)]
pub struct WindowsMemory {
    process_handle: Option<HANDLE>,
    process_id: u32,
}

#[cfg(windows)]
impl WindowsMemory {
    pub fn new() -> Self {
        Self {
            process_handle: None,
            process_id: 0,
        }
    }
    
    fn get_process_handle(&self) -> Result<HANDLE> {
        self.process_handle.context("进程句柄未初始化")
    }
}

#[cfg(windows)]
impl ReadWriteMemory for WindowsMemory {
    fn set_process_id(&mut self, process_id: u32) {
        self.process_id = process_id;
        
        unsafe {
            let handle = OpenProcess(
                PROCESS_VM_OPERATION | PROCESS_VM_READ | PROCESS_VM_WRITE,
                false,
                process_id,
            );
            
            if let Ok(h) = handle {
                self.process_handle = Some(h);
            }
        }
    }
    
    fn read_bytes(&self, address: usize, size: usize) -> Result<Vec<u8>> {
        #[cfg(windows)]
        {
            use windows::Win32::System::Memory::ReadProcessMemory;
            
            let handle = self.get_process_handle()?;
            let mut buffer = vec![0u8; size];
            let mut bytes_read: usize = 0;
            
            unsafe {
                ReadProcessMemory(
                    handle,
                    address as *const _,
                    buffer.as_mut_ptr() as *mut _,
                    size,
                    Some(&mut bytes_read as *mut _),
                )?;
            }
            
            Ok(buffer)
        }
        
        #[cfg(not(windows))]
        {
            Err(anyhow::anyhow!("仅在 Windows 平台支持"))
        }
    }
    
    fn write_bytes(&self, address: usize, data: &[u8]) -> Result<()> {
        #[cfg(windows)]
        {
            use windows::Win32::System::Memory::WriteProcessMemory;
            
            let handle = self.get_process_handle()?;
            let mut bytes_written: usize = 0;
            
            unsafe {
                WriteProcessMemory(
                    handle,
                    address as *mut _,
                    data.as_ptr() as *const _,
                    data.len(),
                    Some(&mut bytes_written as *mut _),
                )?;
            }
            
            Ok(())
        }
        
        #[cfg(not(windows))]
        {
            Err(anyhow::anyhow!("仅在 Windows 平台支持"))
        }
    }
    
    fn read_i32(&self, address: usize) -> Result<i32> {
        let bytes = self.read_bytes(address, 4)?;
        Ok(i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }
    
    fn write_i32(&self, address: usize, value: i32) -> Result<()> {
        self.write_bytes(address, &value.to_le_bytes())
    }
    
    fn read_i64(&self, address: usize) -> Result<i64> {
        let bytes = self.read_bytes(address, 8)?;
        Ok(i64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }
    
    fn write_i64(&self, address: usize, value: i64) -> Result<()> {
        self.write_bytes(address, &value.to_le_bytes())
    }
    
    fn read_f32(&self, address: usize) -> Result<f32> {
        let bytes = self.read_bytes(address, 4)?;
        Ok(f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }
    
    fn write_f32(&self, address: usize, value: f32) -> Result<()> {
        self.write_bytes(address, &value.to_le_bytes())
    }
    
    fn allocate(&self, size: usize) -> Result<usize> {
        #[cfg(windows)]
        {
            let handle = self.get_process_handle()?;
            
            unsafe {
                let addr = VirtualAllocEx(
                    handle,
                    None,
                    size,
                    MEM_COMMIT | MEM_RESERVE,
                    PAGE_EXECUTE_READWRITE,
                );
                
                Ok(addr.0 as usize)
            }
        }
        
        #[cfg(not(windows))]
        {
            Err(anyhow::anyhow!("仅在 Windows 平台支持"))
        }
    }
    
    fn free(&self, address: usize) -> Result<()> {
        #[cfg(windows)]
        {
            use windows::Win32::System::Memory::VirtualFreeEx;
            
            let handle = self.get_process_handle()?;
            
            unsafe {
                VirtualFreeEx(
                    handle,
                    address as *mut _,
                    0,
                    MEM_RELEASE,
                )?;
            }
            
            Ok(())
        }
        
        #[cfg(not(windows))]
        {
            Err(anyhow::anyhow!("仅在 Windows 平台支持"))
        }
    }
}

/// Linux/Mac 占位实现 (仅用于编译测试)
#[cfg(not(windows))]
pub struct LinuxMemory {
    process_id: u32,
}

#[cfg(not(windows))]
impl LinuxMemory {
    pub fn new() -> Self {
        Self { process_id: 0 }
    }
}

#[cfg(not(windows))]
impl ReadWriteMemory for LinuxMemory {
    fn set_process_id(&mut self, process_id: u32) {
        self.process_id = process_id;
        log::warn!("Linux 平台不支持实际内存操作");
    }
    
    fn read_bytes(&self, _address: usize, _size: usize) -> Result<Vec<u8>> {
        Err(anyhow::anyhow!("Linux 平台不支持"))
    }
    
    fn write_bytes(&self, _address: usize, _data: &[u8]) -> Result<()> {
        Err(anyhow::anyhow!("Linux 平台不支持"))
    }
    
    fn read_i32(&self, _address: usize) -> Result<i32> {
        Err(anyhow::anyhow!("Linux 平台不支持"))
    }
    
    fn write_i32(&self, _address: usize, _value: i32) -> Result<()> {
        Err(anyhow::anyhow!("Linux 平台不支持"))
    }
    
    fn read_i64(&self, _address: usize) -> Result<i64> {
        Err(anyhow::anyhow!("Linux 平台不支持"))
    }
    
    fn write_i64(&self, _address: usize, _value: i64) -> Result<()> {
        Err(anyhow::anyhow!("Linux 平台不支持"))
    }
    
    fn read_f32(&self, _address: usize) -> Result<f32> {
        Err(anyhow::anyhow!("Linux 平台不支持"))
    }
    
    fn write_f32(&self, _address: usize, _value: f32) -> Result<()> {
        Err(anyhow::anyhow!("Linux 平台不支持"))
    }
    
    fn allocate(&self, _size: usize) -> Result<usize> {
        Err(anyhow::anyhow!("Linux 平台不支持"))
    }
    
    fn free(&self, _address: usize) -> Result<()> {
        Err(anyhow::anyhow!("Linux 平台不支持"))
    }
}

/// 内存类型别名
#[cfg(windows)]
pub type Memory = WindowsMemory;

#[cfg(not(windows))]
pub type Memory = LinuxMemory;
