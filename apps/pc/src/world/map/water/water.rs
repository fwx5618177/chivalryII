/// 水系配置
/// 定义水系生成的规则和参数
#[derive(Debug, Clone)]
pub struct Water {
    /// 水面高度
    pub water_level: f32,
    /// 波浪振幅
    pub wave_amplitude: f32,
    /// 波浪频率
    pub wave_frequency: f32,
    /// 水流速度
    pub flow_speed: f32,
    /// 水的透明度
    pub transparency: f32,
    /// 水的颜色
    pub color: (f32, f32, f32),

    // 河流参数
    /// 是否生成河流
    pub generate_rivers: bool,
    /// 河流密度
    pub river_density: f32,
    /// 河流宽度
    pub river_width: f32,
    /// 河流深度
    pub river_depth: f32,
    /// 河流弯曲度
    pub river_winding: f32,

    // 湖泊参数
    /// 是否生成湖泊
    pub generate_lakes: bool,
    /// 湖泊密度
    pub lake_density: f32,
    /// 湖泊最小尺寸
    pub lake_min_size: f32,
    /// 湖泊最大尺寸
    pub lake_max_size: f32,

    // 瀑布参数
    /// 是否生成瀑布
    pub generate_waterfalls: bool,
    /// 瀑布高度阈值
    pub waterfall_height_threshold: f32,
}

impl Default for Water {
    fn default() -> Self {
        Self {
            water_level: 0.3,
            wave_amplitude: 0.05,
            wave_frequency: 0.5,
            flow_speed: 1.0,
            transparency: 0.7,
            color: (0.0, 0.3, 0.8),

            generate_rivers: true,
            river_density: 0.1,
            river_width: 2.0,
            river_depth: 0.5,
            river_winding: 0.8,

            generate_lakes: true,
            lake_density: 0.05,
            lake_min_size: 10.0,
            lake_max_size: 50.0,

            generate_waterfalls: true,
            waterfall_height_threshold: 0.2,
        }
    }
}

impl Water {
    /// 创建新的水系配置
    pub fn new() -> Self {
        Self::default()
    }

    pub fn initialize(&mut self, seed: u64) {
        // 初始化水系配置
        // 这里只提供接口，实际实现由Chunk模块负责
        // 返回一个默认值，实际应用中会被覆盖
        self.water_level = 0.3;
    }

    /// 创建河流密集的水系配置
    pub fn river_rich() -> Self {
        Self {
            generate_rivers: true,
            river_density: 0.2,
            river_width: 3.0,
            river_winding: 0.9,
            ..Default::default()
        }
    }

    /// 创建湖泊密集的水系配置
    pub fn lake_rich() -> Self {
        Self {
            generate_lakes: true,
            lake_density: 0.15,
            lake_min_size: 15.0,
            lake_max_size: 80.0,
            ..Default::default()
        }
    }

    /// 创建瀑布密集的水系配置
    pub fn waterfall_rich() -> Self {
        Self {
            generate_waterfalls: true,
            waterfall_height_threshold: 0.1,
            ..Default::default()
        }
    }

    pub fn has_water_at(&self, _x: i32, _y: i32) -> bool {
        // 这里只提供接口，实际实现由Chunk模块负责
        // 返回一个默认值，实际应用中会被覆盖
        false
    }
}
