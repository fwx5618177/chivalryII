/// 植被密度等级
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VegetationDensity {
    None,      // 无植被
    Sparse,    // 稀疏
    Medium,    // 中等
    Dense,     // 茂密
    VeryDense, // 非常茂密
}
