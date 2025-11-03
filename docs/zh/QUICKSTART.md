# NeuroNexus - 快速入门指南

> 🌐 **[English](../../QUICKSTART.md)** | **[Português](../pt/QUICKSTART.md)**

## 🚀 快速入门

### 先决条件
- 已安装 Rust 1.75+
- 已安装 Cargo

### 运行应用程序

```bash
# 编译并运行
cargo run --bin app

# 或仅编译
cargo build --bin app

# 发布版本构建
cargo build --bin app --release
```

应用程序将在 `http://localhost:8080`（或 Dioxus 配置的端口）提供服务。

## 📦 项目结构

```
NeuroNexus/
├── CODEX.md              # 完整项目规范
├── README.md             # 概述
├── SEEDERS.md            # 种子数据文档
├── STATUS.md             # 当前开发状态
├── QUICKSTART.md         # 本文件
├── Cargo.toml            # 工作空间配置
└── crates/
    ├── domain/           # 业务逻辑和模型
    ├── data/             # 存储库和种子数据
    ├── app/              # Dioxus 界面
    ├── shared/           # 共享工具
    └── services/         # 外部服务（未来）
```

## 🎨 已实现功能

### 赛博朋克霓虹界面
- ✅ 霓虹色暗黑主题（紫色、粉色、蓝色、金色）
- ✅ 发光效果和阴影
- ✅ 样式化组件（按钮、卡片、输入）
- ✅ 选项卡导航

### 功能
- ✅ 学习计划仪表板
- ✅ 问题列表（11 个真实问题）
- ✅ 作文列表（4 篇作文）
- ✅ 用户资料
- ✅ 路由系统

### 测试数据
- ✅ 11 个来自多个科目的真实问题
- ✅ 4 篇带反馈的作文
- ✅ 3 条知识路径
- ✅ 已配置测试用户

## 🔧 有用的命令

```bash
# 检查代码
cargo check

# 格式化代码
cargo fmt

# 代码检查
cargo clippy

# 测试（实现后）
cargo test

# 清理构建
cargo clean
```

## 📝 即将推出的功能

- 作文编辑器
- 详细问题查看
- 功能性搜索系统
- 实时存储库集成
- AI 导师聊天（第三阶段）
- 成就系统

## 🐛 已知问题

- NeonInput 尚未完全捕获输入事件（占位符）
- 一些生命周期警告（非关键）

## 📚 文档

- `CODEX.md` - 完整规范
- `SEEDERS.md` - 测试数据详情
- `STATUS.md` - 当前开发状态
