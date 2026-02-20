use crate::map::MapData;

pub const N_MAPS: usize = 8;

pub const MAPS: [&[u8]; N_MAPS] = [
    include_bytes!("../../target/maps/black_world.mtb"),
    include_bytes!("../../target/maps/new_magic_wand.mtb"),
    include_bytes!("../../target/maps/test.mtb"),
    include_bytes!("../../target/maps/competent.mtb"),
    include_bytes!("../../target/maps/see_you_again.mtb"),
    include_bytes!("../../target/maps/balloon.mtb"),
    include_bytes!("../../target/maps/here_comes_a_thought.mtb"),
    include_bytes!("../../target/maps/stronger_than_you.mtb")
];

pub const MAP_DATA: [MapData; N_MAPS] = [
    MapData { title: "Letter to the Black World", artist: "Frog96", id: "black_world" },
    MapData { title: "New Magic Wand", artist: "Tyler, The Creator", id: "new_magic_wand" },
    MapData { title: "Test! Ignore <3", artist: "the metronome test team", id: "test"},
    MapData { title: "JUST BE COMPETENT", artist: "r u s s e l b u c k", id: "competent" },
    MapData { title: "See You Again", artist: "Tyler, The Creator feat. Kali Uchis", id: "see_you_again" },
    MapData { title: "Balloon", artist: "Tyler, The Creator feat. Doechii", id: "balloon" },
    MapData { title: "Here Comes a Thought", artist: "Steven Universe", id: "here_comes_a_thought" },
    MapData { title: "Stronger Than You", artist: "Steven Universe", id: "stronger_than_you" }
];

// UPDATE N_MAPS!!

