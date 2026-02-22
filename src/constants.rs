use crate::eadk::display::*;
use crate::eadk::keyboard::*;

pub mod file {
    // editable?
    pub const HIGH_SCORE_FILE: &str = "metronome_high_scores.mth";
    // .py makes it visible, but i don't want tampering (mostly
    // cause it causes errors)

    // in bytes
    pub const BINARY_STR_LENGTH: usize = 64;
    pub const BINARY_META_LENGTH: usize = 3 * BINARY_STR_LENGTH;
    pub const BINARY_NOTE_LENGTH: usize = 13;
}

pub mod controls {
    use super::*;

    // editable
    pub const QUIT: Key = Key::Back;
    pub const PREV: Key = Key::Up;
    pub const NEXT: Key = Key::Down;
    pub const CONFIRM: Key = Key::Ok;
    pub const HOME: Key = Key::Home;  
    pub const CLEAR: Key = Key::Backspace;
}

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
    pub const GAME_MARGIN: u16 = (SCREEN_WIDTH - GAME_WIDTH) / 2;

    pub const GAME_RECT: ScreenRect = ScreenRect {
        x: GAME_MARGIN,
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

    // text
    pub const TEXT_HEIGHT: u16 = 20;
    pub const SMALL_TEXT_HEIGHT: u16 = 14;
    pub const TEXT_WIDTH: u16 = 10;
    pub const TEXT_PADDING: u16 = 5;

    // judgement
    pub const UI_JUDGEMENT_Y: u16 = SCREEN_HEIGHT - TEXT_HEIGHT;
    pub const UI_JUDGEMENT_WIDTH: u16 = BACKDROP_MARGIN;
    pub const UI_SCORE_X: u16 = GAME_MARGIN + GAME_WIDTH + BORDER_SIZE;
    pub const UI_SCORE_WIDTH: u16 = BACKDROP_MARGIN;

    pub const UI_JUDGEMENT_RECT: ScreenRect = ScreenRect {
        x: 0,
        y: UI_JUDGEMENT_Y,
        width: UI_JUDGEMENT_WIDTH,
        height: TEXT_HEIGHT
    };
    pub const UI_SCORE_RECT: ScreenRect = ScreenRect {
        x: UI_SCORE_X,
        y: 0,
        width: UI_SCORE_WIDTH,
        height: TEXT_HEIGHT
    };

    pub const UI_JUDGEMENT_POINT: ScreenPoint = ScreenPoint {
        x: TEXT_PADDING, y: UI_JUDGEMENT_Y
    };
    pub const UI_SCORE_POINT: ScreenPoint = ScreenPoint {
        x: UI_SCORE_X + TEXT_PADDING, y: 0
    };

    #[allow(unused)]
    pub const UI_MS_POINT: ScreenPoint = ScreenPoint {
        x: 0, y: 0
    };

    // menu
    pub const MENU_NAME_Y: u16 = SCREEN_HEIGHT / 2 - TEXT_HEIGHT;
    pub const MENU_ARTIST_Y: u16 = MENU_NAME_Y - SMALL_TEXT_HEIGHT - TEXT_PADDING;
    pub const MENU_SCORE_Y: u16 = MENU_NAME_Y + TEXT_HEIGHT + TEXT_PADDING * 3;

    pub const MENU_NAME_RECT_Y: u16 = MENU_NAME_Y;
    pub const MENU_NAME_RECT_HEIGHT: u16 = TEXT_HEIGHT + 2 * TEXT_PADDING;

    pub const MENU_NAME_POINT: ScreenPoint = ScreenPoint {
        x: TEXT_PADDING, y: MENU_NAME_Y + TEXT_PADDING
    };

    pub const MENU_ARTIST_POINT: ScreenPoint = ScreenPoint {
        x: TEXT_PADDING, y: MENU_ARTIST_Y
    };

    pub const MENU_SCORE_POINT: ScreenPoint = ScreenPoint {
        x: TEXT_PADDING, y: MENU_SCORE_Y
    };

    // results
    pub const RESULT_NAME_Y: u16 = SCREEN_HEIGHT / 4 - TEXT_HEIGHT;
    pub const RESULT_ARTIST_Y: u16 = RESULT_NAME_Y - SMALL_TEXT_HEIGHT - TEXT_PADDING;
    pub const RESULT_SCORE_Y: u16 = RESULT_NAME_Y + TEXT_HEIGHT + TEXT_PADDING * 3;
    pub const RESULT_JUDGE_Y: u16 = SCREEN_HEIGHT / 3 * 2;

    pub const RESULT_NAME_RECT_Y: u16 = RESULT_NAME_Y;
    pub const RESULT_NAME_RECT_HEIGHT: u16 = TEXT_HEIGHT + 2 * TEXT_PADDING;

    pub const RESULT_NAME_POINT: ScreenPoint = ScreenPoint {
        x: TEXT_PADDING, y: RESULT_NAME_Y + TEXT_PADDING
    };

    pub const RESULT_ARTIST_POINT: ScreenPoint = ScreenPoint {
        x: TEXT_PADDING, y: RESULT_ARTIST_Y
    };

    pub const RESULT_SCORE_POINT: ScreenPoint = ScreenPoint {
        x: TEXT_PADDING, y: RESULT_SCORE_Y
    };

    pub const RESULT_JUDGE_POINT: ScreenPoint = ScreenPoint {
        x: 0, y: RESULT_JUDGE_Y
    };
}

