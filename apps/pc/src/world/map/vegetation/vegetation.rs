/// 植被配置
///
/// 定义植被生成的规则和参数
#[derive(Debug, Clone)]
pub struct Vegetation {
    /// 植被密度
    pub density: f32,
    /// 植被多样性
    pub diversity: f32,
    /// 植被尺寸
    pub size: f32,
    /// 植被分布噪声频率
    pub distribution_frequency: f32,

    // 树木参数
    /// 是否生成树木
    pub generate_trees: bool,
    /// 树木密度
    pub tree_density: f32,
    /// 树木最小高度
    pub tree_min_height: f32,
    /// 树木最大高度
    pub tree_max_height: f32,
    /// 树木种类
    pub tree_types: Vec<String>,

    // 草地参数
    /// 是否生成草地
    pub generate_grass: bool,
    /// 草地密度
    pub grass_density: f32,
    /// 草地高度
    pub grass_height: f32,

    // 花卉参数
    /// 是否生成花卉
    pub generate_flowers: bool,
    /// 花卉密度
    pub flower_density: f32,
    /// 花卉种类
    pub flower_types: Vec<String>,

    // 竹林参数
    /// 是否生成竹林
    pub generate_bamboo: bool,
    /// 竹林密度
    pub bamboo_density: f32,
    /// 竹子高度
    pub bamboo_height: f32,
}

impl Default for Vegetation {
    fn default() -> Self {
        Self {
            density: 0.5,
            diversity: 0.7,
            size: 1.0,
            distribution_frequency: 0.1,

            generate_trees: true,
            tree_density: 0.3,
            tree_min_height: 3.0,
            tree_max_height: 8.0,
            tree_types: vec!["pine".to_string(), "oak".to_string(), "maple".to_string()],

            generate_grass: true,
            grass_density: 0.8,
            grass_height: 0.5,

            generate_flowers: true,
            flower_density: 0.2,
            flower_types: vec![
                "lotus".to_string(),
                "peony".to_string(),
                "plum_blossom".to_string(),
            ],

            generate_bamboo: true,
            bamboo_density: 0.4,
            bamboo_height: 6.0,
        }
    }
}

impl Vegetation {
    /// 创建新的植被配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 创建森林密集的植被配置
    pub fn forest() -> Self {
        Self {
            tree_density: 0.7,
            tree_min_height: 5.0,
            tree_max_height: 12.0,
            grass_density: 0.5,
            ..Default::default()
        }
    }

    /// 创建草原的植被配置
    pub fn grassland() -> Self {
        Self {
            tree_density: 0.1,
            grass_density: 0.9,
            flower_density: 0.4,
            ..Default::default()
        }
    }

    /// 创建竹林的植被配置
    pub fn bamboo_forest() -> Self {
        Self {
            generate_bamboo: true,
            bamboo_density: 0.8,
            bamboo_height: 8.0,
            tree_density: 0.1,
            ..Default::default()
        }
    }
}
