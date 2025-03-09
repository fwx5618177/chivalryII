/// NPC类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NpcType {
    Merchant,   // 商人
    Guard,      // 守卫
    Villager,   // 村民
    Master,     // 武学大师
    Doctor,     // 医师
    Blacksmith, // 铁匠
}
