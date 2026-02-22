use crate::map::MapData;

pub const N_MAPS: usize = 8;

pub const MAPS: [&[u8]; N_MAPS] = [
    include_bytes!("../../assets/maps/mtb/black_world.mtb"),
    include_bytes!("../../assets/maps/mtb/new_magic_wand.mtb"),
    include_bytes!("../../assets/maps/mtb/test.mtb"),
    include_bytes!("../../assets/maps/mtb/competent.mtb"),
    include_bytes!("../../assets/maps/mtb/see_you_again.mtb"),
    include_bytes!("../../assets/maps/mtb/balloon.mtb"),
    include_bytes!("../../assets/maps/mtb/here_comes_a_thought.mtb"),
    include_bytes!("../../assets/maps/mtb/stronger_than_you.mtb")
];

pub const MAP_DATA: [MapData; N_MAPS] = [
    MapData { title: "Letter to the Black World", artist: "Frog96", id: "black_world" },
    MapData { title: "NEW MAGIC WAND", artist: "Tyler, The Creator", id: "new_magic_wand" },
    MapData { title: "test! ignore <3", artist: "the metronome test team", id: "test"},
    MapData { title: "JUST BE COMPETENT", artist: "r u s s e l b u c k", id: "competent" },
    MapData { title: "See You Again", artist: "Tyler, The Creator feat. Kali Uchis", id: "see_you_again" },
    MapData { title: "Balloon", artist: "Tyler, The Creator feat. Doechii", id: "balloon" },
    MapData { title: "Here Comes a Thought", artist: "Steven Universe", id: "here_comes_a_thought" },
    MapData { title: "Stronger Than You", artist: "Steven Universe", id: "stronger_than_you" }
];

// UPDATE N_MAPS!!

