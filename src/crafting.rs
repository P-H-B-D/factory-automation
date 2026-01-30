use crate::types::Item;
use crate::game_state::GameState;

// Crafting recipe struct
pub struct CraftingRecipe {
    pub result: Item,
    pub ingredients: Vec<(Item, u32)>,
}

// Get crafting recipes - scalable system for future recipes
pub fn get_crafting_recipes() -> Vec<CraftingRecipe> {
    vec![
        CraftingRecipe {
            result: Item::Furnace,
            ingredients: vec![(Item::Stone, 5)],
        },
        CraftingRecipe {
            result: Item::Belt,
            ingredients: vec![(Item::IronPlate, 1)],
        },
        CraftingRecipe {
            result: Item::Arm,
            ingredients: vec![(Item::CopperPlate, 1)],
        },
        CraftingRecipe {
            result: Item::Chest,
            ingredients: vec![(Item::IronPlate, 1)],
        },
        CraftingRecipe {
            result: Item::Drill,
            ingredients: vec![(Item::IronOre, 5)],
        },
    ]
}

// Helper function to get item count from inventory
fn get_item_count(game_state: &GameState, item_type: &Item) -> u32 {
    match item_type {
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
    }
}

// Helper function to get item name
fn get_item_name(item_type: &Item) -> &'static str {
    match item_type {
        Item::IronOre => "Iron Ore",
        Item::Copper => "Copper",
        Item::Stone => "Stone",
        Item::Coal => "Coal",
        Item::Furnace => "Furnace",
        Item::IronPlate => "Iron Plate",
        Item::Belt => "Belt",
        Item::CopperPlate => "Copper Plate",
        Item::Arm => "Arm",
        Item::Chest => "Chest",
        Item::Drill => "Drill",
    }
}

// Handle belt crafting
pub fn handle_belt_crafting(game_state: &mut GameState) {
    let recipes = get_crafting_recipes();
    
    // Find belt recipe
    if let Some(recipe) = recipes.iter().find(|r| matches!(r.result, Item::Belt)) {
        // Check if player has all required ingredients
        let mut has_all = true;
        for (item_type, required_count) in &recipe.ingredients {
            let count = get_item_count(game_state, item_type);
            if count < *required_count {
                has_all = false;
                let message = format!("Not enough resources! Need {} {} (have {})", required_count, get_item_name(item_type), count);
                game_state.add_console_message(message);
                break;
            }
        }
        
        if has_all {
            // Remove ingredients
            for (item_type, count) in &recipe.ingredients {
                game_state.player_mut().inventory_mut().remove_items(item_type.clone(), *count);
            }
            
            // Add result to inventory
            game_state.player_mut().add_to_inventory(recipe.result.clone());
            
            // Validate selection after inventory change
            game_state.validate_selection();
            
            game_state.add_console_message("Crafted Belt!".to_string());
        }
    }
}

// Handle arm crafting
pub fn handle_arm_crafting(game_state: &mut GameState) {
    let recipes = get_crafting_recipes();
    
    // Find arm recipe
    if let Some(recipe) = recipes.iter().find(|r| matches!(r.result, Item::Arm)) {
        // Check if player has all required ingredients
        let mut has_all = true;
        for (item_type, required_count) in &recipe.ingredients {
            let count = get_item_count(game_state, item_type);
            if count < *required_count {
                has_all = false;
                let message = format!("Not enough resources! Need {} {} (have {})", required_count, get_item_name(item_type), count);
                game_state.add_console_message(message);
                break;
            }
        }
        
        if has_all {
            // Remove ingredients
            for (item_type, count) in &recipe.ingredients {
                game_state.player_mut().inventory_mut().remove_items(item_type.clone(), *count);
            }
            
            // Add result to inventory
            game_state.player_mut().add_to_inventory(recipe.result.clone());
            
            // Validate selection after inventory change
            game_state.validate_selection();
            
            game_state.add_console_message("Crafted Arm!".to_string());
        }
    }
}

// Handle drill crafting
pub fn handle_drill_crafting(game_state: &mut GameState) {
    let recipes = get_crafting_recipes();
    
    // Find drill recipe
    if let Some(recipe) = recipes.iter().find(|r| matches!(r.result, Item::Drill)) {
        // Check if player has all required ingredients
        let mut has_all = true;
        for (item_type, required_count) in &recipe.ingredients {
            let count = get_item_count(game_state, item_type);
            if count < *required_count {
                has_all = false;
                let message = format!("Not enough resources! Need {} {} (have {})", required_count, get_item_name(item_type), count);
                game_state.add_console_message(message);
                break;
            }
        }
        
        if has_all {
            // Remove ingredients
            for (item_type, count) in &recipe.ingredients {
                game_state.player_mut().inventory_mut().remove_items(item_type.clone(), *count);
            }
            
            // Add result to inventory
            game_state.player_mut().add_to_inventory(recipe.result.clone());
            
            // Validate selection after inventory change
            game_state.validate_selection();
            
            game_state.add_console_message("Crafted Drill!".to_string());
        }
    }
}

// Handle chest crafting
pub fn handle_chest_crafting(game_state: &mut GameState) {
    let recipes = get_crafting_recipes();
    
    // Find chest recipe
    if let Some(recipe) = recipes.iter().find(|r| matches!(r.result, Item::Chest)) {
        // Check if player has all required ingredients
        let mut has_all = true;
        for (item_type, required_count) in &recipe.ingredients {
            let count = get_item_count(game_state, item_type);
            if count < *required_count {
                has_all = false;
                let message = format!("Not enough resources! Need {} {} (have {})", required_count, get_item_name(item_type), count);
                game_state.add_console_message(message);
                break;
            }
        }
        
        if has_all {
            // Remove ingredients
            for (item_type, count) in &recipe.ingredients {
                game_state.player_mut().inventory_mut().remove_items(item_type.clone(), *count);
            }
            
            // Add result to inventory
            game_state.player_mut().add_to_inventory(recipe.result.clone());
            
            // Validate selection after inventory change
            game_state.validate_selection();
            
            game_state.add_console_message("Crafted Chest!".to_string());
        }
    }
}

// Handle crafting
pub fn handle_crafting(game_state: &mut GameState) {
    let recipes = get_crafting_recipes();
    
    // Try to craft a furnace (first recipe for now)
    if let Some(recipe) = recipes.first() {
        // Check if player has all required ingredients
        let mut has_all = true;
        for (item_type, required_count) in &recipe.ingredients {
            let count = get_item_count(game_state, item_type);
            if count < *required_count {
                has_all = false;
                let message = format!("Not enough resources! Need {} {} (have {})", required_count, get_item_name(item_type), count);
                game_state.add_console_message(message);
                break;
            }
        }
        
        if has_all {
            // Remove ingredients
            for (item_type, count) in &recipe.ingredients {
                game_state.player_mut().inventory_mut().remove_items(item_type.clone(), *count);
            }
            
            // Add result to inventory
            game_state.player_mut().add_to_inventory(recipe.result.clone());
            
            // Validate selection after inventory change
            game_state.validate_selection();
            
            let message = format!("Crafted {}!", get_item_name(&recipe.result));
            game_state.add_console_message(message);
        }
    }
}

