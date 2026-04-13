//! DNF Helper - Rust 重构版本
//! 
//! 基于 Rust 实现的 DNF 游戏辅助工具
//! 提供内存读写、自动刷图、技能调用等功能

mod config;
mod driver;
mod entity;
mod game;
mod helper;

use anyhow::Result;
use log::{info, error};

fn main() -> Result<()> {
    // 初始化日志
    env_logger::init();
    
    info!("DNF Helper Rust 版本启动");
    info!("========================");
    
    // 初始化配置
    let config = config::Config::load()?;
    info!("配置加载完成");
    
    // 初始化驱动
    let mut driver = driver::Driver::new();
    driver.initialize()?;
    info!("驱动初始化完成");
    
    // 查找游戏进程
    let process_id = helper::process::find_process("DNF.exe")?;
    info!("找到游戏进程，PID: {}", process_id);
    
    // 设置进程 ID
    driver.set_process_id(process_id);
    
    // 初始化游戏模块
    let mut game = game::Game::new(driver, config);
    game.initialize()?;
    
    // 启动自动刷图线程
    game.start_auto_thread();
    
    info!("DNF Helper 已就绪，按 Ctrl+C 退出");
    
    // 保持主线程运行
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
