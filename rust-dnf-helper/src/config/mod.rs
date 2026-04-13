//! 配置模块
//! 
//! 负责加载和管理配置文件 (helper.ini)

use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;

/// 自动配置结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoConfig {
    /// 技能代码
    pub skill_code: i32,
    /// 技能伤害
    pub skill_damage: i32,
    /// 技能大小
    pub skill_size: i32,
    /// 技能个数
    pub skill_count: i32,
    /// 自动模式：1=剧情 2=搬砖
    pub auto_mode: i32,
    /// 普通地图 ID 列表
    pub normal_maps: Vec<i32>,
    /// 地图难度：0=普通 1=冒险 2=勇士 3=王者 4=噩梦 5=自动取最高
    pub map_difficulty: i32,
    /// 角色数量
    pub role_count: i32,
    /// 跟随打怪：0=关闭 1=跟随 2=跟随打怪 3=技能 call
    pub follow_monster: i32,
    /// 过图方式：0=关闭 1=强制 2=漂移
    pub pass_map_mode: i32,
    /// 处理装备：0=关闭 1=分解
    pub handle_equipment: i32,
    /// 开启功能：0=关闭 1=金身 2=装备 buff 3=满技能
    pub enable_function: i32,
    /// 过滤物品列表
    pub filter_items: Vec<String>,
    /// 出图方式：0=正常 1=快速
    pub exit_map_mode: i32,
}

impl Default for AutoConfig {
    fn default() -> Self {
        Self {
            skill_code: 70231,
            skill_damage: 50123641,
            skill_size: 1,
            skill_count: 1,
            auto_mode: 2,
            normal_maps: vec![
                100002964, 100002965, 100002950, 100002952,
                100002962, 100002705, 100002676, 400001565,
            ],
            map_difficulty: 5,
            role_count: 3,
            follow_monster: 1,
            pass_map_mode: 1,
            handle_equipment: 0,
            enable_function: 0,
            filter_items: vec![],
            exit_map_mode: 1,
        }
    }
}

/// 主配置结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// 自动配置
    pub auto_config: AutoConfig,
    /// 配置文件路径
    #[serde(skip)]
    pub config_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            auto_config: AutoConfig::default(),
            config_path: PathBuf::from("helper.ini"),
        }
    }
}

impl Config {
    /// 加载配置文件
    pub fn load() -> Result<Self> {
        let config_path = PathBuf::from("helper.ini");
        
        if !config_path.exists() {
            log::warn!("配置文件不存在，使用默认配置");
            return Ok(Self::default());
        }
        
        // 解析 INI 文件
        let ini_content = fs::read_to_string(&config_path)
            .context("读取配置文件失败")?;
        
        let mut config = Self::parse_ini(&ini_content)?;
        config.config_path = config_path;
        
        Ok(config)
    }
    
    /// 保存配置文件
    pub fn save(&self) -> Result<()> {
        let ini_content = self.to_ini()?;
        fs::write(&self.config_path, ini_content)
            .context("保存配置文件失败")?;
        Ok(())
    }
    
