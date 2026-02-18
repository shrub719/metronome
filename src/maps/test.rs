use crate::game::map::*;
calc_use!(alloc::vec);

pub fn create_test_map() -> Map {
    Map {
        notes: vec!(
            Note {
                beat: 0.0,
                class: NoteClass::Tap { x: 0.5 }
            },
            Note {
                beat: 1.0,
                class: NoteClass::DoubleTap { x1: 0.0, x2: 1.0 }
            }
        ),
        events: vec!()
    }
}
