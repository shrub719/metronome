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

    fn draw_circle(&mut self, x: usize, y: usize, r: isize) {
        for i in -r..r {
            let u = (x as isize + i) as usize;
            for j in -r..r {
                let v = (y as isize + j) as usize;
                if i * i + j * j < r * r { self.place_pixel(u, v) };
            }
        }
    }

    fn draw_line(&mut self, x: usize, y0: usize, y1: usize) {
        for y in y1..y0 {
            for i in -TAIL_RADIUS_I..TAIL_RADIUS_I { 
                let u = (x as isize + i) as usize;
                self.place_pixel(u, y);
            }
        }

        // HACK
        self.draw_circle(x, y1, TAIL_RADIUS_I);
    }

    fn x_from_normalised(normalised_x: f32) -> usize {
        X0 + (X_RANGE_F * normalised_x) as usize
    }

    fn y_from_ms(ms: i32) -> usize {
        let dy = PX_PER_MS * ms as f32;
        (Y0 as isize - dy as isize) as usize
    }

    pub fn draw_note(&mut self, normalised_x: f32, ms_until: i32) {
        let x = Frame::x_from_normalised(normalised_x);
        let y = Frame::y_from_ms(ms_until);

        self.draw_circle(x, y, NOTE_RADIUS_I);
    }

    pub fn draw_hold(&mut self, normalised_x: f32, start: i32, end: i32) {
        let x = Frame::x_from_normalised(normalised_x);
        let y0 = Frame::y_from_ms(start);
        let mut y1 = Frame::y_from_ms(end);
        if y1 > BUFFER_HEIGHT { y1 = 0; }   // trick; technically it overflows if its off-screen

        self.draw_line(x, y0, y1);
    }
}

