# NeuroNexus

> 🌐 **[English](../../README.md)** | **[Português](../pt/README.md)**

一个使用 Rust 和 Dioxus 构建的跨平台教育平台（Web、桌面、移动端），专注于大学入学考试和 ENEM 备考。

## 🎨 设计

受《赛博朋克 2077》和《银翼杀手 2049》启发的赛博朋克霓虹界面，采用暗黑主题和霓虹效果。

## 🏗️ 架构

项目采用整洁架构（Clean Architecture），结构如下：

```
crates/
├── domain/     # 业务逻辑、模型、用例、特征
├── data/       # 存储库实现、数据库、种子数据
├── app/        # Dioxus 组件、页面、路由、UI
├── shared/     # 共享工具、通用类型
└── services/   # 外部服务（AI、API 等）
```

## 🚀 开发

### 先决条件

- Rust 1.75 或更高版本
- Cargo

### 运行（Web）

```bash
cargo run --bin app
```

### 构建

```bash
cargo build --release
```

## 📋 实施阶段

### ✅ 第一阶段：核心 MVP（进行中）
- [x] Rust 工作空间设置
- [x] Crate 结构
- [x] 基本领域模型
- [x] 赛博朋克 UI 组件
- [x] 基本路由
- [x] 主要页面
- [x] 内存存储库
- [ ] 测试数据种子

### 第二阶段：基本教育功能
- [ ] 作文编辑器
- [ ] 作文评估
- [ ] 问题查看
- [ ] 路径系统

### 第三阶段：AI 和个性化
- [ ] AI 导师聊天
- [ ] AI 驱动的作文评估
- [ ] 个性化学习路径
- [ ] 成就系统

## 📚 文档

完整项目文档请参阅 `CODEX.md`。

## 🎯 技术栈

- **Rust**：主要编程语言
- **Dioxus**：跨平台 UI 框架
- **Tokio**：异步运行时
- **Chrono**：日期处理
- **UUID**：唯一标识符

## 📝 许可证

MIT
