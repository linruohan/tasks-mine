# Tasks Mine - 工具集

一个基于 GPUI 框架和 gpui-component 组件库开发的桌面工具应用，用于管理研发流程中的各类任务和数据。

## 功能模块

### 1. CodeHub - MR 管理
- ✅ 管理 Merge Request（增删查）
- ✅ 统计周期内 MR 的工作量
- ✅ 显示新增行数和删除行数
- ✅ 数据可视化展示

### 2. DTS - 问题单管理
- ✅ 管理问题单（增查）
- ✅ 统计周期内问题单的提交和回归数量
- ✅ 问题单状态跟踪
- ✅ 回归率计算

### 3. Excel - 数据处理
- ✅ Excel 文件读取（支持 .xlsx 和 .xls）
- ✅ Excel 文件写入（导出 .xlsx）
- ✅ 批量导入导出
- ✅ 模板下载

### 4. Hive - 平台管理
- ✅ 失败用例分析
- ✅ 虚拟环境管理（占用、释放、部署）
- ✅ 工程 Job 管理（拉起、续跑）
- ✅ 组织失败分析报告

### 5. Request - HTTP 请求
- ✅ HTTP 请求处理
- ✅ Cookie 管理和登录认证
- ✅ 获取 DTS、MR、Requirement 数据

### 6. Requirement - 需求管理
- ✅ 管理需求版本
- ✅ 测试周期管理
- ✅ 截止日期跟踪
- ✅ 需求状态监控

## 技术栈

- **UI 框架**: GPUI
- **组件库**: gpui-component
- **HTTP 请求**: reqwest
- **Excel 处理**: calamine (读取), rust_xlsxwriter (写入)
- **数据库**: SQLx (SQLite)
- **日期处理**: chrono
- **异步运行时**: tokio

## 安装依赖

确保本地已安装 Rust 工具链，然后运行：

```bash
cargo build
```

## 运行应用

```bash
cargo run
```

## 项目结构

```
tasks-mine/
├── src/
│   ├── main.rs           # 应用入口
│   ├── ui/
│   │   ├── mod.rs
│   │   └── app.rs        # 主应用 UI
│   └── tools/
│       ├── mod.rs
│       ├── codehub.rs    # CodeHub 模块
│       ├── dts.rs        # DTS 模块
│       ├── excel.rs      # Excel 模块
│       ├── hive.rs       # Hive 模块
│       ├── request.rs    # Request 模块
│       └── requirement.rs # Requirement 模块
├── Cargo.toml
└── README.md
```

## UI 特性

- 🎨 现代化的用户界面设计
- 📊 数据可视化展示
- 🔄 实时数据更新
- 📱 响应式布局
- 🎯 直观的导航系统

## 开发计划

- [ ] 添加数据持久化（SQLite）
- [ ] 实现数据导入导出功能
- [ ] 添加图表统计功能
- [ ] 完善 HTTP 请求认证
- [ ] 添加配置管理功能

## License

MIT License
