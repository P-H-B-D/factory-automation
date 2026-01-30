use wasm_bindgen::prelude::*;
use js_sys;
use crate::types::{Item, Direction, PlaceableType, ResourceType};
use crate::game_state::GameState;
use crate::map::DroppedItem;

// Handle player movement
pub fn handle_player_movement(game_state: &mut GameState, keys: &js_sys::Object) {
    // Check for 20ms delay between movements
    let current_time = js_sys::Date::now();
    if current_time - game_state.last_movement_time() < 20.0 {
        return;
    }

    // Check for key presses
    let w_pressed = js_sys::Reflect::get(keys, &JsValue::from_str("w"))
        .ok()
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let a_pressed = js_sys::Reflect::get(keys, &JsValue::from_str("a"))
        .ok()
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let s_pressed = js_sys::Reflect::get(keys, &JsValue::from_str("s"))
        .ok()
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let d_pressed = js_sys::Reflect::get(keys, &JsValue::from_str("d"))
        .ok()
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    // Check if any movement key is pressed
    if !w_pressed && !a_pressed && !s_pressed && !d_pressed {
        return;
    }

    // Calculate potential new position and update direction
    let mut new_x = game_state.player_x();
    let mut new_y = game_state.player_y();
    let mut new_direction = game_state.player_direction();
    
    // Try to move in each direction (1 tile per movement) and update direction
    if w_pressed && new_y > 0 {
        new_y = new_y - 1;
        new_direction = Direction::North;
    }
    if s_pressed && new_y < game_state.map_height() - 1 {
        new_y = new_y + 1;
        new_direction = Direction::South;
    }
    if a_pressed && new_x > 0 {
        new_x = new_x - 1;
        new_direction = Direction::West;
    }
    if d_pressed && new_x < game_state.map_width() - 1 {
        new_x = new_x + 1;
        new_direction = Direction::East;
    }
    
    // Check for collision with water patches and placeable objects before updating position
    if !game_state.map().would_collide_with_water(new_x, new_y) 
        && !game_state.map().would_collide_with_placeable(new_x, new_y) {
        game_state.set_player_position(new_x, new_y);
        game_state.set_player_direction(new_direction);
        game_state.set_last_movement_time(current_time);
    }
}

// Handle mining interaction
pub fn handle_mining(game_state: &mut GameState) {
    let player_x = game_state.player_x();
    let player_y = game_state.player_y();
    
    // Check all 8 adjacent tiles (including diagonals)
    let directions = [
        (-1, -1), (0, -1), (1, -1),
        (-1,  0),          (1,  0),
        (-1,  1), (0,  1), (1,  1),
    ];
    
    for (dx, dy) in directions.iter() {
        let check_x = (player_x as i32) + dx;
        let check_y = (player_y as i32) + dy;
        
        // Check bounds
        if check_x < 0 || check_y < 0 
            || check_x >= game_state.map_width() as i32 
            || check_y >= game_state.map_height() as i32
        {
            continue;
        }
        
        let check_x = check_x as u32;
        let check_y = check_y as u32;
        
        // Check if there's a resource at this position
        if let Some(resource_type) = game_state.map().get_resource_at(check_x, check_y) {
            // Add item to inventory based on resource type
            let item = match resource_type {
                ResourceType::IronOre => Item::IronOre,
                ResourceType::Copper => Item::Copper,
                ResourceType::Stone => Item::Stone,
                ResourceType::Coal => Item::Coal,
            };
            
            game_state.player_mut().add_to_inventory(item.clone());
            
            // Validate selection after inventory change
            game_state.validate_selection();
            
            // Log to console
            let resource_name = match resource_type {
                ResourceType::IronOre => "Iron Ore",
                ResourceType::Copper => "Copper",
                ResourceType::Stone => "Stone",
                ResourceType::Coal => "Coal",
            };
            
            let count = match &item {
                Item::IronOre => game_state.player().get_iron_ore_count(),
                Item::Copper => game_state.player().inventory().count_copper(),
                Item::Stone => game_state.player().inventory().count_stone(),
                Item::Coal => game_state.player().inventory().count_coal(),
                Item::Furnace => game_state.player().inventory().count_furnace(),
                Item::IronPlate => game_state.player().inventory().count_iron_plate(),
                Item::Belt => game_state.player().inventory().count_belt(),
                Item::CopperPlate => game_state.player().inventory().count_copper_plate(),
                Item::Arm => game_state.player().inventory().count_arm(),
                Item::Chest => game_state.player().inventory().count_chest(),
                Item::Drill => game_state.player().inventory().count_drill(),
            };
            
            let message = format!("Mined {}! Total: {}", resource_name, count);
            game_state.add_console_message(message);
            
            break; // Only mine one resource per action
        }
    }
}

// Handle placement
pub fn handle_placement(game_state: &mut GameState) {
    // Use cursor position if available, otherwise use position in front of player
    let (place_x, place_y) = if let (Some(cx), Some(cy)) = (game_state.cursor_x(), game_state.cursor_y()) {
        (cx, cy)
    } else {
        let player_x = game_state.player_x();
        let player_y = game_state.player_y();
        let direction = game_state.player_direction();
        
        // Calculate position in front of player
        match direction {
            Direction::North => (player_x, player_y.saturating_sub(1)),
            Direction::South => (player_x, player_y + 1),
            Direction::East => (player_x + 1, player_y),
            Direction::West => (player_x.saturating_sub(1), player_y),
        }
    };
    
    // Check bounds
    if place_x >= game_state.map_width() || place_y >= game_state.map_height() {
        game_state.add_console_message("Cannot place outside map bounds!".to_string());
        return;
    }
    
    // Get selected item
    let selected_item = match game_state.get_selected_item() {
        Some(item) => item,
        None => {
            game_state.add_console_message("No item selected!".to_string());
            return;
        }
    };
    
    // Check if selected item is placeable
    let placeable_type = match selected_item {
        Item::Furnace => {
            if game_state.player().inventory().count_furnace() == 0 {
                game_state.add_console_message("No furnace in inventory!".to_string());
                return;
            }
            Some(PlaceableType::Furnace)
        }
        Item::Belt => {
            if game_state.player().inventory().count_belt() == 0 {
                game_state.add_console_message("No belt in inventory!".to_string());
                return;
            }
            Some(PlaceableType::Belt)
        }
        Item::Arm => {
            if game_state.player().inventory().count_arm() == 0 {
                game_state.add_console_message("No arm in inventory!".to_string());
                return;
            }
            Some(PlaceableType::Arm)
        }
        Item::Chest => {
            if game_state.player().inventory().count_chest() == 0 {
                game_state.add_console_message("No chest in inventory!".to_string());
                return;
            }
            Some(PlaceableType::Chest)
        }
        Item::Drill => {
            if game_state.player().inventory().count_drill() == 0 {
                game_state.add_console_message("No drill in inventory!".to_string());
                return;
            }
            Some(PlaceableType::Drill)
        }
        _ => {
            game_state.add_console_message("Selected item cannot be placed!".to_string());
            return;
        }
    };
    
    if let Some(place_type) = placeable_type {
        // Check if position is valid
        if game_state.map().would_collide_with_water(place_x, place_y) {
            game_state.add_console_message("Cannot place on water!".to_string());
            return;
        }
        if game_state.map().get_placeable_at(place_x, place_y).is_some() {
            game_state.add_console_message("Position already occupied!".to_string());
            return;
        }
        // Drills can be placed on resources, other items cannot
        if !matches!(place_type, PlaceableType::Drill) {
            if game_state.map().get_resource_at(place_x, place_y).is_some() {
                game_state.add_console_message("Cannot place on resource!".to_string());
                return;
            }
        }
        
        // Remove item from inventory
        let had_multiple = match place_type {
            PlaceableType::Furnace => game_state.player().inventory().count_furnace() > 1,
            PlaceableType::Belt => game_state.player().inventory().count_belt() > 1,
            PlaceableType::Arm => game_state.player().inventory().count_arm() > 1,
            PlaceableType::Chest => game_state.player().inventory().count_chest() > 1,
            PlaceableType::Drill => game_state.player().inventory().count_drill() > 1,
        };
        
        match place_type {
            PlaceableType::Furnace => {
                game_state.player_mut().inventory_mut().remove_items(Item::Furnace, 1);
            }
            PlaceableType::Belt => {
                game_state.player_mut().inventory_mut().remove_items(Item::Belt, 1);
            }
            PlaceableType::Arm => {
                game_state.player_mut().inventory_mut().remove_items(Item::Arm, 1);
            }
            PlaceableType::Chest => {
                game_state.player_mut().inventory_mut().remove_items(Item::Chest, 1);
            }
            PlaceableType::Drill => {
                game_state.player_mut().inventory_mut().remove_items(Item::Drill, 1);
            }
        }
        
        // If we just removed the last item, adjust selection
        if !had_multiple {
            // The selected item was just removed, so we need to pick a new one
            // Just use validate_selection which will pick the first available item
            game_state.validate_selection();
        } else {
            // Still have items of this type, validate selection
            game_state.validate_selection();
        }
        
        // Place the object
        game_state.map_mut().add_placeable(place_x, place_y, place_type);
        let item_name = match place_type {
            PlaceableType::Furnace => "furnace",
            PlaceableType::Belt => "belt",
            PlaceableType::Arm => "arm",
            PlaceableType::Chest => "chest",
            PlaceableType::Drill => "drill",
        };
        game_state.add_console_message(format!("Placed {}!", item_name));
    }
}

