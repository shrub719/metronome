use crate::{
    constants::*, 
    game::{
        frame::Frame, 
        input::Input, 
        map::*, 
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
            Great => " great       ",
            Good => " good        ",
            Miss => " miss...     "
        }
    }
    
    pub fn to_score(self) -> u32 {
        use Judgement::*;
        use judgement::*;

        match self {
            Perfect => PERFECT_SCORE,
            Great => GREAT_SCORE,
            Good => GOOD_SCORE,
            Miss => MISS_SCORE
        }
    }
    
    pub fn from_offset(offset: i32) -> Judgement {
        use Judgement::*;   // the enum variants
        use judgement::*;   // the timing constants

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
    hold: Option<Note>,
    score: u32,
    map: Map
}
impl Game {
    pub fn new() -> Self {
        Self {
            timer: Timer::new(),
            input: Input::new(),
            frame: Frame::new(),
            hold: None,
            score: 0,
            map: test()
        }
    }

    fn draw_notes(&mut self) {
        let mut i = 0;
        while i < self.map.notes.len() {
            let note = self.map.notes[i];
            i += 1;

            let ms_until: i32 = note.ms as i32 - self.timer.ms as i32;    
            if ms_until > judgement::DRAW_AHEAD_MS { break } // looking too far

            self.frame.draw_note(note.x, ms_until);

            match note.class {
                NoteClass::Hold { duration } => {
                    let end = ms_until + duration as i32;
                    self.frame.draw_hold(note.x, ms_until, end);
                },
                _ => ()
            };
        }

        match self.hold {
            Some(Note { ms, x, class: NoteClass::Hold { duration } }) => {
                let end = (ms + duration) as i32 - self.timer.ms as i32;   // uhhh need so much i32?
                self.frame.draw_hold(x, 0, end)
            }
            _ => ()
        };
    }

    fn register_judgement(&mut self, jdg: Judgement) {
        // temp judgement display
        use crate::eadk;
        use crate::constants::palette::*;
        calc_use!(alloc::format);

        self.score += jdg.to_score();

        eadk::display::draw_string(
            jdg.to_str(),
            eadk::display::ScreenPoint::new(0, 0),
            false,
            WHITE, ORANGE
        );
        eadk::display::draw_string(
            &format!("{} ", self.score),
            eadk::display::ScreenPoint::new(
                display::MARGIN + display::GAME_WIDTH + display::BORDER_SIZE, 
                0
            ),
            false,
            WHITE, ORANGE
        );
    }

    fn judge(&mut self) {
        // cull late notes
        loop {
            if self.map.notes.is_empty() { break; } // FIX

            let note = self.map.notes[0];
            let ms_late = self.timer.ms as i32 - note.ms as i32;
            if ms_late > judgement::GOOD { 
                self.map.notes.pop_front();
                self.register_judgement(Judgement::Miss);
            } else {
                break;
            }
        }

        // check hold notes
        match self.hold {
            Some(Note { ms, x: _, class: NoteClass::Hold { duration } }) => {  // i cannot BELIEVE this works
                if self.timer.ms > ms + duration {
                    self.hold = None;
                    self.register_judgement(Judgement::Perfect);
                } else if !self.input.holding {
                    self.hold = None;
                    self.register_judgement(Judgement::Miss);
                }
            },
            _ => ()
        };

        // hit nearest notes
        for _ in 0..self.input.n_hits {
            if self.map.notes.is_empty() { break; } // FIX

            let note = self.map.notes[0];
            let offset = self.timer.ms as i32 - note.ms as i32;
            
            if offset.abs() <= judgement::MISS {
                self.map.notes.pop_front();

                let jdg = Judgement::from_offset(offset);
                
                if !matches!(jdg, Judgement::Miss) {
                    if matches!(note.class, NoteClass::Hold { duration: _ }) {
                        self.hold = Some(note);
                    }
                }

                self.register_judgement(jdg);
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
