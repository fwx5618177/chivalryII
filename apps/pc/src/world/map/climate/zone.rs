/// 气候区域系统
///
/// # 设计目的
/// 1. 地域特色：每个区域都有独特的气候特征
/// 2. 玩法引导：不同区域适合不同的游戏策略
/// 3. 探索动机：通过气候差异创造探索欲望
///
/// # 区域特点
/// - Tropical: 资源丰富，但也有特殊危险
/// - Temperate: 最适合初期发展
/// - Continental: 挑战性适中，资源均衡
/// - Polar: 高难度区域，特殊资源
/// - Desert: 极端环境，独特玩法
/// - Mountains: 战略要地，稀有资源
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Zone {
    Tropical,    // 热带
    Temperate,   // 温带
    Continental, // 大陆性
    Polar,       // 极地
    Desert,      // 沙漠
    Mountains,   // 高山
}