// judgement times
pub mod judgement { 
    use super::*;

    // editable
    pub const PERFECT: i32 = 22;
    pub const GREAT: i32 = 45;
    pub const GOOD: i32 = 90;
    pub const MISS: i32 = 135;      // maximum time before an early hit counts as a miss
    pub const PERFECT_SCORE: u32 = 100;
    pub const GREAT_SCORE: u32 = 75;
    pub const GOOD_SCORE: u32 = 30;
    pub const MISS_SCORE: u32 = 0;
    pub const MAX_HITS: u8 = 3;  // max number of hits detected per frame
    
    pub const DRAW_AHEAD_MS: i32 = (spacing::Y_RANGE_F / spacing::PX_PER_MS) as i32;
}

// lane spacing + positioning
pub mod spacing {
    // note: buffer-related pixel values are in usize, unlike display related values
    // which are in u16, because the buffer is accessed through its array, not ScreenRects
    
    use super::*;
    
    // editable
    pub const NOTE_RADIUS: usize = 10;      // yeah
    pub const TAIL_RADIUS: usize = 5;
    pub const JUDGEMENT_LINE_HEIGHT: usize = 25;   // height of judgement line above bottom
    pub const PX_PER_MS: f32 = 0.38;         // essentially scroll speed

    // buffer
    pub const BUFFER_WIDTH: usize = display::GAME_WIDTH as usize;
    pub const BUFFER_HEIGHT: usize = display::SCREEN_HEIGHT as usize;
    pub const BUFFER_SIZE: usize = BUFFER_WIDTH * BUFFER_HEIGHT;

    // lane size
    pub const LANE_WIDTH: usize = BUFFER_WIDTH;
    pub const LANE_HEIGHT: usize = BUFFER_HEIGHT;

    // note positioning - y
    // note: (haha) y = 0 is the TOP of the screen!!
    pub const JUDGEMENT_LINE: usize = LANE_HEIGHT - JUDGEMENT_LINE_HEIGHT;
    pub const Y0: usize = JUDGEMENT_LINE;
    pub const Y_RANGE: usize = Y0;
    pub const Y_RANGE_F: f32 = Y_RANGE as f32;  // range ABOVE judgement line

    // note positioning - x
    pub const X_PADDING: usize = NOTE_RADIUS;   // technically editable?
    pub const X0: usize = X_PADDING + NOTE_RADIUS;
    pub const X_RANGE: usize = LANE_WIDTH - 2 * (X_PADDING + NOTE_RADIUS);
    pub const X_RANGE_F: f32 = X_RANGE as f32;

    pub const NOTE_RADIUS_I: isize = NOTE_RADIUS as isize;
    pub const TAIL_RADIUS_I: isize = TAIL_RADIUS as isize;
}

// colors
pub mod palette {
    use super::*;

    pub const ACCENT: Color565 = Color565::from_rgb(255, 183, 52);
    //      is the new
    pub const BLACK: Color565 = Color565::from_rgb(0, 0, 0);

    pub const WHITE: Color565 = Color565::from_rgb(255, 255, 255);
    pub const GREY: Color565 = Color565::from_rgb(17, 17, 17);

    pub const RED: Color565 = Color565::from_rgb(255, 52, 52);
}

