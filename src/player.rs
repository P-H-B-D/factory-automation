use wasm_bindgen::prelude::*;
use crate::types::{Item, Direction};
use crate::inventory::Inventory;

// Player struct
#[wasm_bindgen]
pub struct Player {
    x: u32,
    y: u32,
    direction: Direction,
    inventory: Inventory,
}

#[wasm_bindgen]
impl Player {
    #[wasm_bindgen(constructor)]
    pub fn new(x: u32, y: u32) -> Player {
        Player {
            x,
            y,
            direction: Direction::South,
            inventory: Inventory::new(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> u32 {
        self.x
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> u32 {
        self.y
    }

    pub fn set_position(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }

    pub fn add_to_inventory(&mut self, item: Item) {
        self.inventory.add_item(item);
    }

    pub fn get_iron_ore_count(&self) -> u32 {
        self.inventory.count_iron_ore()
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    #[wasm_bindgen(getter)]
    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn direction_value(&self) -> u32 {
        self.direction.value()
    }

    // Internal access methods (not exposed to wasm)
    pub(crate) fn inventory(&self) -> &Inventory {
        &self.inventory
    }

    pub(crate) fn inventory_mut(&mut self) -> &mut Inventory {
        &mut self.inventory
    }
}

