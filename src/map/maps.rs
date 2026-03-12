use crate::map::*;
use crate::eadk::display::Color565;

const METRONOME: MapPack = MapPack {
    maps: &[
        include_bytes!("../../assets/maps/metronome/mtn_00.mtb"),
        include_bytes!("../../assets/maps/metronome/mtn_01.mtb"),
        include_bytes!("../../assets/maps/metronome/mtn_02.mtb"),
        include_bytes!("../../assets/maps/metronome/mtn_03.mtb"),
        include_bytes!("../../assets/maps/metronome/test.mtb"),
    ],
    color: Color565::from_rgb(255, 183, 52)
};

const VOCALOID: MapPack = MapPack {
    maps: &[
        include_bytes!("../../assets/maps/vocaloid/ego.mtb"),
        include_bytes!("../../assets/maps/vocaloid/birdbrain.mtb"),
        include_bytes!("../../assets/maps/vocaloid/spoken_for.mtb"),
        include_bytes!("../../assets/maps/vocaloid/static.mtb"),
        include_bytes!("../../assets/maps/vocaloid/black_world.mtb")
    ],
    color: Color565::from_rgb(0, 221, 192)
};

const OSU: MapPack = MapPack {
    maps: &[
        include_bytes!("../../assets/maps/osu/new_magic_wand.mtb"),
        include_bytes!("../../assets/maps/osu/see_you_again.mtb"),
        include_bytes!("../../assets/maps/osu/here_comes_a_thought.mtb"),
        include_bytes!("../../assets/maps/osu/stronger_than_you.mtb"),
        include_bytes!("../../assets/maps/osu/beatdown.mtb")
    ],
    color: Color565::from_rgb(255, 102, 170)
};

#[cfg(feature = "ext")]
const EXT: MapPack = MapPack {
    maps: &[include_bytes!("../../assets/maps/metronome/test.mtb")],
    color: Color565::from_rgb(180, 180, 180)
};

#[cfg(not(feature = "map-test"))]
#[cfg(feature = "ext")]
pub const PACKS: [MapPack; 4] = [
    METRONOME, VOCALOID, OSU,
    EXT,
];

#[cfg(feature = "map-test")]
pub const PACKS: [MapPack; 1] = [EXT];

#[cfg(not(feature = "ext"))]
pub const PACKS: [MapPack; 3] = [
    METRONOME, VOCALOID, OSU
];

pub const N_PACKS: usize = PACKS.len();

