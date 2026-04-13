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

### 已实现功能

- ✅ 配置管理 (INI 文件解析)
- ✅ 进程查找和管理
- ✅ 内存读写接口
- ✅ 驱动框架 (多驱动支持)
- ✅ 全局状态管理
- ✅ 日志系统
- ✅ 时间工具
- ✅ 字节操作工具
- ✅ CALL 调用系统框架
- ✅ 热键监听模块

### 待实现功能

- ⏳ 完整的远程 CALL 实现 (Windows)
- ⏳ 自动刷图状态机完整逻辑
- ⏳ 技能释放系统
- ⏳ 地图导航系统
- ⏳ 物品拾取逻辑
- ⏳ 装备处理系统
- ⏳ GUI 界面

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
├── main.rs              # 程序入口
├── config/              # 配置模块
│   └── mod.rs           # INI 配置解析
├── driver/              # 驱动模块
│   ├── mod.rs           # 驱动管理
│   ├── driver.rs        # 驱动实现
│   └── memory.rs        # 内存读写
├── entity/              # 实体模块
│   ├── mod.rs           # 数据定义
│   ├── global_data.rs   # 全局数据
│   └── map_types.rs     # 地图类型
├── game/                # 游戏模块
│   ├── mod.rs           # 游戏核心
│   ├── game.rs          # 游戏逻辑
│   ├── address.rs       # 内存地址常量
│   └── call_system.rs   # CALL 调用系统
└── helper/              # 工具模块
    ├── mod.rs           # 工具集合
    ├── process.rs       # 进程管理
    ├── timer.rs         # 时间工具
    ├── bytes.rs         # 字节操作
    └── hotkey.rs        # 热键监听
```

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

### v0.1.0 (当前)
- [x] 基础框架搭建
- [x] 配置管理
- [x] 内存读写接口
- [ ] 基础 CALL 实现

### v0.2.0
- [ ] 完整 CALL 系统
- [ ] 自动刷图逻辑
- [ ] 地图导航

### v0.3.0
- [ ] GUI 界面
- [ ] 热键系统
- [ ] 多角色支持

### v1.0.0
- [ ] 功能完整
- [ ] 性能优化
- [ ] 文档完善

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