// Get furnace position at cursor or in front of player
pub fn get_furnace_at_cursor_or_front(game_state: &GameState) -> Option<(u32, u32)> {
    // Use cursor position if available, otherwise use position in front of player
    let (check_x, check_y) = if let (Some(cx), Some(cy)) = (game_state.cursor_x(), game_state.cursor_y()) {
        (cx, cy)
    } else {
        let player_x = game_state.player_x();
        let player_y = game_state.player_y();
        let direction = game_state.player_direction();
        
        // Calculate position in front of player
        match direction {
            Direction::North => (player_x, player_y.saturating_sub(1)),
            Direction::South => (player_x, player_y + 1),
            Direction::East => (player_x + 1, player_y),
            Direction::West => (player_x.saturating_sub(1), player_y),
        }
    };
    
    // Check bounds
    if check_x >= game_state.map_width() || check_y >= game_state.map_height() {
        return None;
    }
    
    // Check if there's a furnace at this position
    if let Some(placeable_type) = game_state.map().get_placeable_at(check_x, check_y) {
        if matches!(placeable_type, PlaceableType::Furnace) {
            return Some((check_x, check_y));
        }
    }
    
    None
}

// Cycle inventory selection
pub fn cycle_inventory_selection(game_state: &mut GameState, direction: i32) {
    let available = game_state.player().inventory().get_available_items();
    if available.is_empty() {
        game_state.set_selected_item(None);
        return;
    }
    
    // Find current selection index
    let current_index = if let Some(selected) = game_state.selected_item() {
        available.iter().position(|item| std::mem::discriminant(item) == std::mem::discriminant(&selected))
            .unwrap_or(0)
    } else {
        0
    };
    
    // Calculate new index
    let new_index = if direction > 0 {
        (current_index + 1) % available.len()
    } else {
        if current_index == 0 {
            available.len() - 1
        } else {
            current_index - 1
        }
    };
    
    // Set new selection
    game_state.set_selected_item(Some(available[new_index].clone()));
}

// Get container (furnace or chest) position at cursor or in front of player
pub fn get_container_at_cursor_or_front(game_state: &GameState) -> Option<((u32, u32), PlaceableType)> {
    // Use cursor position if available, otherwise use position in front of player
    let (check_x, check_y) = if let (Some(cx), Some(cy)) = (game_state.cursor_x(), game_state.cursor_y()) {
        (cx, cy)
    } else {
        let player_x = game_state.player_x();
        let player_y = game_state.player_y();
        let direction = game_state.player_direction();
        
        // Calculate position in front of player
        match direction {
            Direction::North => (player_x, player_y.saturating_sub(1)),
            Direction::South => (player_x, player_y + 1),
            Direction::East => (player_x + 1, player_y),
            Direction::West => (player_x.saturating_sub(1), player_y),
        }
    };
    
    // Check bounds
    if check_x >= game_state.map_width() || check_y >= game_state.map_height() {
        return None;
    }
    
    // Check if there's a furnace, chest, or drill at this position
    if let Some(placeable_type) = game_state.map().get_placeable_at(check_x, check_y) {
        if matches!(placeable_type, PlaceableType::Furnace | PlaceableType::Chest | PlaceableType::Drill) {
            return Some(((check_x, check_y), placeable_type));
        }
    }
    
    None
}

