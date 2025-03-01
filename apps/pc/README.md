# 2.5D MMORPG Game

基于 Bevy 引擎开发的 2.5D MMORPG 游戏项目。

## 项目结构

```
src/
├── bin/                    # 可执行文件
│   └── game/              # 主游戏二进制
│       ├── mod.rs         # 游戏入口模块
│       └── settings.rs    # 游戏设置
├── core/                   # 核心游戏逻辑
│   ├── mod.rs             # 核心模块定义
│   ├── game_loop.rs       # 游戏主循环
│   ├── state.rs           # 游戏状态管理
│   ├── time.rs            # 时间管理
│   ├── resource.rs        # 资源管理
│   └── constants.rs       # 游戏常量定义
├── components/            # 游戏组件定义
│   ├── mod.rs            # 组件模块定义
│   ├── player/           # 玩家相关组件
│   │   ├── mod.rs        # 玩家模块定义
│   │   ├── stats.rs      # 玩家属性
│   │   ├── skills.rs     # 技能系统
│   │   └── equipment.rs  # 装备系统
│   ├── physics/          # 物理相关组件
│   │   ├── mod.rs        # 物理模块定义
│   │   ├── collider.rs   # 碰撞体
│   │   ├── rigidbody.rs  # 刚体
│   │   └── trigger.rs    # 触发器
│   ├── animation/        # 动画相关组件
│   │   ├── mod.rs        # 动画模块定义
│   │   ├── sprite.rs     # 精灵动画
│   │   ├── skeletal.rs   # 骨骼动画
│   │   └── particle.rs   # 粒子系统
│   ├── network/          # 网络相关组件
│   │   ├── mod.rs        # 网络模块定义
│   │   ├── sync.rs       # 同步组件
│   │   └── replication.rs # 复制组件
│   ├── ai/               # AI相关组件
│   │   ├── mod.rs        # AI模块定义
│   │   ├── behavior.rs   # 行为组件
│   │   ├── pathfinding.rs # 寻路组件
│   │   └── perception.rs # 感知组件
│   └── inventory/        # 库存相关组件
│       ├── mod.rs        # 库存模块定义
│       ├── container.rs  # 容器组件
│       └── item.rs       # 物品组件
├── systems/              # 游戏系统
│   ├── mod.rs           # 系统模块定义
│   ├── render/          # 渲染系统
│   │   ├── mod.rs       # 渲染模块定义
│   │   ├── camera/      # 相机系统
│   │   │   ├── mod.rs   # 相机模块定义
│   │   │   ├── follow.rs # 相机跟随
│   │   │   └── control.rs # 相机控制
│   │   ├── sprite/      # 精灵渲染
│   │   │   ├── mod.rs   # 精灵模块定义
│   │   │   ├── batch.rs # 批处理
│   │   │   └── animation.rs # 动画
│   │   ├── effects/     # 特效系统
│   │   │   ├── mod.rs   # 特效模块定义
│   │   │   ├── particle.rs # 粒子效果
│   │   │   └── post_process.rs # 后处理
│   │   └── layers/      # 渲染层级
│   │       ├── mod.rs   # 层级模块定义
│   │       └── sorting.rs # 排序系统
│   ├── physics/         # 物理系统
│   │   ├── mod.rs       # 物理模块定义
│   │   ├── collision/   # 碰撞检测
│   │   │   ├── mod.rs   # 碰撞模块定义
│   │   │   ├── detection.rs # 碰撞检测
│   │   │   └── response.rs # 碰撞响应
│   │   └── movement/    # 移动系统
│   │       ├── mod.rs   # 移动模块定义
│   │       ├── character.rs # 角色移动
│   │       └── platformer.rs # 平台移动
│   ├── network/         # 网络系统
│   │   ├── mod.rs       # 网络模块定义
│   │   ├── client/      # 客户端网络
│   │   │   ├── mod.rs   # 客户端模块定义
│   │   │   └── connection.rs # 连接管理
│   │   └── sync/        # 状态同步
│   │       ├── mod.rs   # 同步模块定义
│   │       └── interpolation.rs # 插值系统
│   ├── input/           # 输入系统
│   │   ├── mod.rs       # 输入模块定义
│   │   ├── keyboard/    # 键盘输入
│   │   │   ├── mod.rs   # 键盘模块定义
│   │   │   └── mapping.rs # 按键映射
│   │   └── mouse/       # 鼠标输入
│   │       ├── mod.rs   # 鼠标模块定义
│   │       └── camera_ray.rs # 射线检测
│   ├── audio/           # 音频系统
│   │   ├── mod.rs       # 音频模块定义
│   │   ├── music/       # 音乐系统
│   │   │   ├── mod.rs   # 音乐模块定义
│   │   │   └── playlist.rs # 播放列表
│   │   └── sfx/         # 音效系统
│   │       ├── mod.rs   # 音效模块定义
│   │       └── pool.rs  # 音效池
│   └── ai/             # AI系统
│       ├── mod.rs       # AI模块定义
│       ├── pathfinding/ # 寻路系统
│       │   ├── mod.rs   # 寻路模块定义
│       │   ├── astar.rs # A*算法
│       │   └── navmesh.rs # 导航网格
│       └── behavior/    # 行为系统
│           ├── mod.rs   # 行为模块定义
│           └── tree.rs  # 行为树
├── assets/             # 游戏资源
│   ├── textures/      # 纹理资源
│   │   ├── characters/ # 角色贴图
│   │   ├── ui/        # UI贴图
│   │   ├── effects/   # 特效贴图
│   │   └── environment/ # 环境贴图
│   ├── models/        # 3D模型
│   │   ├── characters/ # 角色模型
│   │   ├── props/     # 道具模型
│   │   └── environment/ # 环境模型
│   ├── audio/         # 音频资源
│   │   ├── music/     # 音乐文件
│   │   ├── sfx/       # 音效文件
│   │   └── voice/     # 语音文件
│   ├── shaders/       # 着色器
│   │   ├── post/      # 后处理着色器
│   │   ├── particle/  # 粒子着色器
│   │   └── material/  # 材质着色器
│   └── config/        # 配置文件
│       ├── game/      # 游戏配置
│       └── debug/     # 调试配置
├── scenes/            # 场景定义
│   ├── mod.rs         # 场景模块定义
│   ├── loading/       # 加载场景
│   │   ├── mod.rs     # 加载模块定义
│   │   └── progress.rs # 进度管理
│   ├── menu/          # 菜单场景
│   │   ├── mod.rs     # 菜单模块定义
│   │   ├── main.rs    # 主菜单
│   │   └── pause.rs   # 暂停菜单
│   └── game/          # 游戏场景
│       ├── mod.rs     # 游戏场景定义
│       ├── world.rs   # 世界场景
│       └── instance.rs # 副本场景
├── utils/             # 工具函数
│   ├── mod.rs         # 工具模块定义
│   ├── math/          # 数学工具
│   │   ├── mod.rs     # 数学模块定义
│   │   └── vector.rs  # 向量运算
│   └── debug/         # 调试工具
│       ├── mod.rs     # 调试模块定义
│       └── logger.rs  # 日志工具
├── plugins/           # 插件系统
│   ├── mod.rs         # 插件模块定义
│   ├── game/          # 游戏核心插件
│   │   ├── mod.rs     # 游戏插件定义
│   │   └── states.rs  # 状态管理
│   ├── ui/            # UI插件
│   │   ├── mod.rs     # UI插件定义
│   │   └── layout.rs  # 布局系统
│   └── world/         # 世界管理插件
│       ├── mod.rs     # 世界插件定义
│       └── chunk.rs   # 区块管理
├── role/              # 角色系统
│   ├── mod.rs         # 角色模块定义
│   ├── player/        # 玩家角色
│   │   ├── mod.rs     # 玩家模块定义
│   │   └── class.rs   # 职业系统
│   ├── npc/           # NPC角色
│   │   ├── mod.rs     # NPC模块定义
│   │   └── dialogue.rs # 对话系统
│   └── monster/       # 怪物角色
│       ├── mod.rs     # 怪物模块定义
│       └── ai.rs      # 怪物AI
├── prefabs/           # 预制体
│   ├── mod.rs         # 预制体模块定义
│   ├── characters/    # 角色预制体
│   │   ├── mod.rs     # 角色预制体定义
│   │   └── templates/ # 角色模板
│   ├── items/         # 物品预制体
│   │   ├── mod.rs     # 物品预制体定义
│   │   └── templates/ # 物品模板
│   └── effects/       # 特效预制体
│       ├── mod.rs     # 特效预制体定义
│       └── templates/ # 特效模板
├── ui/                # 用户界面
│   ├── mod.rs         # UI模块定义
│   ├── hud/           # 游戏HUD
│   │   ├── mod.rs     # HUD模块定义
│   │   ├── health.rs  # 生命值显示
│   │   └── minimap.rs # 小地图
│   ├── menu/          # 菜单界面
│   │   ├── mod.rs     # 菜单模块定义
│   │   └── components/ # 菜单组件
│   └── widgets/       # 通用UI组件
│       ├── mod.rs     # 组件模块定义
│       ├── button.rs  # 按钮组件
│       └── panel.rs   # 面板组件
└── config/            # 配置文件
    ├── mod.rs         # 配置模块定义
    ├── game/          # 游戏配置
    │   ├── mod.rs     # 游戏配置定义
    │   └── balance.rs # 平衡性配置
    └── debug/         # 调试配置
        ├── mod.rs     # 调试配置定义
        └── logging.rs # 日志配置
```

