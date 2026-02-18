use crate::eadk::time::get_current_time_millis;

pub struct Timer {
    start_ms: u64,
    pub ms: u32
}
impl Timer {
    pub fn new() -> Self {
        Self {
            start_ms: get_current_time_millis(),
            ms: 0
        }
    }

    pub fn get_ms(&self) -> u32 {
        (get_current_time_millis() - self.start_ms) as u32
    }

    pub fn update(&mut self) {
        self.ms = self.get_ms();
    }
}