// Handle adding selected item to furnace or chest
pub fn handle_furnace_add_item(game_state: &mut GameState) {
    if let Some(((container_x, container_y), container_type)) = get_container_at_cursor_or_front(game_state) {
        // Get selected item
        let selected_item = match game_state.get_selected_item() {
            Some(item) => item,
            None => {
                game_state.add_console_message("No item selected!".to_string());
                return;
            }
        };
        
        // Handle chest (accepts any item)
        if matches!(container_type, PlaceableType::Chest) {
            // Get item count
            let item_count = match selected_item {
                Item::IronOre => game_state.player().inventory().count_iron_ore(),
                Item::Copper => game_state.player().inventory().count_copper(),
                Item::Stone => game_state.player().inventory().count_stone(),
                Item::Coal => game_state.player().inventory().count_coal(),
                Item::Furnace => game_state.player().inventory().count_furnace(),
                Item::IronPlate => game_state.player().inventory().count_iron_plate(),
                Item::Belt => game_state.player().inventory().count_belt(),
                Item::CopperPlate => game_state.player().inventory().count_copper_plate(),
                Item::Arm => game_state.player().inventory().count_arm(),
                Item::Chest => game_state.player().inventory().count_chest(),
                Item::Drill => game_state.player().inventory().count_drill(),
            };
            
            if item_count > 0 {
                if let Some(mut chest_data) = game_state.map().get_chest_data(container_x, container_y) {
                    let had_item = item_count > 1;
                    game_state.player_mut().inventory_mut().remove_items(selected_item.clone(), 1);
                    chest_data.add_item(selected_item.clone(), 1);
                    game_state.map_mut().set_chest_data(container_x, container_y, chest_data);
                    game_state.add_console_message(format!("Added {} to chest!", selected_item.name()));
                    
                    if !had_item {
                        game_state.validate_selection();
                    } else {
                        game_state.validate_selection();
                    }
                }
            } else {
                game_state.add_console_message(format!("No {} in inventory!", selected_item.name()));
            }
            return;
        }
        
        // Handle drill (only allows coal)
        if matches!(container_type, PlaceableType::Drill) {
            match selected_item {
                Item::Coal => {
                    if game_state.player().inventory().count_coal() > 0 {
                        if let Some(mut drill_data) = game_state.map().get_drill_data(container_x, container_y) {
                            let had_coal = game_state.player().inventory().count_coal() > 1;
                            game_state.player_mut().inventory_mut().remove_items(Item::Coal, 1);
                            drill_data.add_coal();
                            game_state.map_mut().set_drill_data(container_x, container_y, drill_data);
                            game_state.add_console_message("Added coal to drill!".to_string());
                            
                            if !had_coal {
                                game_state.validate_selection();
                            } else {
                                game_state.validate_selection();
                            }
                        }
                    } else {
                        game_state.add_console_message("No coal in inventory!".to_string());
                    }
                }
                _ => {
                    game_state.add_console_message("Drill only accepts coal!".to_string());
                }
            }
            return;
        }
        
        // Handle furnace (only allows coal, iron ore, or copper)
        // Only allow coal or iron ore
        match selected_item {
            Item::Coal => {
                if game_state.player().inventory().count_coal() > 0 {
                    if let Some(mut furnace_data) = game_state.map().get_furnace_data(container_x, container_y) {
                        let had_coal = game_state.player().inventory().count_coal() > 1;
                        game_state.player_mut().inventory_mut().remove_items(Item::Coal, 1);
                        furnace_data.add_coal();
                        game_state.map_mut().set_furnace_data(container_x, container_y, furnace_data);
                        game_state.add_console_message("Added coal to furnace!".to_string());
                        
                        // If we just removed the last coal, adjust selection
                        if !had_coal {
                            // Coal count is now 0, need to change selection
                            // Just use validate_selection which will pick the first available item
                            game_state.validate_selection();
                        } else {
                            // Still have coal, validate selection
                            game_state.validate_selection();
                        }
                    }
                } else {
                    game_state.add_console_message("No coal in inventory!".to_string());
                }
            }
            Item::IronOre => {
                if game_state.player().inventory().count_iron_ore() > 0 {
                    if let Some(mut furnace_data) = game_state.map().get_furnace_data(container_x, container_y) {
                        let had_iron = game_state.player().inventory().count_iron_ore() > 1;
                        game_state.player_mut().inventory_mut().remove_items(Item::IronOre, 1);
                        furnace_data.add_iron_ore();
                        game_state.map_mut().set_furnace_data(container_x, container_y, furnace_data);
                        game_state.add_console_message("Added iron ore to furnace!".to_string());
                        
                        // If we just removed the last iron ore, adjust selection
                        if !had_iron {
                            // Iron ore count is now 0, need to change selection
                            // Just use validate_selection which will pick the first available item
                            game_state.validate_selection();
                        } else {
                            // Still have iron ore, validate selection
                            game_state.validate_selection();
                        }
                    }
                } else {
                    game_state.add_console_message("No iron ore in inventory!".to_string());
                }
            }
            Item::Copper => {
                if game_state.player().inventory().count_copper() > 0 {
                    if let Some(mut furnace_data) = game_state.map().get_furnace_data(container_x, container_y) {
                        let had_copper = game_state.player().inventory().count_copper() > 1;
                        game_state.player_mut().inventory_mut().remove_items(Item::Copper, 1);
                        furnace_data.add_copper();
                        game_state.map_mut().set_furnace_data(container_x, container_y, furnace_data);
                        game_state.add_console_message("Added copper to furnace!".to_string());
                        
                        // If we just removed the last copper, adjust selection
                        if !had_copper {
                            game_state.validate_selection();
                        } else {
                            game_state.validate_selection();
                        }
                    }
                } else {
                    game_state.add_console_message("No copper in inventory!".to_string());
                }
            }
            _ => {
                game_state.add_console_message("Furnace only accepts coal, iron ore, or copper!".to_string());
            }
        }
    } else {
        game_state.add_console_message("No furnace, chest, or drill in front of you!".to_string());
    }
}

// Handle picking up items from ground (or harvesting from furnace if no items)
pub fn handle_pickup(game_state: &mut GameState) {
    // Use cursor position if available, otherwise use position in front of player
    let (check_x, check_y) = if let (Some(cx), Some(cy)) = (game_state.cursor_x(), game_state.cursor_y()) {
        (cx, cy)
    } else {
        let player_x = game_state.player_x();
        let player_y = game_state.player_y();
        let direction = game_state.player_direction();
        
        // Calculate position in front of player
        match direction {
            Direction::North => (player_x, player_y.saturating_sub(1)),
            Direction::South => (player_x, player_y + 1),
            Direction::East => (player_x + 1, player_y),
            Direction::West => (player_x.saturating_sub(1), player_y),
        }
    };
    
    // Check bounds
    if check_x >= game_state.map_width() || check_y >= game_state.map_height() {
        return;
    }
    
    // First, check if there's an item on a belt at this position
    if let Some(belt_item) = game_state.map().get_belt_item(check_x, check_y) {
        let item = belt_item.item();
        let quantity = belt_item.quantity();
        
        // Add all items to inventory
        for _ in 0..quantity {
            game_state.player_mut().add_to_inventory(item.clone());
        }
        
        // Remove the belt item
        game_state.map_mut().remove_belt_item(check_x, check_y);
        
        game_state.add_console_message(format!("Picked up {} {} from belt!", quantity, item.name()));
        game_state.validate_selection();
        return;
    }
    
    // Check if there's a dropped item at this position
    if let Some(index) = game_state.map().get_dropped_item_index_at(check_x, check_y) {
        let dropped_item = game_state.map().dropped_items()[index].clone();
        let item = dropped_item.item();
        let quantity = dropped_item.quantity();
        
        // Add all items to inventory
        for _ in 0..quantity {
            game_state.player_mut().add_to_inventory(item.clone());
        }
        
        // Remove the dropped item
        game_state.map_mut().remove_dropped_item(index);
        
        game_state.add_console_message(format!("Picked up {} {}!", quantity, item.name()));
        game_state.validate_selection();
        return;
    }
    
    // If no item, try furnace harvest (backward compatibility)
    if let Some((furnace_x, furnace_y)) = get_furnace_at_cursor_or_front(game_state) {
        if let Some(mut furnace_data) = game_state.map().get_furnace_data(furnace_x, furnace_y) {
            if furnace_data.iron_plate_count() > 0 {
                // Remove iron plate from furnace
                furnace_data.remove_iron_plate(1);
                game_state.map_mut().set_furnace_data(furnace_x, furnace_y, furnace_data);
                // Add iron plate to player inventory
                game_state.player_mut().add_to_inventory(Item::IronPlate);
                game_state.add_console_message("Harvested iron plate!".to_string());
                // Validate selection after inventory change
                game_state.validate_selection();
            } else if furnace_data.copper_plate_count() > 0 {
                // Remove copper plate from furnace
                furnace_data.remove_copper_plate(1);
                game_state.map_mut().set_furnace_data(furnace_x, furnace_y, furnace_data);
                // Add copper plate to player inventory
                game_state.player_mut().add_to_inventory(Item::CopperPlate);
                game_state.add_console_message("Harvested copper plate!".to_string());
                // Validate selection after inventory change
                game_state.validate_selection();
            } else {
                game_state.add_console_message("No plates in furnace!".to_string());
            }
        }
    }
}

