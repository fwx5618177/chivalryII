/// 植被类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VegetationType {
    Grass,    // 草地
    Flower,   // 花丛
    Bush,     // 灌木
    Bamboo,   // 竹子
    Pine,     // 松树
    Oak,      // 橡树
    Maple,    // 枫树
    Willow,   // 柳树
    DeadTree, // 枯树
}
