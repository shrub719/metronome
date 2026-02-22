pub const N_MAPS: usize = 9;

pub const MAPS: [&[u8]; N_MAPS] = [
    include_bytes!("../../assets/maps/mtb/mtn_01.mtb"),
    include_bytes!("../../assets/maps/mtb/black_world.mtb"),
    include_bytes!("../../assets/maps/mtb/new_magic_wand.mtb"),
    include_bytes!("../../assets/maps/mtb/test.mtb"),
    include_bytes!("../../assets/maps/mtb/competent.mtb"),
    include_bytes!("../../assets/maps/mtb/see_you_again.mtb"),
    include_bytes!("../../assets/maps/mtb/balloon.mtb"),
    include_bytes!("../../assets/maps/mtb/here_comes_a_thought.mtb"),
    include_bytes!("../../assets/maps/mtb/stronger_than_you.mtb")
];

// UPDATE N_MAPS!!