// Handle belt rotation
pub fn handle_belt_rotation(game_state: &mut GameState) {
    // Use cursor position if available, otherwise use position in front of player
    let (rotate_x, rotate_y) = if let (Some(cx), Some(cy)) = (game_state.cursor_x(), game_state.cursor_y()) {
        (cx, cy)
    } else {
        let player_x = game_state.player_x();
        let player_y = game_state.player_y();
        let direction = game_state.player_direction();
        
        // Calculate position in front of player
        match direction {
            Direction::North => (player_x, player_y.saturating_sub(1)),
            Direction::South => (player_x, player_y + 1),
            Direction::East => (player_x + 1, player_y),
            Direction::West => (player_x.saturating_sub(1), player_y),
        }
    };
    
    // Check bounds
    if rotate_x >= game_state.map_width() || rotate_y >= game_state.map_height() {
        return;
    }
    
    // Check if there's a belt, arm, or drill at this position
    if let Some(obj) = game_state.map_mut().get_placeable_object_at_mut(rotate_x, rotate_y) {
        if matches!(obj.placeable_type(), PlaceableType::Belt) || matches!(obj.placeable_type(), PlaceableType::Arm) || matches!(obj.placeable_type(), PlaceableType::Drill) {
            // Rotate direction: North -> East -> South -> West -> North
            let new_direction = match obj.direction() {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            };
            obj.set_direction(new_direction);
            let dir_name = match new_direction {
                Direction::North => "North",
                Direction::South => "South",
                Direction::East => "East",
                Direction::West => "West",
            };
            let obj_name = if matches!(obj.placeable_type(), PlaceableType::Belt) {
                "belt"
            } else if matches!(obj.placeable_type(), PlaceableType::Arm) {
                "arm"
            } else {
                "drill"
            };
            game_state.add_console_message(format!("Rotated {} to {}!", obj_name, dir_name));
        }
    }
}

// Handle dropping items
pub fn handle_drop_item(game_state: &mut GameState) {
    // Use cursor position if available, otherwise use position in front of player
    let (drop_x, drop_y) = if let (Some(cx), Some(cy)) = (game_state.cursor_x(), game_state.cursor_y()) {
        (cx, cy)
    } else {
        let player_x = game_state.player_x();
        let player_y = game_state.player_y();
        let direction = game_state.player_direction();
        
        // Calculate position in front of player
        match direction {
            Direction::North => (player_x, player_y.saturating_sub(1)),
            Direction::South => (player_x, player_y + 1),
            Direction::East => (player_x + 1, player_y),
            Direction::West => (player_x.saturating_sub(1), player_y),
        }
    };
    
    // Check bounds
    if drop_x >= game_state.map_width() || drop_y >= game_state.map_height() {
        game_state.add_console_message("Cannot drop outside map bounds!".to_string());
        return;
    }
    
    // Check if position is valid (not on water, not on furnace)
    if game_state.map().would_collide_with_water(drop_x, drop_y) {
        game_state.add_console_message("Cannot drop on water!".to_string());
        return;
    }
    
    if let Some(placeable_type) = game_state.map().get_placeable_at(drop_x, drop_y) {
        if matches!(placeable_type, PlaceableType::Furnace | PlaceableType::Chest) {
            game_state.add_console_message("Cannot drop on furnace or chest!".to_string());
            return;
        }
    }
    
    // Get selected item
    let selected_item = match game_state.get_selected_item() {
        Some(item) => item,
        None => {
            game_state.add_console_message("No item selected!".to_string());
            return;
        }
    };
    
    // Get quantity of selected item in inventory
    let available_count = match selected_item {
        Item::IronOre => game_state.player().inventory().count_iron_ore(),
        Item::Copper => game_state.player().inventory().count_copper(),
        Item::Stone => game_state.player().inventory().count_stone(),
        Item::Coal => game_state.player().inventory().count_coal(),
        Item::Furnace => game_state.player().inventory().count_furnace(),
        Item::IronPlate => game_state.player().inventory().count_iron_plate(),
        Item::Belt => game_state.player().inventory().count_belt(),
        Item::CopperPlate => game_state.player().inventory().count_copper_plate(),
        Item::Arm => game_state.player().inventory().count_arm(),
        Item::Chest => game_state.player().inventory().count_chest(),
        Item::Drill => game_state.player().inventory().count_drill(),
    };
    
    if available_count == 0 {
        game_state.add_console_message("No items to drop!".to_string());
        return;
    }
    
    // Check if dropping on a belt
    if let Some(placeable_type) = game_state.map().get_placeable_at(drop_x, drop_y) {
        if matches!(placeable_type, PlaceableType::Belt) {
            // Belts can only hold 1 stack at a time
            if game_state.map().has_belt_item(drop_x, drop_y) {
                game_state.add_console_message("Belt already has an item!".to_string());
                return;
            }
            
            // Place only 1 item on the belt (it will move in belt direction every 30 ticks)
            let drop_count = 1;
            game_state.player_mut().inventory_mut().remove_items(selected_item.clone(), drop_count);
            game_state.map_mut().set_belt_item(drop_x, drop_y, DroppedItem::new(drop_x, drop_y, selected_item.clone(), drop_count));
            game_state.add_console_message(format!("Placed {} {} on belt!", drop_count, selected_item.name()));
            game_state.validate_selection();
            return;
        }
    }
    
    // Check if there's already a dropped item of the same type at this position
    if let Some(index) = game_state.map().get_dropped_item_index_at(drop_x, drop_y) {
        let existing_item_type = game_state.map().dropped_items()[index].item();
        if std::mem::discriminant(&existing_item_type) == std::mem::discriminant(&selected_item) {
            // Stack with existing item - drop only 1 item
            let drop_count = 1;
            if let Some(existing_item) = game_state.map_mut().get_dropped_item_mut(index) {
                existing_item.add_quantity(drop_count);
            }
            game_state.player_mut().inventory_mut().remove_items(selected_item.clone(), drop_count);
            game_state.add_console_message(format!("Dropped {} {}!", drop_count, selected_item.name()));
            game_state.validate_selection();
            return;
        } else {
            game_state.add_console_message("Cannot drop on different item!".to_string());
            return;
        }
    }
    
    // Drop only 1 item
    let drop_count = 1;
    game_state.player_mut().inventory_mut().remove_items(selected_item.clone(), drop_count);
    game_state.map_mut().add_dropped_item(DroppedItem::new(drop_x, drop_y, selected_item.clone(), drop_count));
    game_state.add_console_message(format!("Dropped {} {}!", drop_count, selected_item.name()));
    game_state.validate_selection();
}

