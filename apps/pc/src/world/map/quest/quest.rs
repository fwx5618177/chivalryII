use super::Reward;

#[derive(Debug, Clone)]
pub struct Quest {
    pub id: String,
    pub title: String,
    pub description: String,
    pub rewards: Vec<Reward>,
}
