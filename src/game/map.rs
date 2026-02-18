use crate::eadk::display::Color565;
calc_use!(alloc::collections::vec_deque::VecDeque);
sim_use!(std::collections::VecDeque);

#[derive(Clone, Copy)]
pub enum NoteClass {
    Tap,
    Hold { duration: u32 }
}

#[derive(Clone, Copy)]
pub struct Note {
    pub ms: u32,
    pub x: f32,
    pub class: NoteClass
}

#[derive(Clone, Copy)]
pub enum EventClass {
    Backlight { fraction: f32, duration: u32 },
    Shake,
    BGColor { color: Color565, duration: u32 }
}

#[derive(Clone, Copy)]
pub struct Event {
    pub ms: u32,
    pub class: EventClass
}

pub struct Map {
    pub notes: VecDeque<Note>,
    pub events: VecDeque<Event>
}

#[macro_export]
macro_rules! note {
    (tap, $ms:expr, $x:expr) => {
        Note {
            ms: $ms,
            x: $x,
            class: NoteClass::Tap,
        }
    };

    (hold, $ms:expr, $x:expr, $dur:expr) => {
        Note {
            ms: $ms,
            x: $x,
            class: NoteClass::Hold { duration: $dur },
        }
    };
}