## 开发环境设置

1. 安装 Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. 安装依赖
```bash
# 安装系统依赖（Ubuntu/Debian）
sudo apt-get install pkg-config libx11-dev libasound2-dev libudev-dev

# 安装系统依赖（macOS）
brew install pkg-config
```

3. 克隆项目
```bash
git clone <project-url>
cd mmorpg-game
```

4. 运行项目
```bash
cargo run
```

## 开发指南

### 添加新功能

1. 组件开发
- 在 `src/components` 中定义新组件
- 确保实现必要的特性 (Component, Debug, etc.)

2. 系统开发
- 在 `src/systems` 中添加新系统
- 遵循 Bevy ECS 模式
- 确保系统正确注册到相应的插件中

3. 资源管理
- 游戏资源放在 `assets` 目录
- 使用 `bevy_asset_loader` 进行资源加载

### 性能优化

1. 使用 Bevy 的性能优化特性
- 启用 dynamic_linking 加快开发编译
- 使用 release 模式运行性能测试

2. 系统优化
- 合理使用系统调度
- 避免不必要的组件查询

### 调试

1. 使用 Bevy 调试工具
```rust
app.add_plugins(bevy_inspector_egui::WorldInspectorPlugin::new());
```

2. 日志系统
- 使用 `log` crate 进行日志记录
- 在开发模式下启用详细日志

