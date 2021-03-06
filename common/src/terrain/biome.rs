use serde_derive::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum BiomeKind {
    Void,
    Grassland,
    Ocean,
    Mountain,
    Snowlands,
    Desert,
    Swamp,
    Forest,
}
