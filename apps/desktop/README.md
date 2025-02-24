# Desktop Client

基于 Bevy 引擎和 Vulkan 的跨平台桌面客户端

## 功能特性

- 跨平台支持 (Linux, macOS, Windows)
- Vulkan 图形渲染支持
- 多种构建模式
- Node.js 集成
- 热重载开发支持

## 系统要求

- Node.js >= 16.0.0
- Rust >= 1.65.0
- Cargo
- 支持 Vulkan 的显卡驱动（可选）

## 安装

1. 克隆仓库：
```bash
git clone <repository-url>
cd desktop-client
```

2. 安装依赖：
```bash
npm install
```

## 构建模式

项目支持四种不同的构建模式，每种模式具有不同的功能集：

### 开发模式 (dev)
- 包含完整的开发工具和调试功能
- 支持热重载
- 包含资源加载器
- 包含调试工具
- 包含精灵和文本渲染
- 默认启用 Vulkan

构建和运行：
```bash
npm run build:dev    # 仅构建
npm run start:dev    # 构建并运行
npm run dev          # 开发模式（支持热重载）
```

### 调试模式 (debug)
- 包含基本的调试功能
- 精简的功能集
- 适合测试和调试

构建和运行：
```bash
npm run build:debug  # 仅构建
npm run start:debug  # 构建并运行
```

### 预发布模式 (pre)
- 包含大多数生产功能
- 支持 GLTF 模型
- 支持 PBR 渲染
- 包含场景系统
- 启用 Vulkan

构建和运行：
```bash
npm run build:pre    # 仅构建
npm run start:pre    # 构建并运行
```

### 发布模式 (release)
- 只包含核心生产功能
- 经过性能优化
- 最小化的依赖
- 启用 Vulkan

构建和运行：
```bash
npm run build:release  # 仅构建
npm run start:release  # 构建并运行
```

## 开发工具

### 代码检查
```bash
npm run lint         # 运行 ESLint 检查
npm run lint:fix     # 自动修复 ESLint 问题
```

### 测试
```bash
npm run test         # 运行所有测试
npm run test:watch   # 监视模式运行测试
```

### 清理
```bash
npm run clean        # 清理构建产物
```

## API 使用说明

### 基本用法

```typescript
import { DesktopClient, ClientConfig } from 'desktop-client';

// 创建客户端实例
const client = new DesktopClient();

// 配置选项
const config: ClientConfig = {
    width: 1280,
    height: 720,
    title: "我的应用",
    vsync: true
};

// 初始化
await client.initialize(config);

// 运行
await client.run();

// 清理资源
await client.cleanup();
```

### 检查 Vulkan 支持

```typescript
import { use_vulkan } from 'desktop-client';

if (use_vulkan()) {
    console.log('Vulkan 支持已启用');
}
```

### 获取构建模式

```typescript
import { get_build_mode } from 'desktop-client';

const mode = get_build_mode();
console.log(`当前构建模式: ${mode}`);
```

## 项目结构

```
desktop-client/
├── src/                # Rust 源代码
├── dist/               # 编译后的 JavaScript 文件
├── tests/              # 测试文件
├── Cargo.toml          # Rust 项目配置
├── package.json        # Node.js 项目配置
└── README.md           # 项目文档
```

## 注意事项

1. Vulkan 支持需要系统安装相应的驱动
2. 不同构建模式的功能集不同，请根据需求选择
3. 开发模式会占用更多系统资源
4. 发布模式已经过优化，适合生产环境

## 常见问题

1. 构建失败
   - 检查 Rust 和 Node.js 版本
   - 确保已安装所有依赖
   - 检查是否有权限问题

2. Vulkan 相关错误
   - 确认显卡驱动是否支持 Vulkan
   - 检查 Vulkan SDK 是否正确安装
   - 验证系统环境变量设置

3. 性能问题
   - 开发模式下性能较低是正常的
   - 生产环境请使用 release 模式
   - 确保系统资源充足

## 贡献指南

1. Fork 项目
2. 创建特性分支
3. 提交改动
4. 推送到分支
5. 创建 Pull Request

## 许可证

MIT
