pub mod gameplay {
    pub const DEFAULT_SCROLL_SPEED: f32 = 1.0;   // ???
}

pub mod display {
    use crate::eadk::display::*;

    pub const SCREEN_WIDTH: u16 = 320;
    pub const SCREEN_HEIGHT: u16 = 240;
    
    pub const LANE_WIDTH: u16 = 150;
    pub const MARGIN: u16 = (SCREEN_WIDTH - LANE_WIDTH) / 2;
    pub const BUFFER_SIZE: usize = (LANE_WIDTH * SCREEN_HEIGHT) as usize;

    pub const LANE_RECT: ScreenRect = ScreenRect {
        x: MARGIN,
        y: 0,
        width: LANE_WIDTH,
        height: SCREEN_HEIGHT
    };

    pub const BORDER_SIZE: u16 = 5;
    pub const BACKDROP_WIDTH: u16 = 150 + 2 * BORDER_SIZE;
    pub const BACKDROP_MARGIN: u16 = (SCREEN_WIDTH - BACKDROP_WIDTH) / 2;
    pub const BACKDROP_RECT: ScreenRect = ScreenRect {
        x: BACKDROP_MARGIN,
        y: 0,
        width: BACKDROP_WIDTH,
        height: SCREEN_HEIGHT
    };
}

pub mod palette {
    use crate::eadk::display::Color565;

    pub const ORANGE: Color565 = Color565::from_rgb(255, 183, 52);
    //      is the new
    pub const BLACK: Color565 = Color565::from_rgb(17, 17, 17);
}
