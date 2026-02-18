use crate::eadk::display::Color565;
calc_use!(alloc::collections::vec_deque::VecDeque);
sim_use!(std::collections::VecDeque);

#[derive(Clone, Copy)]
pub enum NoteClass {
    Tap
}

#[derive(Clone, Copy)]
pub struct Note {
    pub ms: u32,
    pub x: f32
}

#[derive(Clone, Copy)]
pub enum EventClass {
    Backlight { fraction: f32, duration: f32 },
    Shake,
    BGColor { color: Color565, duration: f32 }
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
    ($ms:expr, $x:expr) => {
        Note { ms: $ms, x: $x }
    };
}

#[macro_export]
macro_rules! notes {
    ( $( ($ms:expr, $x:expr) ),* $(,)? ) => {
        vec![
            $(
                Note { ms: $ms, x: $x },
            )*
        ]
    };
}
