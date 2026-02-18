use crate::game::map::{
    *,
    NoteClass::*
};
calc_use!(alloc::vec);

pub fn create_test_map() -> Map {
    Map {
        notes: vec!(
            Note {
                ms: 1000,
                class: Tap { x: 0.5 }
            },
            Note {
                ms: 2000,
                class: DoubleTap { x1: 0.0, x2: 1.0 }
            },
            Note {
                ms: 5000,
                class: Tap { x: 0.7 }
            }
        ).into(),
        events: vec!().into()
    }
}
