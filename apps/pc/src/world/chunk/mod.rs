mod chunk_loader;
/// 区块系统模块组织
/// 将区块系统分为管理、加载和实现三个主要部分
/// 这种分离有助于：
/// 1. 关注点分离：管理器负责状态维护，加载器负责IO操作，实现模块负责具体功能
/// 2. 代码组织：便于维护和扩展
/// 3. 依赖管理：明确模块间的依赖关系
mod chunk_manager;
mod render;
mod systems;

pub use chunk_loader::*;
pub use chunk_manager::*;
pub use render::*;
pub use systems::ChunkSystemPlugin;

/// 区块大小常量
/// 设置为32是因为：
/// 1. 是2的幂，便于内存对齐
/// 2. 足够大以减少区块数量
/// 3. 足够小以保持加载性能
pub const CHUNK_SIZE: usize = 32;
