use wasm_bindgen::prelude::*;
use crate::types::Item;

// Inventory struct
#[wasm_bindgen]
pub struct Inventory {
    items: Vec<Item>,
}

#[wasm_bindgen]
impl Inventory {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Inventory {
        Inventory {
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn count_iron_ore(&self) -> u32 {
        self.items.iter().filter(|item| matches!(item, Item::IronOre)).count() as u32
    }

    pub fn count_copper(&self) -> u32 {
        self.items.iter().filter(|item| matches!(item, Item::Copper)).count() as u32
    }

    pub fn count_stone(&self) -> u32 {
        self.items.iter().filter(|item| matches!(item, Item::Stone)).count() as u32
    }

    pub fn count_coal(&self) -> u32 {
        self.items.iter().filter(|item| matches!(item, Item::Coal)).count() as u32
    }

    pub fn count_furnace(&self) -> u32 {
        self.items.iter().filter(|item| matches!(item, Item::Furnace)).count() as u32
    }

    pub fn count_iron_plate(&self) -> u32 {
        self.items.iter().filter(|item| matches!(item, Item::IronPlate)).count() as u32
    }

    pub fn count_belt(&self) -> u32 {
        self.items.iter().filter(|item| matches!(item, Item::Belt)).count() as u32
    }

    pub fn count_copper_plate(&self) -> u32 {
        self.items.iter().filter(|item| matches!(item, Item::CopperPlate)).count() as u32
    }

    pub fn count_arm(&self) -> u32 {
        self.items.iter().filter(|item| matches!(item, Item::Arm)).count() as u32
    }

    pub fn count_chest(&self) -> u32 {
        self.items.iter().filter(|item| matches!(item, Item::Chest)).count() as u32
    }

    pub fn count_drill(&self) -> u32 {
        self.items.iter().filter(|item| matches!(item, Item::Drill)).count() as u32
    }

    pub fn get_available_items(&self) -> Vec<Item> {
        let mut available = Vec::new();
        let mut seen = std::collections::HashSet::new();
        
        for item in &self.items {
            let item_discriminant = std::mem::discriminant(item);
            if !seen.contains(&item_discriminant) {
                seen.insert(item_discriminant);
                available.push(item.clone());
            }
        }
        
        // Sort items in a consistent order based on item type
        // This ensures the visual order matches the cycling order
        available.sort_by(|a, b| {
            let order_a = match a {
                Item::IronOre => 0,
                Item::Copper => 1,
                Item::Stone => 2,
                Item::Coal => 3,
                Item::Furnace => 4,
                Item::IronPlate => 5,
                Item::Belt => 6,
                Item::CopperPlate => 7,
                Item::Arm => 8,
                Item::Chest => 9,
                Item::Drill => 10,
            };
            let order_b = match b {
                Item::IronOre => 0,
                Item::Copper => 1,
                Item::Stone => 2,
                Item::Coal => 3,
                Item::Furnace => 4,
                Item::IronPlate => 5,
                Item::Belt => 6,
                Item::CopperPlate => 7,
                Item::Arm => 8,
                Item::Chest => 9,
                Item::Drill => 10,
            };
            order_a.cmp(&order_b)
        });
        
        available
    }

    pub fn remove_items(&mut self, item_type: Item, count: u32) -> bool {
        let mut removed = 0;
        self.items.retain(|item| {
            if std::mem::discriminant(item) == std::mem::discriminant(&item_type) && removed < count {
                removed += 1;
                false
            } else {
                true
            }
        });
        removed == count
    }

    // Clean up zero-count items from inventory
    pub fn cleanup_zero_items(&mut self) {
        // This is handled by remove_items, but we can add explicit cleanup if needed
        // For now, items are only removed when explicitly called
    }
}

