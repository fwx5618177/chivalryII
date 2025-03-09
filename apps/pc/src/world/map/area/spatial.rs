use rand_chacha::{rand_core::SeedableRng, ChaChaRng};

/// 生成一个以位置为种子的随机数生成器
pub fn make_rng_from_position(x: i32, y: i32, seed: u64) -> ChaChaRng {
    // 通过位置和种子生成一个新的种子
    let combined_seed = seed
        .wrapping_add(x as u64)
        .wrapping_mul(31)
        .wrapping_add(y as u64);

    ChaChaRng::seed_from_u64(combined_seed)
}

/// 计算两点间的曼哈顿距离
pub fn manhattan_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (x2 - x1).abs() + (y2 - y1).abs()
}

/// 计算两点间的欧几里得距离
pub fn euclidean_distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

/// 检查点是否在矩形内
pub fn point_in_rect(x: i32, y: i32, rect_x: i32, rect_y: i32, width: i32, height: i32) -> bool {
    x >= rect_x && x < rect_x + width && y >= rect_y && y < rect_y + height
}

/// 计算两个矩形是否相交
pub fn rects_intersect(
    rect1_x: i32,
    rect1_y: i32,
    rect1_width: i32,
    rect1_height: i32,
    rect2_x: i32,
    rect2_y: i32,
    rect2_width: i32,
    rect2_height: i32,
) -> bool {
    rect1_x < rect2_x + rect2_width
        && rect1_x + rect1_width > rect2_x
        && rect1_y < rect2_y + rect2_height
        && rect1_y + rect1_height > rect2_y
}

/// 计算点到线段的最短距离
pub fn point_to_line_distance(
    px: f32,
    py: f32,
    line_x1: f32,
    line_y1: f32,
    line_x2: f32,
    line_y2: f32,
) -> f32 {
    let line_length_squared = (line_x2 - line_x1).powi(2) + (line_y2 - line_y1).powi(2);

    // 如果线段长度为0，则直接计算点到端点的距离
    if line_length_squared == 0.0 {
        return euclidean_distance(px, py, line_x1, line_y1);
    }

    // 计算投影比例
    let t = ((px - line_x1) * (line_x2 - line_x1) + (py - line_y1) * (line_y2 - line_y1))
        / line_length_squared;

    // 限制t在[0,1]范围内，确保投影点在线段上
    let t = t.max(0.0).min(1.0);

    // 计算投影点坐标
    let projection_x = line_x1 + t * (line_x2 - line_x1);
    let projection_y = line_y1 + t * (line_y2 - line_y1);

    // 返回点到投影点的距离
    euclidean_distance(px, py, projection_x, projection_y)
}