// Handle picking up placeable objects (delete key)
pub fn handle_pickup_placeable(game_state: &mut GameState) {
    // Use cursor position if available, otherwise use position in front of player
    let (check_x, check_y) = if let (Some(cx), Some(cy)) = (game_state.cursor_x(), game_state.cursor_y()) {
        (cx, cy)
    } else {
        let player_x = game_state.player_x();
        let player_y = game_state.player_y();
        let direction = game_state.player_direction();
        
        // Calculate position in front of player
        match direction {
            Direction::North => (player_x, player_y.saturating_sub(1)),
            Direction::South => (player_x, player_y + 1),
            Direction::East => (player_x + 1, player_y),
            Direction::West => (player_x.saturating_sub(1), player_y),
        }
    };
    
    // Check bounds
    if check_x >= game_state.map_width() || check_y >= game_state.map_height() {
        return;
    }
    
    // Check if there's a placeable object at this position
    if let Some(placeable_type) = game_state.map().get_placeable_at(check_x, check_y) {
        // Collect all items from the placeable object
        match placeable_type {
            PlaceableType::Furnace => {
                // Get all items from furnace
                if let Some(furnace_data) = game_state.map().get_furnace_data(check_x, check_y) {
                    // Add all coal
                    for _ in 0..furnace_data.coal_count() {
                        game_state.player_mut().add_to_inventory(Item::Coal);
                    }
                    // Add all iron ore
                    for _ in 0..furnace_data.iron_ore_count() {
                        game_state.player_mut().add_to_inventory(Item::IronOre);
                    }
                    // Add all iron plates
                    for _ in 0..furnace_data.iron_plate_count() {
                        game_state.player_mut().add_to_inventory(Item::IronPlate);
                    }
                    // Add all copper
                    for _ in 0..furnace_data.copper_count() {
                        game_state.player_mut().add_to_inventory(Item::Copper);
                    }
                    // Add all copper plates
                    for _ in 0..furnace_data.copper_plate_count() {
                        game_state.player_mut().add_to_inventory(Item::CopperPlate);
                    }
                }
                // Add furnace back to inventory
                game_state.player_mut().add_to_inventory(Item::Furnace);
                game_state.add_console_message("Picked up furnace with all contents!".to_string());
            }
            PlaceableType::Belt => {
                // Get item from belt if any
                if let Some(belt_item) = game_state.map().get_belt_item(check_x, check_y) {
                    let item = belt_item.item();
                    let quantity = belt_item.quantity();
                    for _ in 0..quantity {
                        game_state.player_mut().add_to_inventory(item.clone());
                    }
                    game_state.map_mut().remove_belt_item(check_x, check_y);
                }
                // Add belt back to inventory
                game_state.player_mut().add_to_inventory(Item::Belt);
                game_state.add_console_message("Picked up belt!".to_string());
            }
            PlaceableType::Arm => {
                // Arms don't store items, just add arm back to inventory
                game_state.player_mut().add_to_inventory(Item::Arm);
                game_state.add_console_message("Picked up arm!".to_string());
            }
            PlaceableType::Chest => {
                // Get all items from chest
                if let Some(chest_data) = game_state.map().get_chest_data(check_x, check_y) {
                    let items = chest_data.get_all_items();
                    for (index, item) in items.iter().enumerate() {
                        let quantity = chest_data.get_item_quantity(index);
                        for _ in 0..quantity {
                            game_state.player_mut().add_to_inventory(item.clone());
                        }
                    }
                }
                // Add chest back to inventory
                game_state.player_mut().add_to_inventory(Item::Chest);
                game_state.add_console_message("Picked up chest with all contents!".to_string());
            }
            PlaceableType::Drill => {
                // Get all coal from drill
                if let Some(drill_data) = game_state.map().get_drill_data(check_x, check_y) {
                    for _ in 0..drill_data.coal_count() {
                        game_state.player_mut().add_to_inventory(Item::Coal);
                    }
                }
                // Add drill back to inventory
                game_state.player_mut().add_to_inventory(Item::Drill);
                game_state.add_console_message("Picked up drill with all contents!".to_string());
            }
        }
        
        // Remove the placeable object from the map
        game_state.map_mut().remove_placeable(check_x, check_y);
        
        // Validate selection after inventory change
        game_state.validate_selection();
    }
}

// Process furnaces each tick
pub fn handle_furnace_tick_processing(game_state: &mut GameState) {
    // Get list of furnace positions first to avoid borrowing issues
    let furnace_positions: Vec<(u32, u32)> = game_state.map().placeable_objects()
        .iter()
        .filter(|obj| matches!(obj.placeable_type(), PlaceableType::Furnace))
        .map(|obj| (obj.x(), obj.y()))
        .collect();
    
    // Iterate through all furnaces
    for (x, y) in furnace_positions {
        if let Some(mut furnace_data) = game_state.map().get_furnace_data(x, y) {
            // Store counts before processing to detect what was produced
            let iron_before = furnace_data.iron_plate_count();
            let copper_before = furnace_data.copper_plate_count();
            
            // Start processing if we have materials and not already processing
            furnace_data.start_processing_if_able();
            
            // Process one tick
            let completed = furnace_data.process_tick();
            
            // Check what was produced by comparing counts
            if completed {
                let iron_after = furnace_data.iron_plate_count();
                let copper_after = furnace_data.copper_plate_count();
                
                if iron_after > iron_before {
                    game_state.add_console_message(format!("Furnace at ({}, {}) produced an iron plate!", x, y));
                } else if copper_after > copper_before {
                    game_state.add_console_message(format!("Furnace at ({}, {}) produced a copper plate!", x, y));
                }
            }
            
            game_state.map_mut().set_furnace_data(x, y, furnace_data);
        }
    }
}

// Helper function to get next position in a direction
fn get_next_position_in_direction(x: u32, y: u32, direction: Direction) -> (u32, u32) {
    match direction {
        Direction::North => (x, y.saturating_sub(1)),
        Direction::South => (x, y + 1),
        Direction::East => (x + 1, y),
        Direction::West => (x.saturating_sub(1), y),
    }
}

