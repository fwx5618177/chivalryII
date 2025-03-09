use super::super::EnvironmentCompatibility;
use super::{density::VegetationDensity, vegetation_type::VegetationType, Rule};
use bevy::utils::HashMap;
use rand::Rng;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::{rand_core::RngCore, ChaCha8Rng};

/// 植被系统
///
/// 负责植被的生成和分布规则管理
/// 设计理念：
/// 1. 环境适应性 - 不同植被类型适合不同环境
/// 2. 真实分布 - 模拟自然界植被的群落特性
/// 3. 武侠风格 - 中国特色植被分布规律
#[derive(Debug, Clone)]
pub struct System {
    /// 全局参数设置
    pub params: Rule,

    /// 植被类型对环境的适应性规则
    pub compatibility_rules: HashMap<VegetationType, EnvironmentCompatibility>,

    /// 植被分布缓存
    vegetation_cache: HashMap<(i32, i32), Option<VegetationType>>,

    /// 种子
    pub seed: u64,
}

impl Default for System {
    fn default() -> Self {
        let mut compatibility_rules = HashMap::new();

        // 设置各类植被的环境适应性

        // 草地 - 适应性广泛
        compatibility_rules.insert(
            VegetationType::Grass,
            EnvironmentCompatibility {
                ideal_height: (0.2, 0.6),
                survivable_height: (0.1, 0.7),
                ideal_temperature: (0.3, 0.8),
                survivable_temperature: (0.2, 0.9),
                ideal_moisture: (0.4, 0.7),
                survivable_moisture: (0.2, 0.9),
            },
        );

        // 花丛 - 需要适中温度和湿度
        compatibility_rules.insert(
            VegetationType::Flower,
            EnvironmentCompatibility {
                ideal_height: (0.25, 0.5),
                survivable_height: (0.2, 0.6),
                ideal_temperature: (0.4, 0.7),
                survivable_temperature: (0.3, 0.8),
                ideal_moisture: (0.5, 0.8),
                survivable_moisture: (0.4, 0.9),
            },
        );

        // 竹子 - 喜湿润，适温
        compatibility_rules.insert(
            VegetationType::Bamboo,
            EnvironmentCompatibility {
                ideal_height: (0.3, 0.5),
                survivable_height: (0.2, 0.6),
                ideal_temperature: (0.5, 0.8),
                survivable_temperature: (0.4, 0.9),
                ideal_moisture: (0.6, 0.9),
                survivable_moisture: (0.5, 1.0),
            },
        );

        // 松树 - 适应性强，可在较高海拔生存
        compatibility_rules.insert(
            VegetationType::Pine,
            EnvironmentCompatibility {
                ideal_height: (0.4, 0.8),
                survivable_height: (0.3, 0.9),
                ideal_temperature: (0.2, 0.6),
                survivable_temperature: (0.1, 0.7),
                ideal_moisture: (0.3, 0.7),
                survivable_moisture: (0.2, 0.8),
            },
        );

        // 枫树 - 温带树种，较为挑剔
        compatibility_rules.insert(
            VegetationType::Maple,
            EnvironmentCompatibility {
                ideal_height: (0.3, 0.6),
                survivable_height: (0.2, 0.7),
                ideal_temperature: (0.4, 0.7),
                survivable_temperature: (0.3, 0.8),
                ideal_moisture: (0.4, 0.7),
                survivable_moisture: (0.3, 0.8),
            },
        );

        // 柳树 - 喜水
        compatibility_rules.insert(
            VegetationType::Willow,
            EnvironmentCompatibility {
                ideal_height: (0.1, 0.4),
                survivable_height: (0.05, 0.5),
                ideal_temperature: (0.4, 0.7),
                survivable_temperature: (0.3, 0.8),
                ideal_moisture: (0.7, 1.0),
                survivable_moisture: (0.6, 1.0),
            },
        );

        Self {
            params: Rule::default(),
            compatibility_rules,
            vegetation_cache: HashMap::new(),
            seed: 12345,
        }
    }
}

impl System {
    /// 初始化植被系统
    pub fn initialize(&mut self, seed: u64) {
        self.seed = seed;
        self.vegetation_cache.clear();
    }

