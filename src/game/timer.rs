use crate::eadk::time::get_current_time_millis;

pub struct Timer {
    start_ms: u64,
}
impl Timer {
    pub fn new(bpm: f32) -> Self {
        Self {
            bpms: bpm * 60_000.0,
            start_ms: get_current_time_millis()
        }
    }
}
