use crate::{
    constants::*, 
    game::{
        frame::Frame, 
        input::Input, 
        map::{
            NoteClass::*, *
        }, 
        timer::Timer
    }, 
    maps::test::*
};

#[derive(Clone, Copy)]
enum Judgement {
    Perfect,
    Great,
    Good,
    Miss
}
impl Judgement {
    pub fn to_str(self) -> &'static str {
        use Judgement::*;

        match self {
            Perfect => " perfect!    ",
            Great => " great    ",
            Good => " good    ",
            Miss => " miss...    "
        }
    }
    
    pub fn from_offset(offset: i32) -> Judgement {
        use Judgement::*;
        use judgement::*;

        match offset.abs() {
            0..PERFECT => Perfect,
            PERFECT..GREAT => Great,
            GREAT..GOOD => Good,
            _ => Miss
        }
    }
}

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
            map: test()
        }
    }

    fn draw_notes(&mut self) {
        let mut i = 0;
        while i < self.map.notes.len() {
            let note = self.map.notes[i];
            i += 1;

            let ms_until: i32 = note.ms as i32 - self.timer.ms as i32;    
            if ms_until > judgement::DRAW_AHEAD_MS { break } // gone too far

            self.frame.draw_note(note.x, ms_until);
        }
    }

    fn display_judgement(jdg: Judgement) {
        use crate::eadk::display;
        use crate::constants::palette::*;

        display::draw_string(
            jdg.to_str(),
            display::ScreenPoint::new(0, 0),
            false,
            WHITE, ORANGE
        );
    }

    fn judge(&mut self) {
        loop {
            if self.map.notes.is_empty() { break; } // FIX

            let note = self.map.notes[0];
            let ms_late = self.timer.ms as i32 - note.ms as i32;
            if ms_late > judgement::GOOD { 
                self.map.notes.pop_front();
                Game::display_judgement(Judgement::Miss);
            } else {
                break;
            }
        }

        for _ in 0..self.input.n_hits {
            if self.map.notes.is_empty() { break; } // FIX

            let note = self.map.notes[0];
            let offset = self.timer.ms as i32 - note.ms as i32;
            
            if offset.abs() <= judgement::MISS {
                self.map.notes.pop_front();

                let jdg = Judgement::from_offset(offset);
                Game::display_judgement(jdg);
            }
        }
    }

    pub fn update(&mut self) {
        self.timer.update();
        self.input.update();

        self.judge(); 

        self.frame.reset();
        self.draw_notes();
        self.frame.push();
    }
}