    /// 获取指定位置的植被类型
    pub fn get_vegetation_at(
        &self,
        x: i32,
        y: i32,
        height: f32,
        temperature: f32,
        moisture: f32,
    ) -> Option<VegetationType> {
        // 查询缓存
        if let Some(veg_type) = self.vegetation_cache.get(&(x, y)) {
            return *veg_type;
        }

        // 生成随机种子，确保同一位置生成结果一致
        let mut rng = self.make_rng_from_position(x, y);

        // 随机初始值 (0.0-1.0)
        let random_base = rng.gen::<f32>();

        // 密度检查 - 全局密度因子与随机值比较，过滤掉部分位置
        if random_base > self.params.density_factor {
            return None;
        }

        // 确定可能的植被类型
        let mut candidates = Vec::new();

        for (veg_type, compat) in &self.compatibility_rules {
            // 检查环境适应性
            if self.check_compatibility(compat, height, temperature, moisture) {
                // 计算该类型植被在此环境中的适合度 (0.0-1.0)
                let suitability = self.calculate_suitability(compat, height, temperature, moisture);
                candidates.push((*veg_type, suitability));
            }
        }

        // 如果没有适合的植被，返回None
        if candidates.is_empty() {
            return None;
        }

        // 保存最适合的候选项（排序前）
        let best_candidate = candidates[0];

        // 按适合度排序
        candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // 随机选择，适合度高的有更大概率被选中
        let total_weight: f32 = candidates.iter().map(|(_, w)| w).sum();
        let random_value = rng.gen::<f32>() * total_weight;

        let mut cumulative = 0.0;
        for (veg_type, weight) in candidates {
            cumulative += weight;
            if random_value <= cumulative {
                return Some(veg_type);
            }
        }

        // 默认返回最适合的
        Some(best_candidate.0)
    }

    /// 获取指定区域的植被密度
    pub fn get_density(
        &self,
        x: i32,
        y: i32,
        radius: i32,
        height: f32,
        temperature: f32,
        moisture: f32,
    ) -> VegetationDensity {
        let mut count = 0;
        let total = (2 * radius + 1).pow(2);

        for dx in -radius..=radius {
            for dy in -radius..=radius {
                if self
                    .get_vegetation_at(x + dx, y + dy, height, temperature, moisture)
                    .is_some()
                {
                    count += 1;
                }
            }
        }

        let density_ratio = count as f32 / total as f32;

        if density_ratio < 0.05 {
            VegetationDensity::None
        } else if density_ratio < 0.2 {
            VegetationDensity::Sparse
        } else if density_ratio < 0.4 {
            VegetationDensity::Medium
        } else if density_ratio < 0.7 {
            VegetationDensity::Dense
        } else {
            VegetationDensity::VeryDense
        }
    }

    /// 检查植被是否与环境兼容
    fn check_compatibility(
        &self,
        compat: &EnvironmentCompatibility,
        height: f32,
        temperature: f32,
        moisture: f32,
    ) -> bool {
        // 检查是否在可生存范围内
        let height_ok =
            height >= compat.survivable_height.0 && height <= compat.survivable_height.1;
        let temp_ok = temperature >= compat.survivable_temperature.0
            && temperature <= compat.survivable_temperature.1;
        let moisture_ok =
            moisture >= compat.survivable_moisture.0 && moisture <= compat.survivable_moisture.1;

        height_ok && temp_ok && moisture_ok
    }

    /// 计算植被在特定环境中的适合度
    fn calculate_suitability(
        &self,
        compat: &EnvironmentCompatibility,
        height: f32,
        temperature: f32,
        moisture: f32,
    ) -> f32 {
        // 基于理想范围计算各因素的适合度
        let height_score = self.calculate_factor_score(
            height,
            compat.ideal_height.0,
            compat.ideal_height.1,
            compat.survivable_height.0,
            compat.survivable_height.1,
        );

        let temp_score = self.calculate_factor_score(
            temperature,
            compat.ideal_temperature.0,
            compat.ideal_temperature.1,
            compat.survivable_temperature.0,
            compat.survivable_temperature.1,
        );

        let moisture_score = self.calculate_factor_score(
            moisture,
            compat.ideal_moisture.0,
            compat.ideal_moisture.1,
            compat.survivable_moisture.0,
            compat.survivable_moisture.1,
        );

        // 综合评分
        let combined_score = (height_score * temp_score * moisture_score).powf(1.0 / 3.0);

        // 应用环境敏感度
        combined_score.powf(self.params.environment_sensitivity)
    }

    /// 计算单个因子的得分
    fn calculate_factor_score(
        &self,
        value: f32,
        ideal_min: f32,
        ideal_max: f32,
        survive_min: f32,
        survive_max: f32,
    ) -> f32 {
        if value < survive_min || value > survive_max {
            return 0.0;
        }

        if value >= ideal_min && value <= ideal_max {
            return 1.0;
        }

        if value < ideal_min {
            // 从最低可生存值到理想最低值之间线性插值
            return (value - survive_min) / (ideal_min - survive_min);
        } else {
            // 从理想最高值到最高可生存值之间线性插值
            return (survive_max - value) / (survive_max - ideal_max);
        }
    }

    /// 生成位置相关的随机数生成器
    fn make_rng_from_position(&self, x: i32, y: i32) -> ChaCha8Rng {
        let combined_seed = self
            .seed
            .wrapping_add(x as u64)
            .wrapping_mul(31)
            .wrapping_add(y as u64);

        ChaCha8Rng::seed_from_u64(combined_seed)
    }
}
