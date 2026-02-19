use crate::{
    game::map::*,
    note
};
calc_use!(alloc::vec);
calc_use!(alloc::collections::vec_deque::VecDeque);
sim_use!(std::collections::VecDeque);

pub fn polyrhythms() -> Map {
    let mut notes = VecDeque::<Note>::new();

    for i in 1..9 {
        let start = 1000 * i;
        notes.push_back(note!(tap, start, 0.0));
        notes.push_back(note!(tap, start + 500, 0.0));
    }
    for i in 1..9 {
        let start = 1000 * i;
        notes.push_back(note!(tap, start, 1.0));
        notes.push_back(note!(tap, start + 333, 1.0));
        notes.push_back(note!(tap, start + 666, 1.0));
    }

    notes.make_contiguous().sort_by_key(|note| note.ms);

    Map {
        notes,
        events: vec!().into()
    }
}

pub fn test() -> Map {
    Map {
        notes: [
            note!(tap, 1000, 0.2),
            note!(tap, 1500, 0.4),
            note!(hold, 2000, 0.8, 1000),
            note!(tap, 2750, 0.2),
            note!(tap, 3500, 0.6),
            note!(hold, 4000, 0.2, 500),
            note!(hold, 4500, 0.8, 500),
            note!(hold, 5000, 0.5, 1000)
        ].into(),
        events: vec!().into()
    }
}
