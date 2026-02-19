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
            note!(tap, 1000, 1.0),
            note!(hold, 1000, 0.0, 1800),
            note!(tap, 1250, 0.5),
            note!(tap, 1500, 0.5),
            note!(tap, 1750, 1.0),
            note!(tap, 2000, 0.0),
            note!(hold, 2000, 1.0, 2800),
            note!(tap, 2500, 0.6),
            note!(tap, 2625, 0.4),
            note!(tap, 2750, 0.2),
            note!(tap, 2875, 0.0),
            note!(tap, 3000, 1.0),
            note!(hold, 3000, 0.0, 3400),
            note!(tap, 3500, 0.2),
            note!(hold, 3500, 0.8, 3900),
            note!(tap, 4000, 0.6),
            note!(hold, 4000, 0.4, 4500)
        ].into(),
        events: vec!().into()
    }
}
