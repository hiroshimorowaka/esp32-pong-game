use alloc::sync::Arc;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use esp_hal::{
    gpio::{GpioPin, Input, PullDown},
    prelude::_embedded_hal_digital_v2_InputPin,
};

use crate::gamestate::{Bar, GameState};

pub struct Player {
    pub bar: Bar,
    pub score: u32,
}

impl Player {
    pub fn increase_score(&mut self) {
        self.score += 1;
    }
}

pub async fn calcule_scores(game_state: &'static Arc<Mutex<CriticalSectionRawMutex, GameState>>) {
    let mut game_state = game_state.lock().await;

    if game_state.ball.position_x < 5 {
        game_state.player_2.increase_score();
        game_state.ball.reset_ball(1);
    } else if game_state.ball.position_x >= 120 {
        game_state.player_1.increase_score();
        game_state.ball.reset_ball(-1);
    }
}

pub async fn move_player(
    game_state: &'static Arc<Mutex<CriticalSectionRawMutex, GameState>>,
    p1_up_button: Arc<GpioPin<Input<PullDown>, 14>>,
    p1_down_button: Arc<GpioPin<Input<PullDown>, 12>>,
    p2_up_button: Arc<GpioPin<Input<PullDown>, 32>>,
    p2_down_button: Arc<GpioPin<Input<PullDown>, 33>>,
) {
    let mut game_state = game_state.lock().await;

    if p1_up_button.is_high().unwrap_or(false) {
        game_state.player_1.bar.position_y = (game_state.player_1.bar.position_y - 2).clamp(10, 54);
    }
    if p1_down_button.is_high().unwrap_or(false) {
        game_state.player_1.bar.position_y = (game_state.player_1.bar.position_y + 2).clamp(10, 54);
    }

    if p2_up_button.is_high().unwrap_or(false) {
        game_state.player_2.bar.position_y = (game_state.player_2.bar.position_y - 2).clamp(10, 54);
    }
    if p2_down_button.is_high().unwrap_or(false) {
        game_state.player_2.bar.position_y = (game_state.player_2.bar.position_y + 2).clamp(10, 54);
    }
}