// Process belts each tick - move player and items in belt direction every 30 ticks
pub fn handle_belt_tick_processing(game_state: &mut GameState) {
    let player_x = game_state.player_x();
    let player_y = game_state.player_y();
    
    // Check if player is standing on a belt
    if let Some(obj) = game_state.map().get_placeable_object_at(player_x, player_y) {
        if matches!(obj.placeable_type(), PlaceableType::Belt) {
            // Move player in belt direction every 30 ticks
            if game_state.current_tick() % 30 == 0 {
                let belt_direction = obj.direction();
                let (new_x, new_y) = get_next_position_in_direction(player_x, player_y, belt_direction);
                
                // Check bounds and collisions
                if new_x < game_state.map_width() && new_y < game_state.map_height()
                    && !game_state.map().would_collide_with_water(new_x, new_y)
                    && !game_state.map().would_collide_with_placeable(new_x, new_y) {
                    game_state.set_player_position(new_x, new_y);
                    game_state.set_player_direction(belt_direction);
                }
            }
        }
    }
    
    // Move items on belts
    if game_state.current_tick() % 30 == 0 {
        // Process belt items (items directly on belts)
        // Collect belt positions with their directions for proper ordering
        let mut belt_data: Vec<(u32, u32, Direction)> = game_state.map().placeable_objects()
            .iter()
            .filter(|obj| matches!(obj.placeable_type(), PlaceableType::Belt))
            .map(|obj| (obj.x(), obj.y(), obj.direction()))
            .collect();
        
        // Sort belts in reverse order of their direction to ensure local processing
        // This prevents items from moving multiple tiles in one tick
        // For East: process right-to-left (descending x)
        // For West: process left-to-right (ascending x)
        // For South: process bottom-to-top (descending y)
        // For North: process top-to-bottom (ascending y)
        belt_data.sort_by(|a, b| {
            match (a.2, b.2) {
                (Direction::East, Direction::East) => b.0.cmp(&a.0), // Reverse x order
                (Direction::West, Direction::West) => a.0.cmp(&b.0),  // Normal x order
                (Direction::South, Direction::South) => b.1.cmp(&a.1), // Reverse y order
                (Direction::North, Direction::North) => a.1.cmp(&b.1), // Normal y order
                // If directions differ, group by direction first, then by position
                _ => {
                    let dir_order_a = match a.2 {
                        Direction::North => 0,
                        Direction::South => 1,
                        Direction::East => 2,
                        Direction::West => 3,
                    };
                    let dir_order_b = match b.2 {
                        Direction::North => 0,
                        Direction::South => 1,
                        Direction::East => 2,
                        Direction::West => 3,
                    };
                    dir_order_a.cmp(&dir_order_b)
                        .then_with(|| match a.2 {
                            Direction::East => b.0.cmp(&a.0),
                            Direction::West => a.0.cmp(&b.0),
                            Direction::South => b.1.cmp(&a.1),
                            Direction::North => a.1.cmp(&b.1),
                        })
                }
            }
        });
        
        // Process each belt in the sorted order
        for (belt_x, belt_y, belt_direction) in belt_data {
            // Check if there's an item on this belt
            if let Some(belt_item) = game_state.map().get_belt_item(belt_x, belt_y) {
                let mut belt_item = belt_item.clone();
                let (next_x, next_y) = get_next_position_in_direction(belt_x, belt_y, belt_direction);
                
                // Check if next position is valid and free
                // Belts cannot directly put items in furnaces or chests - only arms can do that
                if next_x < game_state.map_width() && next_y < game_state.map_height()
                    && !game_state.map().would_collide_with_water(next_x, next_y)
                    && !game_state.map().has_belt_item(next_x, next_y)
                    && game_state.map().get_dropped_item_index_at(next_x, next_y).is_none() {
                    
                    // Check if next position is a furnace or chest - belts cannot move items there
                    if let Some(next_obj) = game_state.map().get_placeable_object_at(next_x, next_y) {
                        if matches!(next_obj.placeable_type(), PlaceableType::Furnace | PlaceableType::Chest) {
                            // Belt cannot put items in furnaces or chests - item stays on belt (blocked)
                            continue;
                        }
                    }
                    
                    // Check if next position would collide with a placeable (other than belt)
                    if !game_state.map().would_collide_with_placeable(next_x, next_y) {
                        // Check if next position is a belt
                        if let Some(next_obj) = game_state.map().get_placeable_object_at(next_x, next_y) {
                            if matches!(next_obj.placeable_type(), PlaceableType::Belt) {
                                // Move to next belt
                                belt_item.set_position(next_x, next_y);
                                game_state.map_mut().remove_belt_item(belt_x, belt_y);
                                game_state.map_mut().set_belt_item(next_x, next_y, belt_item);
                            } else {
                                // Not a belt, move to dropped items
                                game_state.map_mut().remove_belt_item(belt_x, belt_y);
                                game_state.map_mut().add_dropped_item(belt_item);
                            }
                        } else {
                            // Empty space, move to dropped items
                            belt_item.set_position(next_x, next_y);
                            game_state.map_mut().remove_belt_item(belt_x, belt_y);
                            game_state.map_mut().add_dropped_item(belt_item);
                        }
                    }
                }
                // If blocked, item stays on belt
            }
        }
        
        // Process dropped items that are on belts - move entire stacks onto belts
        // This handles the case where a stack is dropped on a belt position
        let mut items_to_feed: Vec<(usize, u32, u32)> = Vec::new();
        
        for (index, item) in game_state.map().dropped_items().iter().enumerate() {
            let item_x = item.x();
            let item_y = item.y();
            
            // Check if item is on a belt (but not already a belt item)
            if let Some(obj) = game_state.map().get_placeable_object_at(item_x, item_y) {
                if matches!(obj.placeable_type(), PlaceableType::Belt) && !game_state.map().has_belt_item(item_x, item_y) {
                    // Try to feed this entire stack onto the belt
                    items_to_feed.push((index, item_x, item_y));
                }
            }
        }
        
        // Feed entire stacks onto belts (only if belt is empty and can move)
        for (index, belt_x, belt_y) in items_to_feed {
            if !game_state.map().has_belt_item(belt_x, belt_y) {
                if let Some(belt_obj) = game_state.map().get_placeable_object_at(belt_x, belt_y) {
                    let belt_direction = belt_obj.direction();
                    let (next_x, next_y) = get_next_position_in_direction(belt_x, belt_y, belt_direction);
                    
                    // Check if belt can move item (next position must be free)
                    if next_x < game_state.map_width() && next_y < game_state.map_height()
                        && !game_state.map().would_collide_with_water(next_x, next_y)
                        && !game_state.map().would_collide_with_placeable(next_x, next_y)
                        && !game_state.map().has_belt_item(next_x, next_y)
                        && game_state.map().get_dropped_item_index_at(next_x, next_y).is_none() {
                        
                        // Move entire stack onto belt
                        if let Some(dropped_item) = game_state.map_mut().get_dropped_item_mut(index) {
                            if dropped_item.quantity() > 0 {
                                let item_type = dropped_item.item();
                                let quantity = dropped_item.quantity();
                                
                                // Remove the entire stack from dropped items
                                dropped_item.remove_quantity(quantity);
                                
                                // Place entire stack on belt
                                game_state.map_mut().set_belt_item(belt_x, belt_y, DroppedItem::new(belt_x, belt_y, item_type, quantity));
                                
                                // Stack will be removed below if quantity is now 0
                            }
                        }
                    }
                }
            }
        }
        
        // Remove items with zero quantity
        let mut indices_to_remove: Vec<usize> = Vec::new();
        for (index, item) in game_state.map().dropped_items().iter().enumerate() {
            if item.quantity() == 0 {
                indices_to_remove.push(index);
            }
        }
        // Remove in reverse order to maintain indices
        indices_to_remove.sort_by(|a, b| b.cmp(a));
        for index in indices_to_remove {
            game_state.map_mut().remove_dropped_item(index);
        }
    }
}

