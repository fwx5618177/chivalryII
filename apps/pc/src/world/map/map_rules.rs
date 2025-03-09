use crate::world::map::{
    area::SceneRules, area::SceneType, climate::Climate, environment::EnvironmentParams,
    vegetation::Rule as VegetationRule, water::Water, SpecialAreaRules, WorldConfig,
};
use bevy::math::IVec2;
use std::collections::HashMap;

/// 地图生成规则
///
/// # 设计思路
/// 1. 统一管理所有子系统的生成规则
/// 2. 提供可配置的参数接口
/// 3. 确保各系统规则的一致性
///
/// # 功能组成
/// 1. 地形生成规则
/// 2. 场景分布规则
/// 3. 环境参数规则
/// 4. 特殊区域规则
#[derive(Debug, Clone)]
pub struct MapRules {
    /// 世界基础配置
    pub world_config: WorldConfig,

    /// 固定场景配置
    pub fixed_scenes: HashMap<IVec2, SceneType>,

    /// 场景生成规则
    pub scene_rules: SceneRules,

    /// 植被规则
    pub vegetation_rules: VegetationRule,

    /// 水系规则
    pub water_rules: Water,

    /// 气候规则
    pub climate_rules: Climate,

    /// 特殊区域规则
    pub special_area_rules: SpecialAreaRules,
}

impl Default for MapRules {
    fn default() -> Self {
        Self {
            world_config: WorldConfig::default(),
            fixed_scenes: HashMap::new(),
            scene_rules: SceneRules::default(),
            vegetation_rules: VegetationRule::default(),
            water_rules: Water::default(),
            climate_rules: Climate::default(),
            special_area_rules: SpecialAreaRules::default(),
        }
    }
}

impl MapRules {
    /// 创建新的地图规则实例
    pub fn new(seed: u64) -> Self {
        let mut rules = Self::default();
        rules.world_config.seed = seed;
        rules
    }

    /// 添加固定场景
    pub fn add_fixed_scene(&mut self, pos: IVec2, scene_type: SceneType) {
        self.fixed_scenes.insert(pos, scene_type);
    }

    /// 设置场景生成权重
    pub fn set_scene_weight(&mut self, scene_type: SceneType, weight: f32) {
        self.scene_rules.type_weights.insert(scene_type, weight);
    }

    /// 检查位置是否适合生成场景
    pub fn check_scene_compatibility(
        &self,
        pos: IVec2,
        scene_type: SceneType,
        env: &EnvironmentParams,
    ) -> bool {
        // 检查是否与现有场景距离过近
        for (existing_pos, _) in &self.fixed_scenes {
            let distance = pos.distance_squared(*existing_pos);
            if distance < (self.scene_rules.min_distance as i32).pow(2) {
                return false;
            }
        }

        // 检查环境要求
        if let Some(requirements) = self.scene_rules.environment_requirements.get(&scene_type) {
            // 检查地形兼容性
            let terrain_compat = &requirements.terrain_compatibility;
            if !terrain_compat.check_compatibility(env) {
                return false;
            }
        }

        true
    }

    /// 获取指定位置可能的场景类型
    pub fn get_possible_scenes(
        &self,
        pos: IVec2,
        env: &EnvironmentParams,
    ) -> Vec<(SceneType, f32)> {
        let mut possible_scenes = Vec::new();

        for (scene_type, weight) in &self.scene_rules.type_weights {
            if self.check_scene_compatibility(pos, *scene_type, env) {
                possible_scenes.push((*scene_type, *weight));
            }
        }

        possible_scenes
    }

    /// 验证规则配置的有效性
    pub fn validate(&self) -> Result<(), String> {
        // 验证世界配置
        if self.world_config.seed == 0 {
            return Err("世界种子不能为0".to_string());
        }

        // 验证场景规则
        if self.scene_rules.density < 0.0 || self.scene_rules.density > 1.0 {
            return Err("场景密度必须在0.0-1.0之间".to_string());
        }

        // 验证最小距离
        if self.scene_rules.min_distance < 0.0 {
            return Err("最小场景间距不能为负数".to_string());
        }

        Ok(())
    }
}
