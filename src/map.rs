use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use crate::types::{Item, PlaceableType, ResourceType, Direction};
use crate::furnace::FurnaceData;
use crate::arm::ArmData;
use crate::chest::ChestData;
use crate::drill::DrillData;

// Placeable object struct
#[wasm_bindgen]
#[derive(Clone)]
pub struct PlaceableObject {
    x: u32,
    y: u32,
    placeable_type: PlaceableType,
    direction: Direction, // Direction for belts (and potentially other rotatable objects)
}

#[wasm_bindgen]
impl PlaceableObject {
    #[wasm_bindgen(getter)]
    pub fn x(&self) -> u32 {
        self.x
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> u32 {
        self.y
    }

    #[wasm_bindgen(getter)]
    pub fn placeable_type(&self) -> PlaceableType {
        self.placeable_type
    }

    pub fn placeable_type_value(&self) -> u32 {
        self.placeable_type.value()
    }

    #[wasm_bindgen(getter)]
    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn direction_value(&self) -> u32 {
        self.direction.value()
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
}

// Resource structs (for resources that don't disappear)
#[wasm_bindgen]
#[derive(Clone)]
pub struct Resource {
    x: u32,
    y: u32,
    resource_type: ResourceType,
}

#[wasm_bindgen]
impl Resource {
    #[wasm_bindgen(constructor)]
    pub fn new(x: u32, y: u32, resource_type: ResourceType) -> Resource {
        Resource { x, y, resource_type }
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> u32 {
        self.x
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> u32 {
        self.y
    }

    #[wasm_bindgen(getter)]
    pub fn resource_type(&self) -> ResourceType {
        self.resource_type
    }

    pub fn resource_type_value(&self) -> u32 {
        self.resource_type.value()
    }
}

// Water patch struct
#[wasm_bindgen]
#[derive(Clone)]
pub struct WaterPatch {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl WaterPatch {
    #[wasm_bindgen(constructor)]
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> WaterPatch {
        WaterPatch { x, y, width, height }
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> u32 {
        self.x
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> u32 {
        self.y
    }

    #[wasm_bindgen(getter)]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> u32 {
        self.height
    }
}

// Map struct
#[wasm_bindgen]
pub struct Map {
    width: u32,
    height: u32,
    water_patches: Vec<WaterPatch>,
    resources: Vec<Resource>,
    placeable_objects: Vec<PlaceableObject>,
    furnace_data: HashMap<(u32, u32), FurnaceData>,
    arm_data: HashMap<(u32, u32), ArmData>,
    chest_data: HashMap<(u32, u32), ChestData>,
    drill_data: HashMap<(u32, u32), DrillData>,
    dropped_items: Vec<DroppedItem>,
    belt_items: HashMap<(u32, u32), DroppedItem>, // Items currently on belts (only 1 per belt)
}

#[wasm_bindgen]
impl Map {
    #[wasm_bindgen(getter)]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn water_patches(&self) -> Vec<WaterPatch> {
        self.water_patches.clone()
    }

    pub fn resources(&self) -> Vec<Resource> {
        self.resources.clone()
    }

    pub fn placeable_objects(&self) -> Vec<PlaceableObject> {
        self.placeable_objects.clone()
    }

    pub fn dropped_items(&self) -> Vec<DroppedItem> {
        self.dropped_items.clone()
    }

    pub fn get_resource_at(&self, x: u32, y: u32) -> Option<ResourceType> {
        for resource in &self.resources {
            if resource.x == x && resource.y == y {
                return Some(resource.resource_type);
            }
        }
        None
    }

    pub fn get_placeable_at(&self, x: u32, y: u32) -> Option<PlaceableType> {
        for obj in &self.placeable_objects {
            if obj.x == x && obj.y == y {
                return Some(obj.placeable_type);
            }
        }
        None
    }

    pub(crate) fn get_placeable_object_at(&self, x: u32, y: u32) -> Option<&PlaceableObject> {
        self.placeable_objects.iter().find(|obj| obj.x == x && obj.y == y)
    }

    pub(crate) fn get_placeable_object_at_mut(&mut self, x: u32, y: u32) -> Option<&mut PlaceableObject> {
        self.placeable_objects.iter_mut().find(|obj| obj.x == x && obj.y == y)
    }

    pub fn get_furnace_data(&self, x: u32, y: u32) -> Option<FurnaceData> {
        self.furnace_data.get(&(x, y)).cloned()
    }

    pub fn get_dropped_item_index_at(&self, x: u32, y: u32) -> Option<usize> {
        self.dropped_items.iter().position(|item| item.x() == x && item.y() == y)
    }

    pub fn set_furnace_data(&mut self, x: u32, y: u32, data: FurnaceData) {
        self.furnace_data.insert((x, y), data);
    }

    pub fn get_arm_data(&self, x: u32, y: u32) -> Option<ArmData> {
        self.arm_data.get(&(x, y)).cloned()
    }

    pub fn set_arm_data(&mut self, x: u32, y: u32, data: ArmData) {
        self.arm_data.insert((x, y), data);
    }

    pub fn get_chest_data(&self, x: u32, y: u32) -> Option<ChestData> {
        self.chest_data.get(&(x, y)).cloned()
    }

    pub fn set_chest_data(&mut self, x: u32, y: u32, data: ChestData) {
        self.chest_data.insert((x, y), data);
    }

    pub fn get_drill_data(&self, x: u32, y: u32) -> Option<DrillData> {
        self.drill_data.get(&(x, y)).cloned()
    }

    pub fn set_drill_data(&mut self, x: u32, y: u32, data: DrillData) {
        self.drill_data.insert((x, y), data);
    }

    pub fn add_placeable(&mut self, x: u32, y: u32, placeable_type: PlaceableType) {
        // Check if position is valid (not on water, not occupied)
        if self.would_collide_with_water(x, y) {
            return;
        }
        if self.get_placeable_at(x, y).is_some() {
            return;
        }
        // Drills can be placed on resources, other items cannot
        if !matches!(placeable_type, PlaceableType::Drill) {
            if self.get_resource_at(x, y).is_some() {
                return;
            }
        }
        self.placeable_objects.push(PlaceableObject {
            x,
            y,
            placeable_type,
            direction: Direction::East, // Default direction for belts
        });
        // Initialize furnace data if it's a furnace
        if matches!(placeable_type, PlaceableType::Furnace) {
            self.furnace_data.insert((x, y), FurnaceData::new());
        }
        // Initialize arm data if it's an arm
        if matches!(placeable_type, PlaceableType::Arm) {
            self.arm_data.insert((x, y), ArmData::new());
        }
        // Initialize chest data if it's a chest
        if matches!(placeable_type, PlaceableType::Chest) {
            self.chest_data.insert((x, y), ChestData::new());
        }
        // Initialize drill data if it's a drill
        if matches!(placeable_type, PlaceableType::Drill) {
            self.drill_data.insert((x, y), DrillData::new());
        }
    }

    pub fn would_collide_with_water(&self, x: u32, y: u32) -> bool {
        for patch in &self.water_patches {
            if x < patch.x + patch.width
                && x + 1 > patch.x
                && y < patch.y + patch.height
                && y + 1 > patch.y
            {
                return true;
            }
        }
        false
    }

    pub fn would_collide_with_placeable(&self, x: u32, y: u32) -> bool {
        // Furnaces and chests block movement, belts are walkable
        if let Some(placeable_type) = self.get_placeable_at(x, y) {
            matches!(placeable_type, PlaceableType::Furnace | PlaceableType::Chest)
        } else {
            false
        }
    }

    // Internal methods for map generation
    pub fn new(width: u32, height: u32) -> Map {
        Map {
            width,
            height,
            water_patches: Vec::new(),
            resources: Vec::new(),
            placeable_objects: Vec::new(),
            furnace_data: HashMap::new(),
            arm_data: HashMap::new(),
            chest_data: HashMap::new(),
            drill_data: HashMap::new(),
            dropped_items: Vec::new(),
            belt_items: HashMap::new(),
        }
    }

    pub fn add_water_patch(&mut self, patch: WaterPatch) {
        self.water_patches.push(patch);
    }

    pub fn add_resource(&mut self, resource: Resource) {
        self.resources.push(resource);
    }

    pub fn add_dropped_item(&mut self, item: DroppedItem) {
        self.dropped_items.push(item);
    }

    pub fn remove_dropped_item(&mut self, index: usize) {
        if index < self.dropped_items.len() {
            self.dropped_items.remove(index);
        }
    }

    // Internal method (not exposed to wasm)
    pub(crate) fn get_dropped_item_mut(&mut self, index: usize) -> Option<&mut DroppedItem> {
        self.dropped_items.get_mut(index)
    }

    pub(crate) fn has_belt_item(&self, x: u32, y: u32) -> bool {
        self.belt_items.contains_key(&(x, y))
    }

    pub(crate) fn set_belt_item(&mut self, x: u32, y: u32, item: DroppedItem) {
        self.belt_items.insert((x, y), item);
    }

    pub fn get_belt_item(&self, x: u32, y: u32) -> Option<DroppedItem> {
        self.belt_items.get(&(x, y)).cloned()
    }

    pub(crate) fn remove_belt_item(&mut self, x: u32, y: u32) {
        self.belt_items.remove(&(x, y));
    }

    pub(crate) fn remove_placeable(&mut self, x: u32, y: u32) {
        // Remove from placeable_objects
        self.placeable_objects.retain(|obj| !(obj.x == x && obj.y == y));
        // Remove associated data
        self.furnace_data.remove(&(x, y));
        self.arm_data.remove(&(x, y));
        self.chest_data.remove(&(x, y));
        self.drill_data.remove(&(x, y));
        // Remove belt items if it was a belt
        self.belt_items.remove(&(x, y));
    }
}

// Dropped item struct - items that can be picked up from the ground
#[wasm_bindgen]
#[derive(Clone)]
pub struct DroppedItem {
    x: u32,
    y: u32,
    item: Item,
    quantity: u32,
}

#[wasm_bindgen]
impl DroppedItem {
    #[wasm_bindgen(constructor)]
    pub fn new(x: u32, y: u32, item: Item, quantity: u32) -> DroppedItem {
        DroppedItem { x, y, item, quantity }
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> u32 {
        self.x
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> u32 {
        self.y
    }

    #[wasm_bindgen(getter)]
    pub fn item(&self) -> Item {
        self.item.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn quantity(&self) -> u32 {
        self.quantity
    }

    pub fn set_position(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }

    pub fn add_quantity(&mut self, amount: u32) {
        self.quantity += amount;
    }

    pub fn remove_quantity(&mut self, amount: u32) -> bool {
        if self.quantity >= amount {
            self.quantity -= amount;
            true
        } else {
            false
        }
    }
}

// Keep IronOre for backward compatibility (deprecated, use Resource instead)
#[wasm_bindgen]
#[derive(Clone)]
pub struct IronOre {
    x: u32,
    y: u32,
}

#[wasm_bindgen]
impl IronOre {
    #[wasm_bindgen(constructor)]
    pub fn new(x: u32, y: u32) -> IronOre {
        IronOre { x, y }
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> u32 {
        self.x
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> u32 {
        self.y
    }
}

