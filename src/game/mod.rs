use crate::{
    constants::*, 
    game::{
        frame::Frame, 
        input::Input, 
        timer::Timer
    },
    map::*
};

calc_use!(alloc::vec::Vec);

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
    hold: Vec<Note>,    // TODO: JUST hold notes?
    results: Results,
    map: MapContent,
    finished: bool
}
impl Game {
    pub fn new(pack_index: usize, map_index: usize, accent: crate::eadk::display::Color565) -> Self {
        Self {
            timer: Timer::new(),
            input: Input::new(),
            frame: Frame::new(accent),
            hold: Vec::with_capacity(4),
            results: Results::default(),
            map: load_map_content(pack_index, map_index),
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

        for note in &self.hold {
            match note {
                Note { ms: _, x, class: NoteClass::Hold { ms_end } } => {
                    let ms_until_end = *ms_end as i32 - self.timer.ms as i32;   // uhhh need so much i32?
                    self.frame.draw_hold(*x, 0, ms_until_end)
                },
                _ => panic!()
            };
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
        self.frame.draw_judgement(jdg, self.results.score);
    }

    fn check_events(&mut self) {
        loop {
            if self.map.events.is_empty() {
                break;
            }

            let event = self.map.events[0];
            if self.timer.ms > event.ms {
                self.map.events.pop_front();

                match event.class {
                    EventClass::BGColor { color } => {
                        self.frame.bg = color;
                    }
                };
            } else {
                break;
            }
        }
    }

    fn judge(&mut self) {
        // cull late notes
        loop {
            if self.map.notes.is_empty() { 
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
        let mut j = 1;
        for note in self.hold.clone() {
            match note {
                Note { ms: _, x: _, class: NoteClass::Hold { ms_end } } => {
                    if self.timer.ms > ms_end {
                        self.hold.remove(j-1);  // hmmm
                        j -= 1;
                        self.register_judgement(Judgement::Perfect);
                    } else if !self.input.holding {
                        self.hold.remove(j-1);
                        j -= 1;
                        if ms_end - self.timer.ms < judgement::GOOD as u32 {
                            self.register_judgement(Judgement::Perfect);
                        } else { 
                            self.register_judgement(Judgement::Miss);
                        }
                    }
                },
                _ => panic!()
            };
            j += 1;
        };

        // hit nearest notes
        for _ in 0..self.input.n_hits {
            if self.map.notes.is_empty() { 
                break; 
            } // FIX

            let note = self.map.notes[0];
            let offset = self.timer.ms as i32 - note.ms as i32;
            
            if offset.abs() <= judgement::MISS {
                self.map.notes.pop_front();

                let jdg = Judgement::from_offset(offset);
                
                if !matches!(jdg, Judgement::Miss) {
                    if matches!(note.class, NoteClass::Hold { ms_end: _ }) {
                        self.hold.push(note);
                    }
                }

                self.register_judgement(jdg);
            }
        }

        self.finished = self.map.notes.is_empty() && self.hold.is_empty();
    }

    fn update(&mut self) {
        self.timer.update();
        self.input.update();

        #[cfg(debug_assertions)]
        Frame::draw_ms(self.timer.ms);

        self.judge(); 
        self.check_events();

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
