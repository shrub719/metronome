use crate::{
    constants::*, game::{
        frame::Frame, input::Input, map::{
            NoteClass::*, *
        }, timer::Timer
    }, 
    maps::test::*
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
            map: polyrhythms()
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
        loop {
            if self.map.notes.is_empty() { break; } // FIX

            let note = self.map.notes[0];
            let ms_late = self.timer.ms as i32 - note.ms as i32;
            if ms_late > judgement::GOOD { 
                self.map.notes.pop_front();
            } else {
                break;
            }
        }

        for _ in 0..self.input.n_hits {
            if self.map.notes.is_empty() { break; } // FIX

            let note = self.map.notes[0];
            let offset = note.ms as i32 - self.timer.ms as i32;
            
            if offset.abs() <= judgement::MISS { self.map.notes.pop_front(); }

            use judgement::*;
            let text = match offset {
                0..PERFECT => "perfect!",
                PERFECT..GREAT => "great",
                GREAT..GOOD => "good",
                _ => "miss :("
            };

            crate::eadk::utils::debug(offset);
        }
    }

    pub fn update(&mut self) {
        self.timer.update();
        self.input.update();

        self.judge(); 

        self.draw_notes();
    }
}
