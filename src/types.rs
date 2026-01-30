use wasm_bindgen::prelude::*;

// Item enum for inventory
#[wasm_bindgen]
#[derive(Clone)]
pub enum Item {
    IronOre,
    Copper,
    Stone,
    Coal,
    Furnace,
    IronPlate,
    Belt,
    CopperPlate,
    Arm,
    Chest,
    Drill,
}

// Direction enum
#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn value(&self) -> u32 {
        match self {
            Direction::North => 0,
            Direction::South => 1,
            Direction::East => 2,
            Direction::West => 3,
        }
    }
}

// Placeable object enum
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum PlaceableType {
    Furnace,
    Belt,
    Arm,
    Chest,
    Drill,
}

impl PlaceableType {
    pub fn value(&self) -> u32 {
        match self {
            PlaceableType::Furnace => 0,
            PlaceableType::Belt => 1,
            PlaceableType::Arm => 2,
            PlaceableType::Chest => 3,
            PlaceableType::Drill => 4,
        }
    }
}

// Resource type enum
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum ResourceType {
    IronOre,
    Copper,
    Stone,
    Coal,
}

impl ResourceType {
    pub fn value(&self) -> u32 {
        match self {
            ResourceType::IronOre => 0,
            ResourceType::Copper => 1,
            ResourceType::Stone => 2,
            ResourceType::Coal => 3,
        }
    }
}

impl Item {
    pub fn name(&self) -> &'static str {
        match self {
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
}

