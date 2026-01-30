use wasm_bindgen::prelude::*;
use crate::types::Item;

// Chest inventory data - can hold multiple items and stacks
#[wasm_bindgen]
#[derive(Clone)]
pub struct ChestData {
    item_types: Vec<Item>, // Vector of item types
    quantities: Vec<u32>, // Vector of quantities (parallel to item_types)
}

#[wasm_bindgen]
impl ChestData {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ChestData {
        ChestData {
            item_types: Vec::new(),
            quantities: Vec::new(),
        }
    }

    // Add an item to the chest (stacks if same type exists)
    pub fn add_item(&mut self, item: Item, quantity: u32) {
        // Try to find existing stack of same item
        for i in 0..self.item_types.len() {
            if std::mem::discriminant(&self.item_types[i]) == std::mem::discriminant(&item) {
                self.quantities[i] += quantity;
                return;
            }
        }
        // No existing stack, add new entry
        self.item_types.push(item);
        self.quantities.push(quantity);
    }

    // Remove items from chest
    pub fn remove_item(&mut self, item: Item, quantity: u32) -> bool {
        for i in 0..self.item_types.len() {
            if std::mem::discriminant(&self.item_types[i]) == std::mem::discriminant(&item) {
                if self.quantities[i] >= quantity {
                    self.quantities[i] -= quantity;
                    // Remove entry if count is now 0
                    if self.quantities[i] == 0 {
                        self.item_types.remove(i);
                        self.quantities.remove(i);
                    }
                    return true;
                } else {
                    return false;
                }
            }
        }
        false
    }

    // Get all items (for display)
    pub fn get_all_items(&self) -> Vec<Item> {
        self.item_types.clone()
    }

    // Get quantity for an item at a specific index (used with get_all_items)
    pub fn get_item_quantity(&self, index: usize) -> u32 {
        if index < self.quantities.len() {
            self.quantities[index]
        } else {
            0
        }
    }

    // Check if chest has space (for now, unlimited)
    pub fn has_space(&self) -> bool {
        true // Chests have unlimited space
    }
}

