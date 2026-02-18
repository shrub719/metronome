use crate::{
    game::{
        timer::Timer,
        input::Input,
        frame::Frame,
        map::{
            *,
            NoteClass::*
        }
    },
    constants::*,
    maps::test::create_test_map
};

pub struct Game {
    timer: Timer,
    input: Input,
    frame: Frame,
    map: Map
}
impl Game {
    pub fn new() -> Self {
        Self {
            timer: Timer::new(),
            input: Input::new(),
            frame: Frame::new(),
            map: create_test_map()
        }
    }

    fn draw_notes(&mut self) {
        self.frame.reset();

        let mut i = 0;
        while i < self.map.notes.len() {
            let note = self.map.notes[i];
            i += 1;

            let ms_until: i32 = note.ms as i32 - self.timer.ms as i32;    
            if ms_until > judgement::DRAW_AHEAD_MS { break } // gone too far

            self.frame.draw_note(note.x, ms_until);
        }

        self.frame.push();
    }

    fn judge(&mut self) {
        for _ in 0..self.input.n_hits {
            
        }
    }

    pub fn update(&mut self) {
        self.timer.update();
        self.input.update();

        self.judge(); 

        self.draw_notes();
    }
}
