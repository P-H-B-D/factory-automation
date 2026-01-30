use wasm_bindgen::prelude::*;

// Drill data - tracks coal and processing
#[wasm_bindgen]
#[derive(Clone)]
pub struct DrillData {
    coal_count: u32,
    processing_ticks_remaining: u32, // Ticks remaining for current processing (0-60)
}

#[wasm_bindgen]
impl DrillData {
    #[wasm_bindgen(constructor)]
    pub fn new() -> DrillData {
        DrillData {
            coal_count: 0,
            processing_ticks_remaining: 0,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn coal_count(&self) -> u32 {
        self.coal_count
    }

    pub fn add_coal(&mut self) {
        self.coal_count += 1;
    }

    pub fn remove_coal(&mut self, count: u32) -> bool {
        if self.coal_count >= count {
            self.coal_count -= count;
            true
        } else {
            false
        }
    }

    // Start processing if we have coal and not already processing
    pub fn start_processing_if_able(&mut self) {
        if self.processing_ticks_remaining == 0 && self.coal_count > 0 {
            self.processing_ticks_remaining = 60; // 60 ticks to process
        }
    }

    // Process one tick - returns true if processing completed
    pub fn process_tick(&mut self) -> bool {
        if self.processing_ticks_remaining > 0 {
            self.processing_ticks_remaining -= 1;
            if self.processing_ticks_remaining == 0 {
                // Processing complete - burn one coal
                if self.coal_count > 0 {
                    self.coal_count -= 1;
                    return true;
                }
            }
        }
        false
    }

    #[wasm_bindgen(getter)]
    pub fn processing_ticks_remaining(&self) -> u32 {
        self.processing_ticks_remaining
    }
}

