/// 季节系统
///
/// # 设计目标
/// 1. 游戏节奏：通过季节变化创造游戏周期
/// 2. 玩法多样：不同季节提供不同的游戏体验
/// 3. 资源循环：影响资源的分布和获取难度
///
/// # 季节特点
/// - Spring: 适合种植和采集，天气温和
/// - Summer: 炎热干燥，适合探索远方
/// - Autumn: 收获的季节，资源丰富
/// - Winter: 生存考验，需要特殊策略
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}
