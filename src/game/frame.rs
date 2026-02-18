use crate::{
    constants::*, 
    eadk::display::*
};
calc_use!(alloc::boxed::Box);

pub struct Frame {
    buffer: Box<[Color565; BUFFER_SIZE]>
}
impl Frame {
    pub fn new() -> Self {
        Self {
            buffer: Box::new([BLACK; BUFFER_SIZE])
        }
    }
    
    fn push_backdrop() {
        push_rect_uniform(BACKDROP_RECT, ORANGE);
    }
    
    pub fn reset(&mut self) {
        for i in 0..BUFFER_SIZE {
            self.buffer[i] = BLACK;
        }
    }

    pub fn push(&self) {
        push_rect(GAME_RECT, &*self.buffer);
    }

    fn blue_pixel(&mut self, x: usize, y: usize) {
        let i = BUFFER_WIDTH*y + x;
        
        if i < BUFFER_SIZE {
            self.buffer[i] = ORANGE;
        }
    }

    pub fn setup() {
        Self::push_backdrop();
    }

    pub fn draw_note(&mut self, x: f32, y: f32) {
        let u = LANE_START + (LANE_RANGE_F * x) as usize;
        let v = (LANE_HEIGHT_F * (1.0 - y)) as usize;
        self.blue_pixel(u, v);
    }
}

