use crate::{
    constants::*, 
    game::{
        frame::Frame, 
        input::Input, 
        timer::Timer
    },
    map::{
        *,
        maps::MAPS
    }
};

mod frame;
mod timer;
mod input;

#[derive(Clone, Copy)]
pub enum Judgement {
    Perfect,
    Great,
    Good,
    Miss
}
impl Judgement {
    pub fn to_str(self) -> &'static str {
        use Judgement::*;

        match self {
            Perfect => "perfect!",
            Great => "great   ",
            Good => "good    ",
            Miss => "miss... "
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

#[derive(Clone, Copy, Default)]
pub struct Results {
    pub score: u32,
    pub perfect: u32,
    pub great: u32,
    pub good: u32,
    pub miss: u32
}

pub struct Game {
    timer: Timer,
    input: Input,
    frame: Frame,
    hold: Option<Note>,
    results: Results,
    map: Map,
    finished: bool
}
impl Game {
    pub fn new(level_index: usize) -> Self {
        Self {
            timer: Timer::new(),
            input: Input::new(),
            frame: Frame::new(),
            hold: None,
            results: Results::default(),
            map: load_map(MAPS[level_index]),
            finished: false
        }
    }

    fn draw_notes(&mut self) {
        let mut i = 0;
        while i < self.map.notes.len() {
            let note = self.map.notes[i];
            i += 1;

            let ms_until: i32 = note.ms as i32 - self.timer.ms as i32;    
            if ms_until > judgement::DRAW_AHEAD_MS { break } // looking too far

            self.frame.draw_tap(note.x, ms_until);

            match note.class {
                NoteClass::Hold { ms_end } => {
                    let ms_until_end = ms_end as i32 - self.timer.ms as i32;
                    self.frame.draw_hold(note.x, ms_until, ms_until_end);
                },
                _ => ()
            };
        }

        match self.hold {
            Some(Note { ms: _, x, class: NoteClass::Hold { ms_end } }) => {
                let ms_until_end =  ms_end as i32 - self.timer.ms as i32;   // uhhh need so much i32?
                self.frame.draw_hold(x, 0, ms_until_end)
            }
            _ => ()
        };
    }

    fn register_judgement(&mut self, jdg: Judgement) {
        match jdg {
            Judgement::Perfect => self.results.perfect += 1,
            Judgement::Great => self.results.great +=1,
            Judgement::Good => self.results.good += 1,
            Judgement::Miss => self.results.miss += 1
        };

        self.results.score += jdg.to_score();
        Frame::draw_judgement(jdg, self.results.score);
    }

    fn judge(&mut self) {
        // cull late notes
        loop {
            if self.map.notes.is_empty() { 
                self.finished = true;
                break; 
            } // FIX

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
            Some(Note { ms: _, x: _, class: NoteClass::Hold { ms_end } }) => {  // i cannot BELIEVE this works
                if self.timer.ms > ms_end {
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
            if self.map.notes.is_empty() { 
                self.finished = true;
                break; 
            } // FIX

            let note = self.map.notes[0];
            let offset = self.timer.ms as i32 - note.ms as i32;
            
            if offset.abs() <= judgement::MISS {
                self.map.notes.pop_front();

                let jdg = Judgement::from_offset(offset);
                
                if !matches!(jdg, Judgement::Miss) {
                    if matches!(note.class, NoteClass::Hold { ms_end: _ }) {
                        self.hold = Some(note);
                    }
                }

                self.register_judgement(jdg);
            }
        }
    }

    fn update(&mut self) {
        self.timer.update();
        self.input.update();

        self.judge(); 

        self.frame.reset();
        self.draw_notes();
        self.frame.push();
    }

    pub fn main_loop(&mut self) -> Option<Results> {
        while !self.input.quit && !self.finished {
            self.update();
        }

        if self.finished { Some(self.results) } else { None }
    }
}
