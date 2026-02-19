use crate::{
    eadk::keyboard::*,
    constants::judgement::*
};

pub struct Input {
    state: KeyboardState,
    prev_state: KeyboardState,
    pub n_hits: u8,
    pub holding: bool
}
impl Input {
    pub fn new() -> Self {
        Self {
            state: KeyboardState::default(),
            prev_state: KeyboardState::default(),
            n_hits: 0,
            holding: false
        }
    }

    pub fn update(&mut self) {
        self.prev_state = self.state;
        self.state = KeyboardState::scan();
        
        let just_pressed = self.state.get_just_pressed(self.prev_state);
        self.n_hits = 0;

        for k in enum_iterator::all::<Key>() {
            if just_pressed.key_down(k) {
                self.n_hits += 1;
            }
            if self.n_hits == MAX_HITS {
                break;
            }
        }

        self.holding = self.state.any_key_down();
    }
} 
