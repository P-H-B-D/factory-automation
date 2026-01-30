use std::collections::HashSet;
use crate::map::{Map, WaterPatch, Resource};
use crate::types::ResourceType;

// Simple PRNG for procedural generation
pub struct SimpleRng {
    seed: u64,
}

impl SimpleRng {
    pub fn new() -> Self {
        // Use current time as seed (approximated)
        let seed = js_sys::Date::now() as u64;
        SimpleRng { seed }
    }

    pub fn next(&mut self) -> u64 {
        // Linear congruential generator
        self.seed = self.seed.wrapping_mul(1103515245).wrapping_add(12345);
        self.seed
    }
}

// Generate map function
pub fn generate_map() -> Map {
    // Map size in tiles (160x120 tiles - 4x bigger than 40x30)
    let map_width: u32 = 160;
    let map_height: u32 = 120;
    
    let mut rng = SimpleRng::new();
    let mut used_positions = HashSet::new();
    
    let mut map = Map::new(map_width, map_height);
    
    // Generate water patches procedurally
    let num_patches = 10 + (rng.next() % 11) as usize; // 10-20 water patches
    
    for _ in 0..num_patches {
        let mut attempts = 0;
        loop {
            let x = (rng.next() % map_width as u64) as u32;
            let y = (rng.next() % map_height as u64) as u32;
            let width = 1 + (rng.next() % 5) as u32; // 1x1 to 5x5 tiles
            let height = 1 + (rng.next() % 5) as u32;
            
            // Make sure water patch fits within map bounds
            if x + width > map_width || y + height > map_height {
                attempts += 1;
                if attempts > 50 {
                    break;
                }
                continue;
            }
            
            // Check if position overlaps significantly
            let mut overlaps = false;
            for used in &used_positions {
                let (ux, uy) = *used;
                if x < ux + 3 && x + width > ux && y < uy + 3 && y + height > uy {
                    overlaps = true;
                    break;
                }
            }
            
            if !overlaps || attempts > 50 {
                map.add_water_patch(WaterPatch::new(x, y, width, height));
                used_positions.insert((x, y));
                break;
            }
            attempts += 1;
        }
    }
    
    // Generate resources (iron ore, copper, stone, coal) as patches, not on water
    let resource_types = [
        ResourceType::IronOre,
        ResourceType::Copper,
        ResourceType::Stone,
        ResourceType::Coal,
    ];
    
    // Generate 5-10 patches of each resource type
    for resource_type in resource_types.iter() {
        let num_patches = 5 + (rng.next() % 6) as usize; // 5-10 patches of each type
        
        for _ in 0..num_patches {
            let mut attempts = 0;
            loop {
                let x = (rng.next() % map_width as u64) as u32;
                let y = (rng.next() % map_height as u64) as u32;
                let width = 1 + (rng.next() % 4) as u32; // 1x1 to 4x4 tiles
                let height = 1 + (rng.next() % 4) as u32;
                
                // Make sure resource patch fits within map bounds
                if x + width > map_width || y + height > map_height {
                    attempts += 1;
                    if attempts > 50 {
                        break;
                    }
                    continue;
                }
                
                // Check if patch overlaps with water
                let mut overlaps_water = false;
                for patch in map.water_patches() {
                    if x < patch.x() + patch.width()
                        && x + width > patch.x()
                        && y < patch.y() + patch.height()
                        && y + height > patch.y()
                    {
                        overlaps_water = true;
                        break;
                    }
                }
                
                // Check if patch overlaps significantly with existing resources
                let mut overlaps_resource = false;
                for used in &used_positions {
                    let (ux, uy) = *used;
                    if x < ux + 3 && x + width > ux && y < uy + 3 && y + height > uy {
                        overlaps_resource = true;
                        break;
                    }
                }
                
                if !overlaps_water && (!overlaps_resource || attempts > 50) {
                    // Add resources for each tile in the patch
                    for patch_x in x..(x + width) {
                        for patch_y in y..(y + height) {
                            // Double-check not on water
                            let mut on_water = false;
                            for patch in map.water_patches() {
                                if patch_x >= patch.x() && patch_x < patch.x() + patch.width()
                                    && patch_y >= patch.y() && patch_y < patch.y() + patch.height()
                                {
                                    on_water = true;
                                    break;
                                }
                            }
                            if !on_water {
                                map.add_resource(Resource::new(patch_x, patch_y, *resource_type));
                            }
                        }
                    }
                    used_positions.insert((x, y));
                    break;
                }
                
                attempts += 1;
                if attempts > 50 {
                    break;
                }
            }
        }
    }
    
    map
}

