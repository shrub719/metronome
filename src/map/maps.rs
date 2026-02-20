use crate::map::MapData;

pub const N_MAPS: usize = 2;

pub const MAPS: [&[u8]; N_MAPS] = [
    include_bytes!("../../target/maps/black_world.mtb"),
    include_bytes!("../../target/maps/new_magic_wand.mtb"),
];

pub const MAP_DATA: [MapData; 2] = [
    MapData { name: "Letter to the Black World", id: "black_world" },
    MapData { name: "New Magic Wand", id: "new_magic_wand" }
];
