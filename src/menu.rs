use crate::{
    map::{
        maps::*,
        storage::*
    },
    eadk::{
        *,
        display::*
    },
    constants::{
        controls::*,
        display::*,
        palette::*
    },
    game::{
        Game,
        Results
    }
};
calc_use!(alloc::format);

pub struct Menu {
    input: keyboard::InputManager,
    index: usize
}
impl Menu {
    pub fn new() -> Self {
        Self {
            input: keyboard::InputManager::new(),
            index: 0
        }
    }

    fn prev_index(&mut self) {
        if self.index != 0 { self.index -= 1; }
        self.draw_menu();
    }

    fn next_index(&mut self) {
        if self.index != N_MAPS-1 { self.index += 1; }
        self.draw_menu();
    }

    fn dramatic_pause() {
        wait_for_vblank();
        push_rect_uniform(
            SCREEN_RECT,
            BLACK
        );
        utils::refresh_simulator();
        time::wait_milliseconds(500);
    }

    fn start_game(&mut self) {
        Self::dramatic_pause();

        let mut game = Game::new(self.index);
        let results = game.main_loop();

        Self::dramatic_pause();

        match results {
            Some(r) => self.display_results(r),
            None => ()
        };

        self.draw_menu();
    }

    fn display_results(&mut self, results: Results) {
        let map_data = MAP_DATA[self.index];
        let name = map_data.name;
        let id = map_data.id;
        let length = name.len() as u16;
        let score = results.score;
        let high_score = load_high_score(id);

        let mut score_text = format!("score: {}", score);
        if score > high_score {
            score_text = format!("score: {} (high score!)", score);
            save_high_score(id, score);
        }

        let judge = format!(
            "   perfect: {}\n   great: {}\n   good: {}\n   miss: {}",
            results.perfect, results.great, results.good, results.miss
        );

        wait_for_vblank();

        push_rect_uniform(
            ScreenRect::new(
                0, RESULT_NAME_RECT_Y,
                3*TEXT_PADDING + length*TEXT_WIDTH,
                RESULT_NAME_RECT_HEIGHT
            ),
            ORANGE
        );

        draw_string(
            name,
            RESULT_NAME_POINT,
            true, WHITE, ORANGE
        );

        draw_string(
            &score_text,
            RESULT_SCORE_POINT,
            false, ORANGE, BLACK
        );

        draw_string(
            &judge,
            RESULT_JUDGE_POINT,
            false, ORANGE, BLACK
        );
        
        self.input.scan();
        while !(self.input.is_keydown(CONFIRM) || self.input.is_keydown(QUIT)) { self.input.scan(); }

        Self::dramatic_pause();

        self.input.scan();
    }

    fn draw_menu(&self) {
        let map_data = MAP_DATA[self.index];
        let name = map_data.name;
        let length = name.len() as u16;
        let score = load_high_score(map_data.id);

        wait_for_vblank();

        push_rect_uniform(MENU_NAME_MAX_RECT, BLACK);

        push_rect_uniform(
            ScreenRect::new(
                0, MENU_NAME_RECT_Y, 
                3*TEXT_PADDING + length*TEXT_WIDTH, 
                MENU_NAME_RECT_HEIGHT
            ),
            ORANGE
        );

        draw_string(
            name, 
            MENU_NAME_POINT,
            true, WHITE, ORANGE
        );

        draw_string(
            &format!("high score: {}", score),
            MENU_SCORE_POINT,
            false, ORANGE, BLACK
        );
    }

    fn check_clear_scores(&mut self) {
        Menu::dramatic_pause();

        let text = "delete all high scores?";
        let length = text.len() as u16;

        wait_for_vblank();

        push_rect_uniform(
            ScreenRect::new(
                0, MENU_NAME_RECT_Y,
                3*TEXT_PADDING + length*TEXT_WIDTH,
                RESULT_NAME_RECT_HEIGHT
            ),
            RED
        );

        draw_string(
            text,
            MENU_NAME_POINT,
            true, WHITE, RED
        );

        self.input.scan();
        while !(self.input.is_keydown(CONFIRM) || self.input.is_keydown(QUIT)) { self.input.scan(); }

        if self.input.is_keydown(CONFIRM) {
            clear_high_scores();
        }

        Self::dramatic_pause();

        self.input.scan();
        self.draw_menu();
    }

    pub fn main_loop(&mut self) {
        Self::dramatic_pause();
        self.input.scan();
        self.draw_menu();
        utils::wait_ok_released();

        while !self.input.is_keydown(HOME) {
            self.input.scan();
            match self.input.get_last_pressed() {
                Some(PREV) => self.prev_index(),
                Some(NEXT) => self.next_index(),
                Some(CONFIRM) => self.start_game(),
                Some(CLEAR) => self.check_clear_scores(),
                _ => ()
            };
        }
    }
}
