use wasm_bindgen::prelude::*;
use js_sys;
use crate::types::{Item, Direction};
use crate::player::Player;
use crate::map::{Map, WaterPatch, Resource, PlaceableObject, IronOre, DroppedItem};
use crate::furnace::FurnaceData;
use crate::chest::ChestData;
use crate::drill::DrillData;
use crate::map_generation::generate_map;
use crate::handlers::{
    handle_player_movement, handle_mining, handle_placement,
    cycle_inventory_selection, handle_furnace_add_item, handle_pickup,
    handle_furnace_tick_processing, handle_belt_tick_processing, handle_drop_item,
    handle_belt_rotation, handle_pickup_placeable, handle_arm_tick_processing,
    handle_drill_tick_processing, get_container_at_cursor_or_front
};
use crate::crafting::{handle_crafting, handle_belt_crafting, handle_arm_crafting, handle_chest_crafting, handle_drill_crafting};

// GameState struct
#[wasm_bindgen]
pub struct GameState {
    player: Player,
    map: Map,
    console_messages: Vec<String>,
    last_movement_time: f64,
    selected_item: Option<Item>,
    current_tick: u64,
    cursor_x: Option<u32>,
    cursor_y: Option<u32>,
}

#[wasm_bindgen]
impl GameState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GameState {
        let map = generate_map();
        let player_x = map.width() / 2;
        let player_y = map.height() / 2;
        let mut player = Player::new(player_x, player_y);

        // Add 50 of each item to starting inventory
        for _ in 0..50 {
            player.add_to_inventory(Item::IronOre);
            player.add_to_inventory(Item::Copper);
            player.add_to_inventory(Item::Stone);
            player.add_to_inventory(Item::Coal);
            player.add_to_inventory(Item::Furnace);
            player.add_to_inventory(Item::IronPlate);
            player.add_to_inventory(Item::Belt);
            player.add_to_inventory(Item::CopperPlate);
            player.add_to_inventory(Item::Arm);
            player.add_to_inventory(Item::Chest);
            player.add_to_inventory(Item::Drill);
        }

        let mut game_state = GameState {
            player,
            map,
            console_messages: Vec::new(),
            last_movement_time: 0.0,
            selected_item: None,
            current_tick: 0,
            cursor_x: None,
            cursor_y: None,
        };

        // Validate selection to pick first available item
        game_state.validate_selection();

