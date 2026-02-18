use crate::eadk::display::*;

// note: editable constants at the top; calculated values after
pub mod display {
    use super::*;

    // editable
    pub const GAME_WIDTH: u16 = 150;    // width of the game
    pub const BORDER_SIZE: u16 = 5;     // size of the orange border

    // calculator screen
    pub const SCREEN_WIDTH: u16 = 320;
    pub const SCREEN_HEIGHT: u16 = 240;

    // game rect
    pub const MARGIN: u16 = (SCREEN_WIDTH - GAME_WIDTH) / 2;

    pub const GAME_RECT: ScreenRect = ScreenRect {
        x: MARGIN,
        y: 0,
        width: GAME_WIDTH,
        height: SCREEN_HEIGHT
    };

    // backdrop rect
    pub const BACKDROP_WIDTH: u16 = GAME_WIDTH + 2 * BORDER_SIZE;
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
    use super::*;

    // editable
    pub const PERFECT: u32 = 22;
    pub const GREAT: u32 = 45;
    pub const GOOD: u32 = 90;
    pub const MISS: u32 = 180;  // maximum time before an early hit counts as a miss
    
    pub const DRAW_AHEAD_MS: i32 = (spacing::Y_RANGE_F / spacing::PX_PER_MS) as i32;
}

// lane spacing + positioning
pub mod spacing {
    // note: buffer-related pixel values are in usize, unlike display related values
    // which are in u16, because the buffer is accessed through its array, not ScreenRects
    
    use super::*;
    
    // editable
    pub const NOTE_RADIUS: usize = 10;      // yeah
    pub const JUDGEMENT_LINE: usize = 25;   // height of judgement line above bottom
    pub const PX_PER_MS: f32 = 0.5;         // essentially scroll speed

    // buffer
    pub const BUFFER_WIDTH: usize = display::GAME_WIDTH as usize;
    pub const BUFFER_HEIGHT: usize = display::SCREEN_HEIGHT as usize;
    pub const BUFFER_SIZE: usize = BUFFER_WIDTH * BUFFER_HEIGHT;

    // lane size
    pub const LANE_WIDTH: usize = BUFFER_WIDTH;
    pub const LANE_WIDTH_F: f32 = LANE_WIDTH as f32;
    pub const LANE_HEIGHT: usize = BUFFER_HEIGHT;
    pub const LANE_HEIGHT_F: f32 = LANE_HEIGHT as f32;

    // note positioning - y
    // note: (haha) y = 0 is the TOP of the screen!!
    pub const Y0: usize = JUDGEMENT_LINE;
    pub const Y_RANGE: usize = LANE_HEIGHT - Y0;
    pub const Y_RANGE_F: f32 = Y_RANGE as f32;  // range ABOVE judgement line

    // note positioning - x
    pub const X_PADDING: usize = NOTE_RADIUS;   // technically editable?
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

    pub const WHITE: Color565 = Color565::from_rgb(255, 255, 255);
}

