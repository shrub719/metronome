use crate::eadk::display::Color565;
calc_use!(alloc::vec::Vec);

pub enum NoteClass {
    Tap { x: f32 },
    DoubleTap { x1: f32, x2: f32 },
}

pub struct Note {
    pub beat: f32,
    pub class: NoteClass
}

pub enum EventClass {
    Backlight { fraction: f32, duration: f32 },
    Shake,
    BGColor { color: Color565, duration: f32 }
}

pub struct Event {
    pub beat: f32,
    pub class: EventClass
}

pub struct Map {
    pub notes: Vec<Note>,
    pub events: Vec<Event>
}