// Process arms each tick - move items from below to above every 120 ticks
pub fn handle_arm_tick_processing(game_state: &mut GameState) {
    
    // Get list of arm positions first to avoid borrowing issues
    let arm_positions: Vec<(u32, u32)> = game_state.map().placeable_objects()
        .iter()
        .filter(|obj| matches!(obj.placeable_type(), PlaceableType::Arm))
        .map(|obj| (obj.x(), obj.y()))
        .collect();
    
    // Iterate through all arms
    for (arm_x, arm_y) in arm_positions {
        if let Some(mut arm_data) = game_state.map().get_arm_data(arm_x, arm_y) {
            // Process one tick - returns true if it's time to act
            let should_act = arm_data.process_tick();
            game_state.map_mut().set_arm_data(arm_x, arm_y, arm_data);
            
            if should_act {
                // Get the arm object to get its direction
                if let Some(arm_obj) = game_state.map().get_placeable_object_at(arm_x, arm_y) {
                    let direction = arm_obj.direction();
                    
                    // Calculate source position (below arm) and target position (above arm)
                    // "Below" means opposite of direction, "above" means in direction
                    let (source_x, source_y) = match direction {
                        Direction::North => (arm_x, arm_y + 1), // Source is south (below)
                        Direction::South => (arm_x, arm_y.saturating_sub(1)), // Source is north (below)
                        Direction::East => (arm_x.saturating_sub(1), arm_y), // Source is west (below)
                        Direction::West => (arm_x + 1, arm_y), // Source is east (below)
                    };
                    
                    let (target_x, target_y) = match direction {
                        Direction::North => (arm_x, arm_y.saturating_sub(1)), // Target is north (above)
                        Direction::South => (arm_x, arm_y + 1), // Target is south (above)
                        Direction::East => (arm_x + 1, arm_y), // Target is east (above)
                        Direction::West => (arm_x.saturating_sub(1), arm_y), // Target is west (above)
                    };
                    
                    // Check bounds
                    if source_x < game_state.map_width() && source_y < game_state.map_height()
                        && target_x < game_state.map_width() && target_y < game_state.map_height() {
                        
                        // First, check if target is occupied (before taking item from source)
                        // Target is occupied if:
                        // - It's a belt and has a belt item
                        // - It's empty space and has a dropped item
                        // Chest/furnace/drill can accept items, so they're not considered "occupied"
                        let target_occupied = {
                            if let Some(target_obj) = game_state.map().get_placeable_object_at(target_x, target_y) {
                                if matches!(target_obj.placeable_type(), PlaceableType::Belt) {
                                    // Belt is occupied if it has an item
                                    game_state.map().has_belt_item(target_x, target_y)
                                } else {
                                    // Chest/furnace/drill can accept items, not occupied
                                    false
                                }
                            } else {
                                // Empty space is occupied if it has a dropped item
                                game_state.map().get_dropped_item_index_at(target_x, target_y).is_some()
                            }
                        };
                        
                        // Only proceed if target is not occupied
                        if !target_occupied {
                            // Check if source position has an item (furnace, chest, belt, or dropped item)
                            let mut item_to_move: Option<DroppedItem> = None;
                            
                            // First, check if source is a chest - try to take an item from it
                            if let Some(placeable_type) = game_state.map().get_placeable_at(source_x, source_y) {
                                if matches!(placeable_type, PlaceableType::Chest) {
                                    if let Some(mut chest_data) = game_state.map().get_chest_data(source_x, source_y) {
                                        // Get first available item from chest
                                        let items = chest_data.get_all_items();
                                        if let Some(item) = items.first() {
                                            let quantity = chest_data.get_item_quantity(0);
                                            if quantity > 0 {
                                                let item_to_take = item.clone();
                                                chest_data.remove_item(item_to_take.clone(), 1);
                                                game_state.map_mut().set_chest_data(source_x, source_y, chest_data);
                                                item_to_move = Some(DroppedItem::new(source_x, source_y, item_to_take, 1));
                                            }
                                        }
                                    }
                                }
                                // Then check if source is a furnace - try to harvest plates from it
                                else if matches!(placeable_type, PlaceableType::Furnace) {
                                    if let Some(mut furnace_data) = game_state.map().get_furnace_data(source_x, source_y) {
                                        // Try to harvest iron plate first, then copper plate
                                        if furnace_data.iron_plate_count() > 0 {
                                            furnace_data.remove_iron_plate(1);
                                            game_state.map_mut().set_furnace_data(source_x, source_y, furnace_data);
                                            item_to_move = Some(DroppedItem::new(source_x, source_y, Item::IronPlate, 1));
                                        } else if furnace_data.copper_plate_count() > 0 {
                                            furnace_data.remove_copper_plate(1);
                                            game_state.map_mut().set_furnace_data(source_x, source_y, furnace_data);
                                            item_to_move = Some(DroppedItem::new(source_x, source_y, Item::CopperPlate, 1));
                                        }
                                    }
                                }
                            }
                            
                            // If no item from furnace, check for belt items or dropped items
                            if item_to_move.is_none() {
                                // Check if there's a belt item at source
                                if let Some(belt_item) = game_state.map().get_belt_item(source_x, source_y) {
                                    item_to_move = Some(belt_item.clone());
                                    game_state.map_mut().remove_belt_item(source_x, source_y);
                                }
                                // Then check if there's a dropped item at source
                                else if let Some(index) = game_state.map().get_dropped_item_index_at(source_x, source_y) {
                                    item_to_move = Some(game_state.map().dropped_items()[index].clone());
                                    game_state.map_mut().remove_dropped_item(index);
                                }
                            }
                            
                            // If we found an item, try to place it at target
                            if let Some(mut item) = item_to_move {
                            let item_type = item.item();
                            
                            // Check if target is a chest - try to place item into it
                            if let Some(placeable_type) = game_state.map().get_placeable_at(target_x, target_y) {
                                if matches!(placeable_type, PlaceableType::Chest) {
                                    // Chest accepts any item
                                    if let Some(mut chest_data) = game_state.map().get_chest_data(target_x, target_y) {
                                        let quantity = item.quantity();
                                        if quantity > 0 {
                                            // Place 1 item at a time into chest
                                            chest_data.add_item(item_type.clone(), 1);
                                            game_state.map_mut().set_chest_data(target_x, target_y, chest_data);
                                            
                                            // If there are more items, put them back at source
                                            if quantity > 1 {
                                                item.remove_quantity(1);
                                                item.set_position(source_x, source_y);
                                                game_state.map_mut().add_dropped_item(item);
                                            }
                                            // Item successfully placed in chest, move to next arm
                                            continue;
                                        }
                                    }
                                }
                                // Check if target is a furnace - try to place item into it
                                else if matches!(placeable_type, PlaceableType::Furnace) {
                                    // Check if item can be placed in furnace (coal, iron ore, or copper)
                                    match item_type {
                                        Item::Coal | Item::IronOre | Item::Copper => {
                                            if let Some(mut furnace_data) = game_state.map().get_furnace_data(target_x, target_y) {
                                                // Only place 1 item at a time into furnace
                                                let quantity = item.quantity();
                                                if quantity > 0 {
                                                    match item_type {
                                                        Item::Coal => {
                                                            furnace_data.add_coal();
                                                            game_state.map_mut().set_furnace_data(target_x, target_y, furnace_data);
                                                        }
                                                        Item::IronOre => {
                                                            furnace_data.add_iron_ore();
                                                            game_state.map_mut().set_furnace_data(target_x, target_y, furnace_data);
                                                        }
                                                        Item::Copper => {
                                                            furnace_data.add_copper();
                                                            game_state.map_mut().set_furnace_data(target_x, target_y, furnace_data);
                                                        }
                                                        _ => {}
                                                    }
                                                    
                                                    // If there are more items, put them back at source
                                                    if quantity > 1 {
                                                        item.remove_quantity(1);
                                                        item.set_position(source_x, source_y);
                                                        game_state.map_mut().add_dropped_item(item);
                                                    }
                                                    // Item successfully placed in furnace, move to next arm
                                                    continue;
                                                }
                                            }
                                        }
                                        _ => {
                                            // Item cannot be placed in furnace, fall through to normal placement
                                        }
                                    }
                                }
                                // Check if target is a drill - try to place coal into it
                                else if matches!(placeable_type, PlaceableType::Drill) {
                                    // Drill only accepts coal
                                    if matches!(item_type, Item::Coal) {
                                        if let Some(mut drill_data) = game_state.map().get_drill_data(target_x, target_y) {
                                            // Only place 1 item at a time into drill
                                            let quantity = item.quantity();
                                            if quantity > 0 {
                                                drill_data.add_coal();
                                                game_state.map_mut().set_drill_data(target_x, target_y, drill_data);
                                                
                                                // If there are more items, put them back at source
                                                if quantity > 1 {
                                                    item.remove_quantity(1);
                                                    item.set_position(source_x, source_y);
                                                    game_state.map_mut().add_dropped_item(item);
                                                }
                                                // Item successfully placed in drill, move to next arm
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                            
                            // If target is not a furnace, or item cannot be placed in furnace, use normal placement logic
                            // Check if target position is valid
                            if !game_state.map().would_collide_with_water(target_x, target_y) {
                                // Check if target is a belt
                                if let Some(target_obj) = game_state.map().get_placeable_object_at(target_x, target_y) {
                                    if matches!(target_obj.placeable_type(), PlaceableType::Belt) {
                                        // Place on belt if belt is empty
                                        if !game_state.map().has_belt_item(target_x, target_y) {
                                            item.set_position(target_x, target_y);
                                            game_state.map_mut().set_belt_item(target_x, target_y, item);
                                        } else {
                                            // Belt is full, put item back at source
                                            item.set_position(source_x, source_y);
                                            game_state.map_mut().add_dropped_item(item);
                                        }
                                    } else {
                                        // Not a belt, place as dropped item
                                        item.set_position(target_x, target_y);
                                        game_state.map_mut().add_dropped_item(item);
                                    }
                                } else {
                                    // Empty space, place as dropped item
                                    item.set_position(target_x, target_y);
                                    game_state.map_mut().add_dropped_item(item);
                                }
                            } else {
                                // Target is water, put item back at source
                                item.set_position(source_x, source_y);
                                game_state.map_mut().add_dropped_item(item);
                            }
                        }
                        }
                    }
                }
            }
        }
    }
}

// Process drills each tick - produces items every 60 ticks if placed over resources
pub fn handle_drill_tick_processing(game_state: &mut GameState) {
    // Get list of drill positions first to avoid borrowing issues
    let drill_positions: Vec<(u32, u32)> = game_state.map().placeable_objects()
        .iter()
        .filter(|obj| matches!(obj.placeable_type(), PlaceableType::Drill))
        .map(|obj| (obj.x(), obj.y()))
        .collect();
    
    // Iterate through all drills
    for (drill_x, drill_y) in drill_positions {
        if let Some(mut drill_data) = game_state.map().get_drill_data(drill_x, drill_y) {
            // Only start processing if not already processing
            if drill_data.processing_ticks_remaining() == 0 {
                // Check if target position is free before starting a new cycle
                if let Some(drill_obj) = game_state.map().get_placeable_object_at(drill_x, drill_y) {
                    let direction = drill_obj.direction();
                    
                    // Calculate target position (below drill in its direction)
                    let (target_x, target_y) = match direction {
                        Direction::North => (drill_x, drill_y.saturating_sub(1)),
                        Direction::South => (drill_x, drill_y + 1),
                        Direction::East => (drill_x + 1, drill_y),
                        Direction::West => (drill_x.saturating_sub(1), drill_y),
                    };
                    
                    // Determine what item will be produced (based on resource being mined)
                    let item_to_produce = game_state.map().get_resource_at(drill_x, drill_y)
                        .and_then(|resource_type| match resource_type {
                            ResourceType::IronOre => Some(Item::IronOre),
                            ResourceType::Coal => Some(Item::Coal),
                            ResourceType::Copper => Some(Item::Copper),
                            ResourceType::Stone => None,
                        });
                    
                    // Check if target position can accept the item
                    let target_can_accept = if target_x < game_state.map_width() 
                        && target_y < game_state.map_height()
                        && !game_state.map().would_collide_with_water(target_x, target_y) {
                        
                        if let Some(placeable_type) = game_state.map().get_placeable_at(target_x, target_y) {
                            // Check if target placeable can accept the item
                            if let Some(item) = item_to_produce {
                                match placeable_type {
                                    PlaceableType::Chest => {
                                        // Chests accept any item
                                        true
                                    }
                                    PlaceableType::Furnace => {
                                        // Furnaces accept coal, iron ore, or copper
                                        matches!(item, Item::Coal | Item::IronOre | Item::Copper)
                                    }
                                    PlaceableType::Drill => {
                                        // Drills accept coal
                                        matches!(item, Item::Coal)
                                    }
                                    _ => false,
                                }
                            } else {
                                false
                            }
                        } else {
                            // No placeable, check if space is free (no belt item, no dropped item)
                            !game_state.map().has_belt_item(target_x, target_y)
                                && game_state.map().get_dropped_item_index_at(target_x, target_y).is_none()
                        }
                    } else {
                        false
                    };
                    
                    // Only start processing if target can accept the item and we have coal
                    if target_can_accept {
                        drill_data.start_processing_if_able();
                    }
                }
            }
            
            // Process one tick
            let completed = drill_data.process_tick();
            game_state.map_mut().set_drill_data(drill_x, drill_y, drill_data);
            
            if completed {
                // Get the drill object to get its direction
                if let Some(drill_obj) = game_state.map().get_placeable_object_at(drill_x, drill_y) {
                    let direction = drill_obj.direction();
                    
                    // Calculate target position (below drill in its direction)
                    let (target_x, target_y) = match direction {
                        Direction::North => (drill_x, drill_y.saturating_sub(1)),
                        Direction::South => (drill_x, drill_y + 1),
                        Direction::East => (drill_x + 1, drill_y),
                        Direction::West => (drill_x.saturating_sub(1), drill_y),
                    };
                    
                    // Check bounds
                    if target_x < game_state.map_width() && target_y < game_state.map_height() {
                        // Check if drill is placed over a resource (iron, coal, or copper)
                        if let Some(resource_type) = game_state.map().get_resource_at(drill_x, drill_y) {
                            match resource_type {
                                ResourceType::IronOre | ResourceType::Coal | ResourceType::Copper => {
                                    // Produce the corresponding item
                                    let item = match resource_type {
                                        ResourceType::IronOre => Item::IronOre,
                                        ResourceType::Coal => Item::Coal,
                                        ResourceType::Copper => Item::Copper,
                                        ResourceType::Stone => Item::Stone, // Shouldn't happen but handle it
                                    };
                                    
                                    // Check if target position is valid (not on water)
                                    if !game_state.map().would_collide_with_water(target_x, target_y) {
                                        // Check if target is a placeable that can accept items
                                        if let Some(target_obj) = game_state.map().get_placeable_object_at(target_x, target_y) {
                                            match target_obj.placeable_type() {
                                                PlaceableType::Chest => {
                                                    // Chest accepts any item
                                                    if let Some(mut chest_data) = game_state.map().get_chest_data(target_x, target_y) {
                                                        chest_data.add_item(item, 1);
                                                        game_state.map_mut().set_chest_data(target_x, target_y, chest_data);
                                                    }
                                                }
                                                PlaceableType::Furnace => {
                                                    // Furnace accepts coal, iron ore, or copper
                                                    if matches!(item, Item::Coal | Item::IronOre | Item::Copper) {
                                                        if let Some(mut furnace_data) = game_state.map().get_furnace_data(target_x, target_y) {
                                                            match item {
                                                                Item::Coal => {
                                                                    furnace_data.add_coal();
                                                                }
                                                                Item::IronOre => {
                                                                    furnace_data.add_iron_ore();
                                                                }
                                                                Item::Copper => {
                                                                    furnace_data.add_copper();
                                                                }
                                                                _ => {}
                                                            }
                                                            game_state.map_mut().set_furnace_data(target_x, target_y, furnace_data);
                                                        }
                                                    }
                                                }
                                                PlaceableType::Drill => {
                                                    // Drill accepts coal
                                                    if matches!(item, Item::Coal) {
                                                        if let Some(mut target_drill_data) = game_state.map().get_drill_data(target_x, target_y) {
                                                            target_drill_data.add_coal();
                                                            game_state.map_mut().set_drill_data(target_x, target_y, target_drill_data);
                                                        }
                                                    }
                                                }
                                                PlaceableType::Belt => {
                                                    // Place on belt if belt is empty
                                                    if !game_state.map().has_belt_item(target_x, target_y) {
                                                        game_state.map_mut().set_belt_item(target_x, target_y, DroppedItem::new(target_x, target_y, item, 1));
                                                    }
                                                }
                                                _ => {
                                                    // Not a placeable that accepts items, place as dropped item
                                                    if !game_state.map().would_collide_with_placeable(target_x, target_y) {
                                                        game_state.map_mut().add_dropped_item(DroppedItem::new(target_x, target_y, item, 1));
                                                    }
                                                }
                                            }
                                        } else {
                                            // Empty space, place as dropped item
                                            game_state.map_mut().add_dropped_item(DroppedItem::new(target_x, target_y, item, 1));
                                        }
                                    }
                                }
                                ResourceType::Stone => {
                                    // Stone is not drillable
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

