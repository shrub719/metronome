use crate::{
    constants::file::*,
    eadk::display::Color565
};
calc_use!(alloc::collections::vec_deque::VecDeque);
sim_use!(std::collections::VecDeque);

pub mod maps;
pub mod storage;

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
    // Shift { displacement: i16 },
    // BGColor { color: Color565 }  // this pleases me
    BGColor { color: Color565 }
}

#[derive(Clone, Copy)]
pub struct Event {
    pub ms: u32,
    pub class: EventClass
}

pub struct MapContent {
    pub notes: VecDeque<Note>,
    pub events: VecDeque<Event>
}

#[derive(Clone, Copy)]
pub struct MapData {
    pub title: &'static str,
    pub artist: &'static str,
    pub id: &'static str
}

pub type Map = &'static [u8];

pub struct MapPack {
    pub maps: &'static [Map],
    pub color: Color565
}

fn load_str(full_bytes: &[u8], at: usize) -> &str {
    let bytes = &full_bytes[BINARY_STR_LENGTH*at..BINARY_STR_LENGTH*(at+1)];

    let nul_pos = bytes.iter().position(|&b| b == 0).expect("non-terminated metadata string");
    str::from_utf8(&bytes[..nul_pos]).expect("invalid utf-8 in metadata string")
}

pub fn load_map_data(pack_index: usize, map_index: usize) -> MapData {
    let bytes = maps::PACKS[pack_index].maps[map_index];

    let title = load_str(bytes, 0);
    let artist = load_str(bytes, 1);
    let id = load_str(bytes, 2);

    MapData { title, artist, id }
}

pub fn load_map_content(pack_index: usize, map_index: usize) -> MapContent {
    let bytes = maps::PACKS[pack_index].maps[map_index];

    let mut notes = VecDeque::with_capacity(bytes.len() / BINARY_ITEM_LENGTH);
    let mut events = VecDeque::with_capacity(bytes.len() / BINARY_ITEM_LENGTH);
    
    let mut i = BINARY_META_LENGTH;
    while i + BINARY_ITEM_LENGTH <= bytes.len() {
        let class_id: char = bytes[i] as char;

        let ms = u32::from_le_bytes([
            bytes[i+1], bytes[i+2], bytes[i+3], bytes[i+4]
        ]);

        match class_id {
            't' | 'h' => {
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
                        _ => unreachable!() // uhhh
                    }
                };

                notes.push_back(note);
            },
            'e' => {
                let event = Event {
                    ms,
                    class: match class_id {
                        'e' => {
                            EventClass::BGColor {
                                color: Color565::from_rgb(
                                    u16::from_le_bytes([bytes[i+5], bytes[i+6]]),
                                    u16::from_le_bytes([bytes[i+7], bytes[i+8]]),
                                    u16::from_le_bytes([bytes[i+9], bytes[i+10]])
                                )
                            }
                        },
                        _ => unreachable!()
                    }
                };

                events.push_back(event);
            },
            _ => panic!("invalid item class")
        }

        i += BINARY_ITEM_LENGTH;
    }


    MapContent { notes, events }
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

