# DNF Helper - Rust 重构版本

[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows-lightgrey.svg)]()

## 项目简介

这是基于 Java 版本的 DNF 游戏辅助工具的 Rust 重构实现。使用 Rust 语言重写，提供更好的性能、内存安全性和更小的二进制体积。

## ⚠️ 重要声明

**本项目仅供学习交流使用，请勿用于任何违法违规用途！**

- 使用本软件可能导致游戏账号被封禁
- 在某些地区使用游戏辅助工具可能违反法律法规
- 开发者不对使用本软件产生的任何后果负责
- 请在合法合规的前提下学习和研究相关技术

## 功能特性

### ✅ 已实现功能 (v1.0 完整版)

#### 核心系统
- ✅ **配置管理**: INI 文件完整解析，支持技能代码、自动模式、地图配置等 20+ 参数
- ✅ **进程管理**: 游戏进程查找、句柄获取、权限验证
- ✅ **内存读写**: 完整的 Windows API 实现 (ReadProcessMemory/WriteProcessMemory)
- ✅ **驱动框架**: 多驱动支持 (LTQ/TAN/API)，条件编译切换
- ✅ **全局状态**: Arc<RwLock<T>> 线程安全数据共享
- ✅ **日志系统**: env_logger 集成，支持多级日志

#### CALL 调用系统 (核心难点)
- ✅ **远程注入**: CreateRemoteThread + 虚拟内存分配
- ✅ **汇编生成**: 自动构建 x86 调用栈 (pushad/pushfd/参数/CALL/恢复)
- ✅ **返回值读取**: 远程内存结果回传机制
- ✅ **12+ 游戏 CALL**:
  - 技能释放 (普通/无色/觉醒)
  - 角色移动 (X/Y 坐标)
  - 过图切换 (左右上下)
  - 进入副本
  - 任务提交
  - 瞬移定位
  - 普通攻击
  - 物品拾取
  - 坐飞机/返回城镇
  - 关闭窗口

#### 自动刷图系统
- ✅ **状态机引擎**: 城镇→选图→进图→战斗→过图→循环
- ✅ **怪物检测**: 内存扫描 + 最近目标选择算法
- ✅ **技能序列**: 根据配置自动释放技能组合
- ✅ **疲劳值检测**: 自动判断并切换角色
- ✅ **通关检测**: 自动识别副本完成状态
- ✅ **自动过图**: 智能判断出口方向
- ✅ **启动/停止**: AtomicBool 线程安全控制

#### 热键系统
- ✅ **全局注册**: RegisterHotKey API 集成
- ✅ **隐藏窗口**: 无界面消息循环
- ✅ **回调处理**: 启动/停止/切换角色快捷键
- ✅ **默认配置**: Ctrl+F1~F4 可自定义

#### 工具模块
- ✅ **计时器**: 毫秒级延时和超时控制
- ✅ **字节操作**: 类型转换 (i32/i64/f32/bytes)
- ✅ **地址常量**: 100+ 游戏内存地址定义

## 与 Java 版本对比

| 特性 | Java 版本 | Rust 版本 |
|------|-----------|-----------|
| 运行时 | JVM (需要安装) | 原生二进制 (无需运行时) |
| 内存占用 | ~100MB+ | ~5MB |
| 启动速度 | 较慢 | 极快 |
| 内存安全 | GC 管理 | 编译期检查 |
| 执行效率 | 中等 | 高 |
| 反编译难度 | 较低 | 较高 |
| 跨平台 | 理论上支持 | Windows 优先 |

## 技术架构

```
src/
├── main.rs              # 程序入口和初始化流程
├── config/              # 配置模块
│   └── mod.rs           # INI 配置解析 (20+ 参数)
├── driver/              # 驱动模块
│   ├── mod.rs           # 驱动管理
│   ├── driver.rs        # 多驱动实现 (LTQ/TAN/API)
│   └── memory.rs        # 内存读写 (完整 Windows API)
├── entity/              # 实体模块
│   ├── mod.rs           # 数据定义
│   ├── global_data.rs   # Arc<RwLock<T>>全局状态
│   └── map_types.rs     # 地图类型和坐标系统
├── game/                # 游戏核心模块
│   ├── mod.rs           # 游戏核心
│   ├── game.rs          # 自动刷图状态机 (520 行)
│   ├── address.rs       # 100+ 内存地址常量
│   └── call_system.rs   # CALL 调用系统 (340 行远程注入)
└── helper/              # 工具模块
    ├── mod.rs           # 工具集合
    ├── process.rs       # 进程查找和管理
    ├── timer.rs         # 毫秒级计时器
    ├── bytes.rs         # 字节操作和类型转换
    └── hotkey.rs        # 全局热键监听 (RegisterHotKey)
```

**代码统计:**
- 总代码量：2,730 行 Rust
- 源文件数：17 个
- 核心模块：6 个
- 编译产物：单一可执行文件 (~2MB)

## 构建说明

### 环境要求

- Rust 1.70+
- Windows 10/11 (目标平台)
- Visual Studio Build Tools (Windows)

### 编译步骤

```bash
# 克隆项目
git clone <repository-url>
cd rust-dnf-helper

# 开发模式编译
cargo build

# 发布模式编译 (优化)
cargo build --release

# 运行测试
cargo test
```

### 输出文件

编译完成后，可执行文件位于：
- 调试版：`target/debug/dnf_helper.exe`
- 发布版：`target/release/dnf_helper.exe`

## 使用说明

1. **配置文件**: 将 `helper.ini` 放置在程序同目录
2. **驱动准备**: 确保已准备好相应的内核驱动文件
3. **运行程序**: 以管理员身份运行 `dnf_helper.exe`
4. **热键操作**: 使用配置的热键控制自动功能

## 配置示例

```ini
[自动配置]
技能代码 = 70231
技能伤害 = 50123641
自动模式 = 2
普通地图 = 100002964,100002965,100002950
地图难度 = 5
角色数量 = 3
跟随打怪 = 1
过图方式 = 1
```

## 依赖说明

主要依赖库：
- `windows`: Windows API 绑定
- `anyhow`: 错误处理
- `log` + `env_logger`: 日志系统
- `parking_lot`: 高效锁实现
- `serde`: 序列化/反序列化

## 开发计划

### ✅ v1.0.0 (已完成)
- [x] 基础框架搭建
- [x] 配置管理 (INI 解析)
- [x] 内存读写完整实现
- [x] CALL 调用系统 (远程注入)
- [x] 自动刷图状态机
- [x] 热键监听系统
- [x] 多角色切换支持
- [x] 完整文档和注释

### 🔮 v1.1.0 (规划中)
- [ ] GUI 界面 (egui/Tauri)
- [ ] 图形化配置编辑器
- [ ] 实时日志查看器
- [ ] 地图可视化显示

### 🔮 v1.2.0 (规划中)
- [ ] 智能装备管理系统
- [ ] 自动强化/增幅
- [ ] 仓库整理功能
- [ ] 金币交易处理

### 🔮 v2.0.0 (愿景)
- [ ] 多游戏支持架构
- [ ] 插件系统设计
- [ ] 云端配置同步
- [ ] AI 智能决策

## 贡献指南

欢迎提交 Issue 和 Pull Request！

1. Fork 本项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 致谢

- 感谢原 Java 版本的作者
- 感谢 Rust 社区的优秀库
- 感谢所有贡献者

## 联系方式

如有问题或建议，请通过 Issue 联系。

---

**再次提醒：本项目仅供学习研究，请勿用于非法用途！**
