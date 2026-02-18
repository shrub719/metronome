use crate::{
    game::map::{
        *,
        NoteClass::*
    },
    notes
};
calc_use!(alloc::vec);

pub fn create_test_map() -> Map {
    Map {
        notes: notes!(
        (750, 0.0),
        (1000, 0.5),
        (1000, 1.0),
        (1750, 1.0 ),
        (2000, 0.5),
        (2250, 0.0),
        (2750, 0.5),
        (3000, 0.0),
        (3000, 1.0),
        ).into(),
        events: vec!().into()
    }
}
