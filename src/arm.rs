use wasm_bindgen::prelude::*;

// Arm data - tracks tick counter for duty cycle
#[wasm_bindgen]
#[derive(Clone)]
pub struct ArmData {
    tick_counter: u32, // Current tick in the duty cycle (0-119)
}

#[wasm_bindgen]
impl ArmData {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ArmData {
        ArmData {
            tick_counter: 0,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn tick_counter(&self) -> u32 {
        self.tick_counter
    }

    // Increment tick counter and return true if it's time to act (every 120 ticks)
    pub fn process_tick(&mut self) -> bool {
        self.tick_counter += 1;
        if self.tick_counter >= 120 {
            self.tick_counter = 0;
            true
        } else {
            false
        }
    }
}