    /// 解析 INI 内容
    fn parse_ini(content: &str) -> Result<Self> {
        let mut auto_config = AutoConfig::default();
        let mut current_section = String::new();
        
        for line in content.lines() {
            let line = line.trim();
            
            // 跳过空行和注释
            if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
                continue;
            }
            
            // 检测节
            if line.starts_with('[') && line.ends_with(']') {
                current_section = line[1..line.len()-1].to_string();
                continue;
            }
            
            // 解析键值对
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();
                
                if current_section == "自动配置" {
                    match key {
                        "技能代码" => auto_config.skill_code = value.parse().unwrap_or(auto_config.skill_code),
                        "技能伤害" => auto_config.skill_damage = value.parse().unwrap_or(auto_config.skill_damage),
                        "技能大小" => auto_config.skill_size = value.parse().unwrap_or(auto_config.skill_size),
                        "技能个数" => auto_config.skill_count = value.parse().unwrap_or(auto_config.skill_count),
                        "自动模式" => auto_config.auto_mode = value.parse().unwrap_or(auto_config.auto_mode),
                        "普通地图" => {
                            auto_config.normal_maps = value
                                .split(',')
                                .filter_map(|s| s.trim().parse().ok())
                                .collect();
                        },
                        "地图难度" => auto_config.map_difficulty = value.parse().unwrap_or(auto_config.map_difficulty),
                        "角色数量" => auto_config.role_count = value.parse().unwrap_or(auto_config.role_count),
                        "跟随打怪" => auto_config.follow_monster = value.parse().unwrap_or(auto_config.follow_monster),
                        "过图方式" => auto_config.pass_map_mode = value.parse().unwrap_or(auto_config.pass_map_mode),
                        "处理装备" => auto_config.handle_equipment = value.parse().unwrap_or(auto_config.handle_equipment),
                        "开启功能" => auto_config.enable_function = value.parse().unwrap_or(auto_config.enable_function),
                        "过滤物品" => {
                            auto_config.filter_items = value
                                .split(',')
                                .map(|s| s.trim().to_string())
                                .collect();
                        },
                        "出图方式" => auto_config.exit_map_mode = value.parse().unwrap_or(auto_config.exit_map_mode),
                        _ => {}
                    }
                }
            }
        }
        
        Ok(Self {
            auto_config,
            config_path: PathBuf::new(),
        })
    }
    
    /// 转换为 INI 格式字符串
    fn to_ini(&self) -> Result<String> {
        let mut output = String::new();
        
        output.push_str("[自动配置]\n");
        output.push_str("; ------------------技能全屏----------------------------\n");
        output.push_str(&format!("技能代码 = {}\n", self.auto_config.skill_code));
        output.push_str(&format!("技能伤害 = {}\n", self.auto_config.skill_damage));
        output.push_str(&format!("技能大小 = {}\n", self.auto_config.skill_size));
        output.push_str(&format!("技能个数 = {}\n", self.auto_config.skill_count));
        output.push_str("; ------------------自动模式 1=剧情 2=搬砖------------------\n");
        output.push_str(&format!("自动模式 = {}\n", self.auto_config.auto_mode));
        output.push_str("; ------------------地图编号------------------\n");
        output.push_str(&format!("普通地图 = {}\n", 
            self.auto_config.normal_maps.iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(",")));
        output.push_str("; ------------------地图难度 0=普通 1=冒险 2=勇士 3=王者 4=噩梦 5=自动取最高副本等级------------------\n");
        output.push_str(&format!("地图难度 = {}\n", self.auto_config.map_difficulty));
        output.push_str("; ------------------角色数量 例如刷 10 个角色填写 9，因为默认第一个角色不计入其中------------------\n");
        output.push_str(&format!("角色数量 = {}\n", self.auto_config.role_count));
        output.push_str("; ------------------跟随打怪 0=关闭 1=跟随 2=跟随打怪 3=技能 call------------------\n");
        output.push_str(&format!("跟随打怪 = {}\n", self.auto_config.follow_monster));
        output.push_str("; ------------------过图方式 0=关闭 1=强制 2=漂移------------------\n");
        output.push_str(&format!("过图方式 = {}\n", self.auto_config.pass_map_mode));
        output.push_str("; ------------------处理装备 0=关闭 1=分解------------------\n");
        output.push_str(&format!("处理装备 = {}\n", self.auto_config.handle_equipment));
        output.push_str("; ------------------开启功能 0=关闭 1=金身 2=装备 buff 3=满技能------------------\n");
        output.push_str(&format!("开启功能 = {}\n", self.auto_config.enable_function));
        output.push_str("; ------------------过滤物品------------------\n");
        output.push_str(&format!("过滤物品 = {}\n", 
            self.auto_config.filter_items.join(",")));
        output.push_str("; ------------------出图方式 0 正常出图 1 快速出图------------------\n");
        output.push_str(&format!("出图方式 = {}\n", self.auto_config.exit_map_mode));
        
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_ini() {
        let ini_content = r#"
[自动配置]
技能代码 = 70231
自动模式 = 2
"#;
        let config = Config::parse_ini(ini_content).unwrap();
        assert_eq!(config.auto_config.skill_code, 70231);
        assert_eq!(config.auto_config.auto_mode, 2);
    }
}
