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
        Frame::setup();
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
            let note = self.map.notes.[i];
            
            let delta_ms = note.ms - self.timer.ms;
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
