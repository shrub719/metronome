use crate::{
    map::{
        *,
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
    accent: Color565,
    pack_index: usize,
    pack_length: usize,
    map_index: usize
}
impl Menu {
    pub fn new() -> Self {
        Self {
            input: keyboard::InputManager::new(),
            accent: PACKS[0].color,
            pack_index: 0,
            pack_length: PACKS[0].maps.len(),
            map_index: 0
        }
    }

    fn draw_background() {
        push_rect_uniform(SCREEN_RECT, GREY);
    }

    fn prev_map(&mut self) {
        if self.map_index == 0 { 
            self.map_index = self.pack_length-1;
        } else {
            self.map_index -= 1; 
        }

        self.draw_menu();
    }

    fn next_map(&mut self) {
        if self.map_index == self.pack_length-1 { 
            self.map_index = 0
        } else { 
            self.map_index += 1; 
        }

        self.draw_menu();
    }

    fn pack_update(&mut self) {
        self.accent = PACKS[self.pack_index].color;
        self.pack_length = PACKS[self.pack_index].maps.len();
        self.map_index = 0;
    }

    fn prev_pack(&mut self) {
        if self.pack_index == 0 {
            self.pack_index = N_PACKS-1;
        } else {
            self.pack_index -= 1;
        }

        self.pack_update();
        self.draw_menu();
    }

    fn next_pack(&mut self) {
        if self.pack_index == N_PACKS-1 {
            self.pack_index = 0;
        } else {
            self.pack_index += 1;
        }

        self.pack_update();
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

        let mut game = Game::new(self.pack_index, self.map_index, self.accent);
        let results = game.main_loop();

        Self::dramatic_pause();

        match results {
            Some(r) => self.display_results(r),
            None => ()
        };

        self.draw_menu();
    }

    fn display_results(&mut self, results: Results) {
        let map_data = load_map_data(self.pack_index, self.map_index);
        let title = map_data.title;
        let artist = map_data.artist;
        let id = map_data.id;
        let length = title.len() as u16;
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

        Menu::draw_background();

        push_rect_uniform(
            ScreenRect::new(
                0, RESULT_TITLE_RECT_Y,
                3*TEXT_PADDING + length*TEXT_WIDTH,
                RESULT_TITLE_RECT_HEIGHT
            ),
            self.accent
        );

        draw_string(
            title,
            RESULT_TITLE_POINT,
            true, WHITE, self.accent
        );

        draw_string(
            artist,
            RESULT_ARTIST_POINT,
            false, self.accent, GREY
        );

        draw_string(
            &score_text,
            RESULT_SCORE_POINT,
            false, self.accent, GREY
        );

        draw_string(
            &judge,
            RESULT_JUDGE_POINT,
            false, self.accent, GREY
        );
        
        self.input.scan();
        while !(self.input.is_keydown(CONFIRM) || self.input.is_keydown(QUIT)) { self.input.scan(); }
    }

    fn draw_menu(&self) {
        let map_data = load_map_data(self.pack_index, self.map_index);
        let title = map_data.title;
        let artist = map_data.artist;
        let length = title.len() as u16;
        let score = load_high_score(map_data.id);

        wait_for_vblank();

        Menu::draw_background();

        push_rect_uniform(
            ScreenRect::new(
                0, MENU_TITLE_RECT_Y, 
                3*TEXT_PADDING + length*TEXT_WIDTH, 
                MENU_TITLE_RECT_HEIGHT
            ),
            self.accent
        );

        draw_string(
            title, 
            MENU_TITLE_POINT,
            true, WHITE, self.accent
        );

        draw_string(
            artist,
            MENU_ARTIST_POINT,
            false, self.accent, GREY
        );

        draw_string(
            &format!("high score: {}", score),
            MENU_SCORE_POINT,
            false, self.accent, GREY
        );
    }

    fn check_clear_scores(&mut self) {
        Menu::dramatic_pause();

        let text = "delete all high scores?";
        let length = text.len() as u16;

        wait_for_vblank();

        Menu::draw_background();

        push_rect_uniform(
            ScreenRect::new(
                0, MENU_TITLE_RECT_Y,
                3*TEXT_PADDING + length*TEXT_WIDTH,
                RESULT_TITLE_RECT_HEIGHT
            ),
            RED
        );

        draw_string(
            text,
            MENU_TITLE_POINT,
            true, WHITE, RED
        );

        self.input.scan();
        while !(self.input.is_keydown(CONFIRM) || self.input.is_keydown(QUIT)) { self.input.scan(); }

        if self.input.is_keydown(CONFIRM) {
            clear_high_scores();
        }

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
                Some(PREV_MAP) => self.prev_map(),
                Some(NEXT_MAP) => self.next_map(),
                Some(PREV_PACK) => self.prev_pack(),
                Some(NEXT_PACK) => self.next_pack(),
                Some(CONFIRM) => self.start_game(),
                Some(CLEAR) => self.check_clear_scores(),
                _ => ()
            };
        }
    }
}
