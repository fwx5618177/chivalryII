#[derive(Debug, Clone)]
pub struct River {
    /// 最小河道宽度
    pub min_width: i32,
    /// 最大河道宽度
    pub max_width: i32,
    /// 河流蜿蜒程度 (0.0-1.0)
    pub meandering: f32,
    /// 分支概率
    pub branch_probability: f32,
    /// 最大分支数
    pub max_branches: i32,
}

impl Default for River {
    fn default() -> Self {
        Self {
            min_width: 1,
            max_width: 3,
            meandering: 0.3,
            branch_probability: 0.15,
            max_branches: 2,
        }
    }
}