        game_state
    }

    pub fn add_console_message(&mut self, message: String) {
        self.console_messages.push(message);
        // Keep only last 50 messages
        if self.console_messages.len() > 50 {
            self.console_messages.remove(0);
        }
    }

    pub fn get_console_messages(&self) -> Vec<String> {
        self.console_messages.clone()
    }

    #[wasm_bindgen]
    pub fn next_step(&mut self, keys: &js_sys::Object, cursor_x: Option<u32>, cursor_y: Option<u32>) {
        // Update cursor position
        self.cursor_x = cursor_x;
        self.cursor_y = cursor_y;
        // Increment tick counter
        self.current_tick += 1;
        
        // Handle movement (with delay)
        handle_player_movement(self, keys);
        
        // Handle mining (M key)
        let m_pressed = js_sys::Reflect::get(keys, &JsValue::from_str("m"))
            .ok()
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        
        if m_pressed {
            handle_mining(self);
        }

        // Handle crafting (F key for furnace)
        let f_pressed = js_sys::Reflect::get(keys, &JsValue::from_str("f"))
            .ok()
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        
        if f_pressed {
            handle_crafting(self);
        }

        // Handle belt crafting (B key)
        let b_pressed = js_sys::Reflect::get(keys, &JsValue::from_str("b"))
            .ok()
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        
        if b_pressed {
            handle_belt_crafting(self);
        }

        // Handle arm crafting (P key)
        let p_pressed = js_sys::Reflect::get(keys, &JsValue::from_str("p"))
            .ok()
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        
        if p_pressed {
            handle_arm_crafting(self);
        }

        // Handle chest crafting (C key)
        let c_pressed = js_sys::Reflect::get(keys, &JsValue::from_str("c"))
            .ok()
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        
        if c_pressed {
            handle_chest_crafting(self);
        }

        // Handle drill crafting (T key)
        let t_pressed = js_sys::Reflect::get(keys, &JsValue::from_str("t"))
            .ok()
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        
        if t_pressed {
            handle_drill_crafting(self);
        }

        // Handle placement/interaction (Space key)
        // First check if there's a container (furnace/chest/drill) to add items to,
        // otherwise try to place the selected item
        let space_pressed = js_sys::Reflect::get(keys, &JsValue::from_str(" "))
            .ok()
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        
        if space_pressed {
            // Check if there's a container at cursor or in front of player
            if get_container_at_cursor_or_front(self).is_some() {
                // Add item to container
                handle_furnace_add_item(self);
            } else {
                // Try to place item
                handle_placement(self);
            }
        }

        // Handle inventory selection cycling
        let bracket_left_pressed = js_sys::Reflect::get(keys, &JsValue::from_str("["))
            .ok()
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        
        if bracket_left_pressed {
            cycle_inventory_selection(self, -1);
        }

        let bracket_right_pressed = js_sys::Reflect::get(keys, &JsValue::from_str("]"))
            .ok()
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        
        if bracket_right_pressed {
            cycle_inventory_selection(self, 1);
        }

        // Handle pickup (h key) - picks up items, or harvests from furnace if no items
        let h_pressed = js_sys::Reflect::get(keys, &JsValue::from_str("h"))
            .ok()
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        
        if h_pressed {
            handle_pickup(self);
        }

        // Handle drop (j key)
        let j_pressed = js_sys::Reflect::get(keys, &JsValue::from_str("j"))
            .ok()
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        
        if j_pressed {
            handle_drop_item(self);
        }

        // Handle belt rotation (r key)
        let r_pressed = js_sys::Reflect::get(keys, &JsValue::from_str("r"))
            .ok()
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        
        if r_pressed {
            handle_belt_rotation(self);
        }

        // Handle pickup placeable (Delete key)
        let delete_pressed = js_sys::Reflect::get(keys, &JsValue::from_str("delete"))
            .ok()
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        
        if delete_pressed {
            handle_pickup_placeable(self);
        }

        // Process furnaces each tick
        handle_furnace_tick_processing(self);
        
        // Process belts each tick
        handle_belt_tick_processing(self);
        
        // Process arms each tick
        handle_arm_tick_processing(self);
        
        // Process drills each tick
        handle_drill_tick_processing(self);
    }
    
    #[wasm_bindgen(getter)]
    pub fn current_tick(&self) -> u64 {
        self.current_tick
    }

    // Getters for JavaScript
    #[wasm_bindgen(getter)]
    pub fn player_x(&self) -> u32 {
        self.player.x()
    }

    #[wasm_bindgen(getter)]
    pub fn player_y(&self) -> u32 {
        self.player.y()
    }

    #[wasm_bindgen(getter)]
    pub fn map_width(&self) -> u32 {
        self.map.width()
    }

    #[wasm_bindgen(getter)]
    pub fn map_height(&self) -> u32 {
        self.map.height()
    }

    pub fn water_patches(&self) -> Vec<WaterPatch> {
        self.map.water_patches()
    }

    // Backward compatibility - convert resources to iron_ore list
    pub fn iron_ore(&self) -> Vec<IronOre> {
        use crate::types::ResourceType;
        self.map.resources()
            .iter()
            .filter(|r| matches!(r.resource_type(), ResourceType::IronOre))
            .map(|r| IronOre::new(r.x(), r.y()))
            .collect()
    }

    pub fn resources(&self) -> Vec<Resource> {
        self.map.resources()
    }

    pub fn placeable_objects(&self) -> Vec<PlaceableObject> {
        self.map.placeable_objects()
    }

    pub fn dropped_items(&self) -> Vec<DroppedItem> {
        self.map.dropped_items()
    }

    pub fn belt_items(&self) -> Vec<DroppedItem> {
        // Get all belt items as a vector
        let mut items = Vec::new();
        for obj in self.map.placeable_objects() {
            if matches!(obj.placeable_type(), crate::types::PlaceableType::Belt) {
                if let Some(item) = self.map.get_belt_item(obj.x(), obj.y()) {
                    items.push(item);
                }
            }
        }
        items
    }

    #[wasm_bindgen(getter)]
    pub fn iron_ore_count(&self) -> u32 {
        self.player.get_iron_ore_count()
    }

    pub fn copper_count(&self) -> u32 {
        self.player.inventory().count_copper()
    }

    pub fn stone_count(&self) -> u32 {
        self.player.inventory().count_stone()
    }

    pub fn coal_count(&self) -> u32 {
        self.player.inventory().count_coal()
    }

    pub fn furnace_count(&self) -> u32 {
        self.player.inventory().count_furnace()
    }

    pub fn iron_plate_count(&self) -> u32 {
        self.player.inventory().count_iron_plate()
    }

    pub fn belt_count(&self) -> u32 {
        self.player.inventory().count_belt()
    }

    pub fn copper_plate_count(&self) -> u32 {
        self.player.inventory().count_copper_plate()
    }

    pub fn arm_count(&self) -> u32 {
        self.player.inventory().count_arm()
    }

    pub fn chest_count(&self) -> u32 {
        self.player.inventory().count_chest()
    }

    pub fn drill_count(&self) -> u32 {
        self.player.inventory().count_drill()
    }

    pub fn get_furnace_data(&self, x: u32, y: u32) -> Option<FurnaceData> {
        self.map.get_furnace_data(x, y)
    }

    pub fn get_chest_data(&self, x: u32, y: u32) -> Option<ChestData> {
        self.map.get_chest_data(x, y)
    }

    pub fn get_drill_data(&self, x: u32, y: u32) -> Option<DrillData> {
        self.map.get_drill_data(x, y)
    }

    pub fn get_selected_item(&self) -> Option<Item> {
        // Just return what's stored - validate_selection() should keep it in sync
        self.selected_item.clone()
    }

    // Validate and update selection if needed (call after inventory changes)
    pub fn validate_selection(&mut self) {
        let available = self.player.inventory().get_available_items();
        if available.is_empty() {
            self.selected_item = None;
            return;
        }
        
        // Check if current selection is still valid
        if let Some(selected) = &self.selected_item {
            let still_available = available.iter().any(|item| std::mem::discriminant(item) == std::mem::discriminant(selected));
            if !still_available {
                // Selected item no longer exists, try to find a similar item or select first available
                // Try to find the same item type first, otherwise select first available
                if let Some(first_available) = available.first() {
                    self.selected_item = Some(first_available.clone());
                } else {
                    self.selected_item = None;
                }
            }
        } else {
            // No selection, select first available
            if let Some(first_available) = available.first() {
                self.selected_item = Some(first_available.clone());
            }
        }
    }
    
    // Get the index of the selected item in the available items list
    pub fn get_selected_item_index_in_available(&self) -> i32 {
        if let Some(selected) = &self.selected_item {
            let available = self.player.inventory().get_available_items();
            for (index, item) in available.iter().enumerate() {
                if std::mem::discriminant(item) == std::mem::discriminant(selected) {
                    return index as i32;
                }
            }
        }
        -1
    }

    pub fn get_available_items(&self) -> Vec<Item> {
        self.player.inventory().get_available_items()
    }

    #[wasm_bindgen(getter)]
    pub fn player_direction(&self) -> Direction {
        self.player.direction()
    }

    pub fn player_direction_value(&self) -> u32 {
        self.player.direction_value()
    }

    // Keep mine() for backward compatibility, but it now uses next_step
    #[wasm_bindgen]
    pub fn mine(&mut self) {
        handle_mining(self);
    }
}

// Internal access methods for handlers
impl GameState {
    pub fn player(&self) -> &Player {
        &self.player
    }

    pub fn player_mut(&mut self) -> &mut Player {
        &mut self.player
    }

    pub fn map(&self) -> &Map {
        &self.map
    }

    pub fn map_mut(&mut self) -> &mut Map {
        &mut self.map
    }

    pub fn last_movement_time(&self) -> f64 {
        self.last_movement_time
    }

    pub fn set_last_movement_time(&mut self, time: f64) {
        self.last_movement_time = time;
    }

    pub fn set_player_position(&mut self, x: u32, y: u32) {
        self.player.set_position(x, y);
    }

    pub fn set_player_direction(&mut self, direction: Direction) {
        self.player.set_direction(direction);
    }

    pub fn selected_item(&self) -> Option<Item> {
        self.selected_item.clone()
    }

    pub fn set_selected_item(&mut self, item: Option<Item>) {
        self.selected_item = item;
    }

    pub fn cursor_x(&self) -> Option<u32> {
        self.cursor_x
    }

    pub fn cursor_y(&self) -> Option<u32> {
        self.cursor_y
    }
}

