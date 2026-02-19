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
    // pub events: VecDeque<Event>
}

pub fn load_map() -> Map {
    let bytes = include_bytes!("../../target/maps/test.bin");

    let mut notes = VecDeque::with_capacity(bytes.len() / 13);   // TODO: extract note length

    let mut i = 0;
    while i + 13 <= bytes.len() {
        let class_id: char = bytes[i] as char;

        let ms = u32::from_le_bytes([
            bytes[i+1], bytes[i+2], bytes[i+3], bytes[i+4]
        ]);

        let x = f32::from_le_bytes([
            bytes[i+5], bytes[i+6], bytes[i+7], bytes[i+8]
        ]);

        let note = Note {
            ms, x,
            class: match class_id {
                't' => NoteClass::Tap,
                'h' => NoteClass::Hold { 
                    ms_end: u32::from_le_bytes([
                        bytes[i+9], bytes[i+10], bytes[i+11], bytes[i+12]
                    ])
                },
                _ => NoteClass::Tap
            }
        };

        notes.push_back(note);

        i += 13;
    }


    Map { notes }
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

