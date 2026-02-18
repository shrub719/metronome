use crate::eadk::display::*;

pub mod display {
    use super::*;

    // calculator screen
    pub const SCREEN_WIDTH: u16 = 320;
    pub const SCREEN_HEIGHT: u16 = 240;

    // game rect
    pub const GAME_WIDTH: u16 = 150;
    pub const MARGIN: u16 = (SCREEN_WIDTH - GAME_WIDTH) / 2;

    pub const GAME_RECT: ScreenRect = ScreenRect {
        x: MARGIN,
        y: 0,
        width: GAME_WIDTH,
        height: SCREEN_HEIGHT
    };

    // backdrop rect
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

// judgement times
pub mod judgement { 
    pub const PERFECT: u32 = 22;
    pub const GREAT: u32 = 45;
    pub const GOOD: u32 = 90;
    pub const MISS: u32 = 180;
}

// lane spacing + positioning
pub mod lane {
    use super::*;

    // buffer
    pub const BUFFER_WIDTH: usize = display::GAME_WIDTH as usize;
    pub const BUFFER_HEIGHT: usize = display::SCREEN_HEIGHT as usize;
    pub const BUFFER_SIZE: usize = BUFFER_WIDTH * BUFFER_HEIGHT;

    // ms captured relative to judgement line
    pub const ABOVE_MS: u32 = 800;
    pub const ABOVE_MS_F: f32 = HEIGHT_MS as f32;
    pub const BELOW_MS: u32 = judgement::GOOD;
    pub const BELOW_MS_F: f32 = BELOW_MS as f32;
    pub const TOTAL_MS: u32 = ABOVE_MS + BELOW_MS;
    pub const TOTAL_MS_F: f32 = TOTAL_MS as f32;

    // lane size
    pub const LANE_WIDTH: usize = BUFFER_WIDTH;
    pub const LANE_WIDTH_F: f32 = LANE_WIDTH as f32;
    pub const LANE_HEIGHT: usize = BUFFER_HEIGHT;
    pub const LANE_HEIGHT_F: f32 = LANE_HEIGHT as f32;
    pub const PX_PER_MS: f32 = LANE_HEIGHT_F / TOTAL_MS_F;

    // note size
    pub const NOTE_RADIUS: usize = 10;

    // note positioning - y
    pub const Y0: usize = (BELOW_MS_F * PX_PER_MS - NOTE_RADIUS) as usize;

    // note positioning - x
    pub const X_PADDING: usize = NOTE_RADIUS;
    pub const X0: usize = X_PADDING + NOTE_RADIUS;
    pub const X_RANGE: usize = BUFFER_WIDTH - 2 * (X_PADDING + NOTE_RADIUS);
    pub const X_RANGE_F: f32 = X_RANGE as f32;
}

// colors
pub mod palette {
    use super::*;

    pub const ORANGE: Color565 = Color565::from_rgb(255, 183, 52);
    //      is the new
    pub const BLACK: Color565 = Color565::from_rgb(17, 17, 17);
}

