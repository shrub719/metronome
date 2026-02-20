use crate::map::MapData;

pub static MAPS: [&[u8]; 2] = [
    include_bytes!("../../target/maps/black_world.mtb"),
    include_bytes!("../../target/maps/new_magic_wand.mtb"),
];

pub static MAP_DATA: [MapData; 2] = [
    MapData { name: "Letter to the Black World", id: "black_world" },
    MapData { name: "New Magic Wand", id: "new_magic_wand" }
];
