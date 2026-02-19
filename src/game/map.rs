use crate::eadk::display::Color565;
calc_use!(alloc::collections::vec_deque::VecDeque);
sim_use!(std::collections::VecDeque);

#[derive(Clone, Copy)]
pub enum NoteClass {
    Tap,
    Hold { ms_end: u32 }
}

#[derive(Clone, Copy)]
pub struct Note {
    pub ms: u32,
    pub x: f32,
    pub class: NoteClass
}

#[derive(Clone, Copy)]
pub enum EventClass {
    // Backlight { fraction: f32, duration: u32 },
    Shift { displacement: i16 },
    BGColor { color: Color565 }  // this pleases me
}

#[derive(Clone, Copy)]
pub struct Event {
    pub ms: u32,
    pub duration: u32,
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

    (hold, $ms:expr, $x:expr, $end:expr) => {
        Note {
            ms: $ms,
            x: $x,
            class: NoteClass::Hold { ms_end: $end },
        }
    };
}

#[macro_export]
macro_rules! event {
    (shift, $ms:expr, $dur:expr, $dis:expr) => {
        Event {
            ms: $ms,
            duration: $dur,
            class: EventClass::Shift { displacement: $dis },
        }
    };

    (bg, $ms:expr, $dur:expr, $r:expr, $g:expr, $b:expr) => {
        Event {
            ms: $ms,
            duration: $dur,
            class: EventClass::BGColor { color: Color565::from_rgb($r, $g, $b) }
        }
    };
}

