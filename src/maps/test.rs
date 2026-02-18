use crate::{
    game::map::{
        *,
        NoteClass::*
    },
    note, notes
};
calc_use!(alloc::vec);
calc_use!(alloc::collections::vec_deque::VecDeque);
sim_use!(std::collections::VecDeque);

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

pub fn polyrhythms() -> Map {
    let mut notes = VecDeque::<Note>::new();

    for i in 1..9 {
        let start = 1000 * i;
        notes.push_back(note!(start, 0.0));
        notes.push_back(note!(start + 500, 0.0));
    }
    for i in 1..9 {
        let start = 1000 * i;
        notes.push_back(note!(start, 1.0));
        notes.push_back(note!(start + 333, 1.0));
        notes.push_back(note!(start + 666, 1.0));
    }

    notes.make_contiguous().sort_by_key(|note| note.ms);

    Map {
        notes,
        events: vec!().into()
    }
}