## 贡献指南

1. 代码风格
- 遵循 Rust 标准代码风格
- 使用 `rustfmt` 格式化代码
- 使用 `clippy` 进行代码检查

2. 提交规范
- 使用清晰的提交信息
- 遵循语义化版本控制

## 许可证

MIT License 


@src 目前看下来，要启动这个 MMORPG，需要提前处理的是，
0. 目录需要规范化，代码要标注是哪个文件，为什么这么写。如果有已经实现的，就要考虑删除原来实现的地方，尤其是 Config/mod.rs
1. 全局配置资源、资源管理器、状态管理器、事件通道
2. 全局资源中，应该包含键位绑定和输入控制，目前只写 window、network、input 的 events 就行
3. app.add_plugins(DefaultPlugins); 是怎么做到不设置window 就能用的？
4.  下面这些可以写到 trait 里吗？
       app.add_systems(Update, (
            handle_input,
            handle_window_events,
            handle_network_events,
        ));


上面的这些要添加的，应该放到一个新的目录里吧？目前的目录像 plugins 等应该是不适合的，是放到 system 里，还是放到 events 里会好些？还是哪里？




@src 我现在重新进行了拆分，先完成我说的这些吧

1. @plugins 现在这种拆分才算好
2. @core_game_plugin.rs setup 改名字，而且应该是加载图片、音频、模型、初始化实体、摄像机、场景、地图、关卡等，这里是启动后首先加载的一批
3. 首次加载的时候，要记得图片等是有很多，不是一个。同时图片要分层的，比如背景、场景、人物、UI等。