/// 地图瓦片基础类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Empty,       // 空地块
    Ground,      // 一般地面
    Wall,        // 墙壁
    Water,       // 水体
    Grass,       // 草地
    Sand,        // 沙地
    Rock,        // 岩石
    Snow,        // 雪地
    Forest,      // 森林
    Path,        // 小径
    Plains,      // 平原
    Wasteland,   // 荒地
    Bamboo,      // 竹林
    DenseForest, // 密林
    Mountain,    // 山地
}
