use crate::{
    constants::{
        spacing::*,
        display::*,
        palette::*
    }, 
    eadk::display::*
};
calc_use!(alloc::boxed::Box);

pub struct Frame {
    buffer: Box<[Color565; BUFFER_SIZE]>
}
impl Frame {
    pub fn new() -> Self {
        Self::push_backdrop();
        Self {
            buffer: Box::new([BLACK; BUFFER_SIZE])
        }
    }
    
    fn push_backdrop() {
        push_rect_uniform(SCREEN_RECT, COLOR_BLACK);
        push_rect_uniform(BACKDROP_RECT, ORANGE);
    }
    
    fn clear(&mut self) {
        for x in &mut *self.buffer {
            *x = BLACK;
        }
    }

    fn draw_judgement_line(&mut self) {
        for x in &mut (*self.buffer)[BUFFER_WIDTH*JUDGEMENT_LINE..BUFFER_WIDTH*(JUDGEMENT_LINE+1)] {
            *x = WHITE;
        }
    }

    pub fn reset(&mut self) {
        self.clear();
        self.draw_judgement_line();
    }

    pub fn push(&self) {
        push_rect(GAME_RECT, &*self.buffer);
    }

    fn place_pixel(&mut self, x: usize, y: usize) {
        if x > BUFFER_WIDTH { return };
        if y > BUFFER_HEIGHT { return };

        let i = BUFFER_WIDTH*y + x;
        if i > BUFFER_SIZE { return };
        self.buffer[i] = ORANGE;
    }

    fn draw_circle(&mut self, x: usize, y: usize) {
        for i in -NOTE_RADIUS_I..NOTE_RADIUS_I {
            let u = (x as isize + i) as usize;
            for j in -NOTE_RADIUS_I..NOTE_RADIUS_I {
                let v = (y as isize + j) as usize;
                if i * i + j * j < NOTE_RADIUS_I * NOTE_RADIUS_I { self.place_pixel(u, v) };
            }
        }
    }

    pub fn draw_note(&mut self, normalised_x: f32, ms_until: i32) {
        let x = X0 + (X_RANGE_F * normalised_x) as usize;

        let dy = PX_PER_MS * ms_until as f32;
        let y = (Y0 as isize - dy as isize) as usize;

        self.draw_circle(x, y);
    }
}

