use specs::{Component, VecStorage};
use vek::*;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Agent {
    Wanderer(Vec2<f32>),
}

impl Component for Agent {
    type Storage = VecStorage<Self>;
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Control {
    pub move_dir: Vec2<f32>,
}

impl Default for Control {
    fn default() -> Self {
        Self {
            move_dir: Vec2::zero(),
        }
    }
}

impl Component for Control {
    type Storage = VecStorage<Self>;
}
