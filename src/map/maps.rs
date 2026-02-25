use crate::map::*;
use crate::eadk::display::Color565;

const METRONOME: MapPack = MapPack {
    maps: &[
        include_bytes!("../../assets/maps/metronome/mtn_01.mtb"),
        include_bytes!("../../assets/maps/metronome/mtn_02.mtb"),
        include_bytes!("../../assets/maps/metronome/test.mtb"),
    ],
    color: Color565::from_rgb(255, 183, 52)
};

const OSU: MapPack = MapPack {
    maps: &[
        include_bytes!("../../assets/maps/osu/black_world.mtb"),
        include_bytes!("../../assets/maps/osu/new_magic_wand.mtb"),
        include_bytes!("../../assets/maps/osu/see_you_again.mtb"),
        include_bytes!("../../assets/maps/osu/balloon.mtb"),
        include_bytes!("../../assets/maps/osu/competent.mtb"),
        include_bytes!("../../assets/maps/osu/here_comes_a_thought.mtb"),
        include_bytes!("../../assets/maps/osu/stronger_than_you.mtb"),
    ],
    color: Color565::from_rgb(255, 84, 155)
};

#[cfg(feature = "ext")]
const EXT: MapPack = MapPack {
    maps: &[include_bytes!("../../assets/maps/metronome/test.mtb")],
    color: Color565::from_rgb(255, 183, 52)
};

#[cfg(feature = "ext")]
pub const PACKS: [MapPack; 3] = [
    METRONOME, OSU,
    EXT,
];

#[cfg(not(feature = "ext"))]
pub const PACKS: [MapPack; 2] = [
    METRONOME, OSU,
];

pub const N_PACKS: usize = PACKS.len();

