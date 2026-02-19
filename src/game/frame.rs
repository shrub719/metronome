use crate::{
    constants::{
        spacing::*,
        display::*,
        palette::*
    }, 
    eadk::display::*,
    game::game::Judgement
};
calc_use!(alloc::boxed::Box);
calc_use!(alloc::string::ToString);

pub struct Frame {
    buffer: Box<[Color565; BUFFER_SIZE]>
}
impl Frame {
    pub fn new() -> Self {
        Self::setup();
        Self {
            buffer: Box::new([BLACK; BUFFER_SIZE])
        }
    }
    
    fn setup() {
        push_rect_uniform(SCREEN_RECT, COLOR_BLACK);
        push_rect_uniform(BACKDROP_RECT, ORANGE);

        push_rect_uniform(UI_JUDGEMENT_RECT, ORANGE);
        push_rect_uniform(UI_SCORE_RECT, ORANGE);

        draw_string("ready?", UI_JUDGEMENT_POINT, true, WHITE, ORANGE);
        draw_string("0", UI_SCORE_POINT, true, WHITE, ORANGE);
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

    pub fn draw_judgement(jdg: Judgement, score: u32) {
        draw_string(
            &jdg.to_str(),
            UI_JUDGEMENT_POINT,
            true, WHITE, ORANGE
        );
        draw_string(
            &score.to_string(),
            UI_SCORE_POINT,
            true, WHITE, ORANGE
        );
    }

    fn place_pixel(&mut self, x: usize, y: usize) {
        if x > BUFFER_WIDTH { return };
        if y > BUFFER_HEIGHT { return };

        let i = BUFFER_WIDTH*y + x;
        if i > BUFFER_SIZE { return };
        self.buffer[i] = ORANGE;
    }

    fn draw_note(&mut self, x: usize, y: usize, r: isize) {
        for i in -r..r {
            let u = (x as isize + i) as usize;
            for j in -r..r {
                let v = (y as isize + j) as usize;
                if i * i + j * j < r * r { self.place_pixel(u, v) };
            }
        }
    }

    fn draw_tail(&mut self, x: usize, y0: usize, y1: usize) {
        for y in y1..y0 {
            for i in -TAIL_RADIUS_I..TAIL_RADIUS_I { 
                let u = (x as isize + i) as usize;
                self.place_pixel(u, y);
            }
        }

        // HACK
        self.draw_note(x, y1, TAIL_RADIUS_I);
    }

    fn x_from_normalised(normalised_x: f32) -> usize {
        X0 + (X_RANGE_F * normalised_x) as usize
    }

    fn y_from_ms(ms: i32) -> usize {
        let dy = PX_PER_MS * ms as f32;
        (Y0 as isize - dy as isize) as usize
    }

    pub fn draw_tap(&mut self, normalised_x: f32, ms_until: i32) {
        let x = Frame::x_from_normalised(normalised_x);
        let y = Frame::y_from_ms(ms_until);

        self.draw_note(x, y, NOTE_RADIUS_I);
    }

    pub fn draw_hold(&mut self, normalised_x: f32, start: i32, end: i32) {
        let x = Frame::x_from_normalised(normalised_x);
        let y0 = Frame::y_from_ms(start);
        let mut y1 = Frame::y_from_ms(end);
        if y1 > BUFFER_HEIGHT { y1 = 0; }   // trick; technically it overflows if its off-screen

        self.draw_tail(x, y0, y1);
    }
}

