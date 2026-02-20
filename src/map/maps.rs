use crate::map::MapData;

pub const N_MAPS: usize = 3;

pub const MAPS: [&[u8]; N_MAPS] = [
    include_bytes!("../../target/maps/black_world.mtb"),
    include_bytes!("../../target/maps/new_magic_wand.mtb"),
    include_bytes!("../../target/maps/test.mtb")
];

pub const MAP_DATA: [MapData; N_MAPS] = [
    MapData { name: "Letter to the Black World", id: "black_world" },
    MapData { name: "New Magic Wand", id: "new_magic_wand" },
    MapData { name: "Test! Ignore <3", id: "test"}
];
