// Module declarations
mod types;
mod inventory;
mod player;
mod map;
mod furnace;
mod arm;
mod chest;
mod drill;
mod crafting;
mod handlers;
mod map_generation;
mod game_state;

// Re-export public types for wasm-bindgen
pub use types::{Item, Direction, PlaceableType, ResourceType};
pub use inventory::Inventory;
pub use player::Player;
pub use map::{Map, Resource, WaterPatch, PlaceableObject, IronOre, DroppedItem};
pub use furnace::FurnaceData;
pub use arm::ArmData;
pub use chest::ChestData;
pub use drill::DrillData;
pub use game_state::GameState;

// Console message struct (kept for backward compatibility if needed)
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ConsoleMessage {
    text: String,
}

#[wasm_bindgen]
impl ConsoleMessage {
    pub fn text(&self) -> String {
        self.text.clone()
    }
}
