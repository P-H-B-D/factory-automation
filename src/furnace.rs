use wasm_bindgen::prelude::*;

// Processing type enum (internal)
#[derive(Clone, Copy, PartialEq)]
enum ProcessingType {
    None,
    IronPlate,
    CopperPlate,
}

// Furnace inventory data
#[wasm_bindgen]
#[derive(Clone)]
pub struct FurnaceData {
    coal_count: u32,
    iron_ore_count: u32,
    iron_plate_count: u32,
    copper_count: u32,
    copper_plate_count: u32,
    processing_ticks_remaining: u32, // Ticks remaining for current processing
    processing_type: ProcessingType, // What we're currently processing
}

#[wasm_bindgen]
impl FurnaceData {
    #[wasm_bindgen(constructor)]
    pub fn new() -> FurnaceData {
        FurnaceData {
            coal_count: 0,
            iron_ore_count: 0,
            iron_plate_count: 0,
            copper_count: 0,
            copper_plate_count: 0,
            processing_ticks_remaining: 0,
            processing_type: ProcessingType::None,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn coal_count(&self) -> u32 {
        self.coal_count
    }

    #[wasm_bindgen(getter)]
    pub fn iron_ore_count(&self) -> u32 {
        self.iron_ore_count
    }

    #[wasm_bindgen(getter)]
    pub fn iron_plate_count(&self) -> u32 {
        self.iron_plate_count
    }

    #[wasm_bindgen(getter)]
    pub fn copper_count(&self) -> u32 {
        self.copper_count
    }

    #[wasm_bindgen(getter)]
    pub fn copper_plate_count(&self) -> u32 {
        self.copper_plate_count
    }

    pub fn add_coal(&mut self) {
        self.coal_count += 1;
    }

    pub fn add_iron_ore(&mut self) {
        self.iron_ore_count += 1;
    }

    pub fn add_copper(&mut self) {
        self.copper_count += 1;
    }

    pub fn remove_coal(&mut self, count: u32) -> bool {
        if self.coal_count >= count {
            self.coal_count -= count;
            true
        } else {
            false
        }
    }

    pub fn remove_iron_ore(&mut self, count: u32) -> bool {
        if self.iron_ore_count >= count {
            self.iron_ore_count -= count;
            true
        } else {
            false
        }
    }

    pub fn remove_copper(&mut self, count: u32) -> bool {
        if self.copper_count >= count {
            self.copper_count -= count;
            true
        } else {
            false
        }
    }

    pub fn add_iron_plate(&mut self) {
        self.iron_plate_count += 1;
    }

    pub fn remove_iron_plate(&mut self, count: u32) -> bool {
        if self.iron_plate_count >= count {
            self.iron_plate_count -= count;
            true
        } else {
            false
        }
    }

    pub fn add_copper_plate(&mut self) {
        self.copper_plate_count += 1;
    }

    pub fn remove_copper_plate(&mut self, count: u32) -> bool {
        if self.copper_plate_count >= count {
            self.copper_plate_count -= count;
            true
        } else {
            false
        }
    }

    // Start processing if we have materials and not already processing
    pub fn start_processing_if_able(&mut self) {
        if self.processing_ticks_remaining == 0 {
            // Try iron plate first
            if self.coal_count > 0 && self.iron_ore_count > 0 {
                self.processing_ticks_remaining = 60; // 60 ticks to process
                self.processing_type = ProcessingType::IronPlate;
            }
            // Then try copper plate
            else if self.coal_count > 0 && self.copper_count > 0 {
                self.processing_ticks_remaining = 60; // 60 ticks to process
                self.processing_type = ProcessingType::CopperPlate;
            }
        }
    }
    
    // Process one tick - returns true if processing completed
    pub fn process_tick(&mut self) -> bool {
        if self.processing_ticks_remaining > 0 {
            self.processing_ticks_remaining -= 1;
            if self.processing_ticks_remaining == 0 {
                // Processing complete - convert based on processing type
                match self.processing_type {
                    ProcessingType::IronPlate => {
                        if self.coal_count > 0 && self.iron_ore_count > 0 {
                            self.coal_count -= 1;
                            self.iron_ore_count -= 1;
                            self.iron_plate_count += 1;
                            self.processing_type = ProcessingType::None;
                            return true;
                        }
                    }
                    ProcessingType::CopperPlate => {
                        if self.coal_count > 0 && self.copper_count > 0 {
                            self.coal_count -= 1;
                            self.copper_count -= 1;
                            self.copper_plate_count += 1;
                            self.processing_type = ProcessingType::None;
                            return true;
                        }
                    }
                    ProcessingType::None => {}
                }
            }
        }
        false
    }
    
    #[wasm_bindgen(getter)]
    pub fn processing_ticks_remaining(&self) -> u32 {
        self.processing_ticks_remaining
    }
    
    // Try to combine coal and iron ore into iron plate (deprecated - kept for compatibility)
    pub fn try_combine(&mut self) -> bool {
        // This is now handled by process_tick, but keep for compatibility
        false
    }
}

