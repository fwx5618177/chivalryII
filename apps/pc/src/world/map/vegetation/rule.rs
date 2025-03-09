/// 植被规则参数
#[derive(Debug, Clone)]
pub struct Rule {
    /// 植被整体密度系数 (0.0-1.0)
    pub density_factor: f32,

    /// 丛生植被比例 (0.0-1.0)
    /// 控制植被是否成群分布
    pub cluster_ratio: f32,

    /// 环境兼容性要求强度 (0.0-1.0)
    /// 值越高，植被类型对环境的要求越严格
    pub environment_sensitivity: f32,

    /// 随机变异度 (0.0-1.0)
    /// 控制植被分布的随机性
    pub variation: f32,
}

impl Default for Rule {
    fn default() -> Self {
        Self {
            density_factor: 0.5,
            cluster_ratio: 0.3,
            environment_sensitivity: 0.6,
            variation: 0.2,
        }
    }
}
