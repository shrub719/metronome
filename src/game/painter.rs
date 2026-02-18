use crate::{
    constants::{ display::*, palette::* }, 
    eadk::display::*
};
calc_use!(alloc::boxed::Box);

pub struct Painter {
    frame_buffer: Box<[Color565; BUFFER_SIZE]>
}
impl Painter {
    pub fn new() -> Self {
        Self {
            frame_buffer: Box::new([BLACK; BUFFER_SIZE])
        }
    }
    
    pub fn push_backdrop() {
        push_rect_uniform(BACKDROP_RECT, ORANGE);
    }

    pub fn push(&self) {
        push_rect(LANE_RECT, &*self.frame_buffer);
    }
}

