use crate::eadk::display::Color565;
calc_use!(alloc::vec::Vec);

#[derive(Clone, Copy)]
pub enum NoteClass {
    Tap { x: f32 },
    DoubleTap { x1: f32, x2: f32 },
}

#[derive(Clone, Copy)]
pub struct Note {
    pub ms: u32,
    pub class: NoteClass
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
    pub notes: Vec<Note>,
    pub events: Vec<Event>
}
