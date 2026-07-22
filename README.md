# AUTO PASTE

一款基于 [Tauri](https://tauri.app/) 的桌面端账号管理工具，用于本地保存账号信息，并支持一键复制用户名或密码到剪贴板，方便日常登录与切换账号。

当前版本：**1.1.2**

## 功能特性

### 账号管理

- **增删改查**：添加、编辑、删除账号，支持名称、用户名、密码、备注、优先级等字段
- **一键复制**：点击账号卡片上的用户名或密码按钮，即可复制到系统剪贴板
- **收藏标记**：为常用账号添加「喜欢」标记，并支持按收藏状态筛选
- **拖拽排序**：在「全部」分组且无搜索/筛选条件下，可拖拽调整账号显示顺序
- **敏感信息脱敏**：默认隐藏用户名中间段与密码明文，可在功能面板中切换显示

### 分组管理

- **自定义分组**：创建、修改、删除分组，将账号归类到不同分组
- **分组筛选**：通过顶部分组标签页快速切换查看不同分组下的账号
- **右键菜单**：在分组标签上右键可进行修改或删除操作

### 搜索与筛选

- **关键词搜索**：支持按「全部」「名称」「账号」三种维度模糊搜索
- **收藏筛选**：循环切换「全部 / 仅收藏 / 仅未收藏」三种模式
- **分组筛选**：结合分组标签页过滤账号列表

### 窗口与界面

- **窗口置顶**：一键将窗口置于最前，方便在多窗口环境下使用
- **窗口大小**：支持自定义窗口尺寸，设置会持久化保存；可一键恢复默认大小
- **暗色主题**：默认使用 Vuetify 暗色主题
- **紧凑布局**：默认窗口宽度 460px，适合作为侧边辅助工具使用

## 技术栈

| 层级 | 技术 |
|------|------|
| 桌面框架 | [Tauri](https://tauri.app/) 1.4 |
| 前端框架 | [Vue 3](https://vuejs.org/) + [TypeScript](https://www.typescriptlang.org/) |
| UI 组件库 | [Vuetify 3](https://vuetifyjs.com/) |
| 构建工具 | [Vite 4](https://vitejs.dev/) |
| 数据库 | [SQLite](https://www.sqlite.org/)（通过 `rusqlite`） |
| 后端语言 | [Rust](https://www.rust-lang.org/) |

其他依赖：`vuedraggable`（拖拽排序）、`vuetify-use-dialog`（确认对话框与提示）、`@mdi/font`（图标）。

## 环境要求

开始之前，请确保本机已安装：

- [Node.js](https://nodejs.org/)（建议 LTS 版本）
- [Rust](https://www.rust-lang.org/tools/install)（通过 `rustup` 安装）
- 各平台 Tauri 构建依赖，详见 [Tauri 官方前置条件文档](https://tauri.app/v1/guides/getting-started/prerequisites)

| 平台 | 额外依赖 |
|------|----------|
| macOS | Xcode Command Line Tools |
| Windows | Microsoft C++ Build Tools、WebView2 |
| Linux | `webkit2gtk`、`libayatana-appindicator` 等（因发行版而异） |

## 快速开始

### 1. 克隆仓库

```bash
git clone git@github.com:hidden-lii/auto-paste.git
cd auto-paste
```

### 2. 安装依赖

```bash
npm install
```

首次运行 `tauri dev` 或 `tauri build` 时，Rust 会自动拉取并编译后端依赖，可能需要几分钟。

### 3. 启动开发环境

```bash
npm run tauri dev
```

该命令会同时启动 Vite 开发服务器（端口 `1420`）和 Tauri 桌面窗口。

### 4. 构建发布包

```bash
npm run tauri build
```

构建产物位于 `src-tauri/target/release/bundle/` 目录，按平台生成 `.dmg`、`.msi`、`.AppImage` 等安装包。

## 常用命令

| 命令 | 说明 |
|------|------|
| `npm run dev` | 仅启动前端 Vite 开发服务器 |
| `npm run build` | 仅构建前端静态资源到 `dist/` |
| `npm run tauri dev` | 启动 Tauri 开发模式（推荐） |
| `npm run tauri build` | 构建桌面应用安装包 |
| `npm run preview` | 预览前端构建结果 |

## 项目结构

```
auto-paste/
├── src/                          # Vue 前端源码
│   ├── api/                      # Tauri invoke 封装（account / category / window）
│   ├── components/               # UI 组件
│   │   ├── AccountCard.vue       # 账号卡片
│   │   ├── AccountCardList.vue   # 账号列表（含拖拽）
│   │   ├── AccountFormDialog.vue # 账号新增/编辑对话框
│   │   ├── AccountSearchBar.vue  # 搜索栏
│   │   ├── AppFooterToolbar.vue  # 底部工具栏
│   │   ├── AppFunctionPanel.vue  # 功能面板（脱敏、窗口大小）
│   │   ├── CategoryAccountPanel.vue
│   │   ├── CategoryFormDialog.vue
│   │   ├── CategoryTabBar.vue    # 分组标签页
│   │   └── WindowSizeDialog.vue  # 窗口大小设置
│   ├── composables/              # 组合式函数
│   ├── entity/                   # 前端数据模型
│   ├── pages/
│   │   └── AccountPage.vue       # 主页面
│   └── utils/                    # 工具函数（脱敏、窗口、反馈提示）
├── src-tauri/                    # Rust 后端
│   ├── src/
│   │   ├── main.rs               # Tauri 入口与 Command 定义
│   │   ├── sqlite.rs             # SQLite 数据访问层
│   │   └── entity/               # Rust 数据模型
│   ├── tauri.conf.json           # Tauri 配置
│   └── tauri.macos.conf.json     # macOS 平台覆盖配置
├── index.html
├── package.json
└── vite.config.ts
```

## 数据存储

应用使用本地 SQLite 数据库 `auto_paste.db` 存储所有数据，数据库文件在应用运行时创建于当前工作目录。

### 数据表

| 表名 | 说明 |
|------|------|
| `account` | 账号信息（名称、用户名、密码、优先级、收藏状态、备注等） |
| `category` | 分组信息 |
| `account_category` | 账号与分组的多对多关联 |
| `app_setting` | 应用设置（如窗口宽高） |

> **注意**：账号密码以明文存储在本地数据库中，请仅在可信任的个人设备上使用，并注意备份与权限管理。

### 窗口大小持久化

窗口尺寸保存在 `app_setting` 表中。应用启动时会自动读取并应用上次保存的尺寸；若从未设置，则使用平台默认值（宽 460px，macOS 高 761px，其他平台高 732px）。

## Tauri Commands

前端通过 `@tauri-apps/api` 的 `invoke` 调用以下 Rust 命令：

| Command | 说明 |
|---------|------|
| `query_all_accounts` | 查询全部账号 |
| `query_accounts_by_value` | 按条件搜索账号 |
| `insert_account` | 新增账号 |
| `update_account` | 更新账号 |
| `delete_account` | 删除账号 |
| `update_like` | 更新收藏状态 |
| `reorder_accounts` | 批量更新账号排序 |
| `query_all_category` | 查询全部分组 |
| `create_category` | 新增分组 |
| `update_category` | 更新分组 |
| `delete_category_by_id` | 删除分组 |
| `reorder_categories` | 批量更新分组排序 |
| `get_saved_window_size` | 获取已保存的窗口尺寸 |
| `save_window_size` | 保存窗口尺寸 |
| `get_default_window_size` | 获取默认窗口尺寸 |

## 使用说明

### 复制账号信息

1. 在账号卡片上点击**用户名**或**密码**按钮
2. 内容会自动写入系统剪贴板，并弹出「复制成功」提示
3. 切换到目标应用后粘贴即可

### 管理分组

1. 点击分组标签栏右侧的 `+` 按钮创建新分组
2. 在分组标签上**右键**可修改或删除分组
3. 在账号/分组编辑对话框中可关联账号与分组

### 拖拽排序

仅在同时满足以下条件时可拖拽排序：

- 当前选中「全部」分组
- 无搜索关键词
- 收藏筛选处于「全部」模式

### 底部工具栏

| 按钮 | 功能 |
|------|------|
| 添加账号 | 打开新增账号对话框 |
| 心形图标 | 循环切换收藏筛选（全部 → 仅收藏 → 仅未收藏） |
| 图钉图标 | 切换窗口置顶 |
| 刷新 | 重置搜索与筛选条件并重新加载数据 |

### 功能面板（左下角）

点击 `⊞` 图标展开功能菜单：

- 显示/隐藏完整用户名
- 显示/隐藏明文密码
- 设置窗口大小（支持恢复默认）

## 平台支持

支持 macOS、Windows、Linux 三大桌面平台。macOS 构建使用独立的 `tauri.macos.conf.json` 配置文件（默认窗口高度为 761px）。

Bundle Identifier：`aki.auto-paste`

## 开发说明

- 前端开发服务器固定使用端口 **1420**（`strictPort: true`）
- 应用启动时禁用右键菜单（`App.vue`）
- 旧版窗口尺寸若保存在 `localStorage`（键名 `window-size`），首次启动会自动迁移至数据库
- TypeScript 类型检查：`vue-tsc --noEmit`（已集成在 `npm run build` 中）

### 推荐 VS Code 扩展

项目已配置 `.vscode/extensions.json`，推荐安装：

- [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar)

## 许可证

本项目基于 [MIT License](LICENSE) 开源，Copyright (c) 2023 Hidden_Lii。
