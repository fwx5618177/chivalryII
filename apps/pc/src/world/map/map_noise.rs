use noise::{NoiseFn, Perlin};

/// 噪声生成器
///
/// # 什么是噪声生成器？
/// 噪声生成器是一个数学工具，用于生成看起来随机但实际可控的连续值。
/// 它不是用于生成声音，而是用于生成各种自然看起来的图形和数据。
///
/// # 主要用途
/// 1. 地形生成
///    - 生成自然的山脉和丘陵
///    - 创建逼真的地形起伏
///    - 模拟河流和湖泊的分布
///
/// 2. 纹理生成
///    - 创建自然的地表纹理（草地、沙漠、岩石等）
///    - 生成云纹理
///    - 制作水面波纹效果
///
/// 3. 游戏环境
///    - 控制天气系统的变化
///    - 生成自然的资源分布
///    - 创建动态的环境效果
///
/// # 工作原理示例
/// 想象一下地形生成：
/// 1. 平原区域可能的值：0.2-0.4
/// 2. 丘陵区域可能的值：0.4-0.6
/// 3. 山地区域可能的值：0.6-0.8
/// 4. 山峰区域可能的值：0.8-1.0
///
/// 通过控制这些值的平滑过渡，可以创建出自然的地形起伏。
///
/// # 为什么使用柏林噪声？
/// 1. 连续性：生成的值是平滑过渡的，没有突兀的跳变
/// 2. 可预测：相同的输入总是产生相同的输出
/// 3. 多层次：可以叠加多个频率生成更丰富的细节
///
/// # 实际应用示例
/// ```rust
/// let generator = Noise::default();
///
/// // 生成地形高度
/// let height = generator.get_in_range(x, y, 0.0, 100.0);
///
/// // 生成更自然的地形（使用分形叠加）
/// let natural_height = generator.get_fbm(x, y, 6, 0.5, 2.0);
///
/// // 生成云层覆盖
/// let cloud_cover = generator.get(x, y);
/// ```
#[derive(Debug, Clone)]
pub struct MapNoise {
    /// 柏林噪声函数实例
    ///
    /// # 说明
    /// - 使用noise-rs库提供的实现
    /// - 线程安全且性能优化
    /// - 保证确定性结果
    noise: Perlin,

    /// 缩放因子
    ///
    /// # 作用
    /// - 控制噪声的"频率"
    /// - 较大的值产生更密集的变化
    /// - 较小的值产生更平缓的变化
    ///
    /// # 取值范围
    /// - 推荐范围：0.001 ~ 1.0
    /// - 默认值：0.01
    pub scale: f32,

    /// 偏移值
    ///
    /// # 作用
    /// - 整体抬升或降低噪声值
    /// - 用于调整最终结果的范围
    ///
    /// # 取值范围
    /// - 通常在 -1.0 ~ 1.0 之间
    /// - 默认值：0.0
    pub offset: f32,
}

impl MapNoise {
    /// 创建新的噪声生成器实例
    ///
    /// # 参数说明
    /// * `seed` - 随机种子，确保生成结果的可重现性
    /// * `scale` - 缩放因子，控制噪声的"频率"
    /// * `offset` - 偏移值，调整结果范围
    ///
    /// # 示例
    /// ```rust
    /// let generator = Noise::new(42, 0.01, 0.0);
    /// ```
    pub fn new(seed: u32, scale: f32, offset: f32) -> Self {
        Self {
            noise: Perlin::new(seed),
            scale,
            offset,
        }
    }

    /// 获取单点噪声值
    ///
    /// # 功能说明
    /// 生成单个位置的噪声值，适用于简单的随机化需求
    ///
    /// # 参数
    /// * `x` - X坐标
    /// * `y` - Y坐标
    ///
    /// # 返回值
    /// 返回范围在 0.0 ~ 1.0 之间的噪声值
    ///
    /// # 性能考虑
    /// - 单次噪声计算
    /// - 适合偶尔使用的场景
    pub fn get(&self, x: f32, y: f32) -> f32 {
        let nx = x as f64 * self.scale as f64;
        let ny = y as f64 * self.scale as f64;

        let noise_val = self.noise.get([nx, ny]) as f32;
        (noise_val + 1.0) * 0.5 * self.scale + self.offset
    }

    /// 生成分形布朗运动(FBM)噪声
    ///
    /// # 功能说明
    /// 通过叠加多个不同频率的噪声层，生成更自然的噪声效果
    ///
    /// # 参数
    /// * `x` - X坐标
    /// * `y` - Y坐标
    /// * `octaves` - 噪声层数，越多细节越丰富，性能消耗越大
    /// * `persistence` - 持续度，控制每层噪声的影响程度
    /// * `lacunarity` - 层间频率变化率，控制细节的缩放程度
    ///
    /// # 实现原理
    /// 1. 每层噪声使用不同的频率和振幅
    /// 2. 频率随层数增加而提高
    /// 3. 振幅随层数增加而降低
    ///
    /// # 性能考虑
    /// - 计算复杂度与octaves成正比
    /// - 建议octaves不超过8
    ///
    /// # 应用场景
    /// 1. 地形高度图
    /// 2. 云层生成
    /// 3. 纹理细节
    pub fn get_fbm(
        &self,
        x: f32,
        y: f32,
        octaves: usize,
        persistence: f32,
        lacunarity: f32,
    ) -> f32 {
        let mut total = 0.0;
        let mut frequency = 1.0;
        let mut amplitude = 1.0;
        let mut max_value = 0.0;

        for _ in 0..octaves {
            let nx = x as f64 * frequency as f64 * self.scale as f64;
            let ny = y as f64 * frequency as f64 * self.scale as f64;

            let noise_val = self.noise.get([nx, ny]) as f32;
            total += noise_val * amplitude;

            max_value += amplitude;
            amplitude *= persistence;
            frequency *= lacunarity;
        }

        // 归一化并应用偏移
        ((total / max_value) + 1.0) * 0.5 + self.offset
    }

    /// 在指定范围内生成噪声值
    ///
    /// # 功能说明
    /// 生成指定范围内的噪声值，复用已有的Perlin实例
    ///
    /// # 参数
    /// * `x` - X坐标
    /// * `y` - Y坐标
    /// * `min` - 最小值
    /// * `max` - 最大值
    ///
    /// # 返回值
    /// 返回范围在 [min, max] 之间的噪声值
    ///
    /// # 性能优势
    /// - 复用已存在的Perlin实例
    /// - 避免重复创建开销
    /// - 适合频繁调用场景
    pub fn get_in_range(&self, x: i32, y: i32, min: f32, max: f32) -> f32 {
        let nx = x as f64 * self.scale as f64;
        let ny = y as f64 * self.scale as f64;

        let noise_val = self.noise.get([nx, ny]) as f32;
        let normalized = (noise_val + 1.0) * 0.5; // 将-1..1映射到0..1

        min + normalized * (max - min)
    }
}

impl Default for MapNoise {
    /// 创建默认的噪声生成器实例
    ///
    /// # 默认值说明
    /// - seed: 42 (经典的随机数种子)
    /// - scale: 0.01 (适合一般地形生成的缩放)
    /// - offset: 0.0 (无偏移)
    ///
    /// # 使用场景
    /// 1. 快速原型开发
    /// 2. 测试和调试
    /// 3. 无特殊需求的常规使用
    fn default() -> Self {
        Self {
            noise: Perlin::new(42),
            scale: 0.01,
            offset: 0.0,
        }
    }
}
