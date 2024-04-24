use alloc::sync::Arc;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};

use crate::{gamestate::GameState, players::Player};

pub struct Ball {
    pub position_x: i32,
    pub position_y: i32,

    pub direction_x: i32,
    pub direction_y: i32,

    pub diameter: i32,
    pub speed: i32,

    pub touches: u32,
}

impl Ball {
    pub fn move_ball(&mut self) {
        self.position_x = (self.position_x
            + (self.direction_x * (self.speed + (self.touches / 2) as i32).clamp(2, 8)))
        .clamp(0, 120);
        self.position_y = (self.position_y + self.direction_y * self.speed).clamp(10, 60);
    }

    pub fn change_x_direction(&mut self) {
        self.direction_x *= -1;
    }

    pub fn change_y_direction(&mut self) {
        self.direction_y *= -1;
    }

    pub fn reset_ball(&mut self, direction: i32) {
        self.direction_x = direction;
        self.position_x = 60;
        self.position_y = 32;

        self.speed = 2;
        self.touches = 0;
    }

    pub fn check_collision_with_bar(&self, player: &Player) -> bool {
        self.position_x < player.bar.position_x + player.bar.width
            && self.position_x + self.diameter > player.bar.position_x
            && self.position_y < player.bar.position_y + player.bar.height
            && self.position_y + self.diameter > player.bar.position_y
    }
}

pub async fn calcule_ball_position(
    game_state: &'static Arc<Mutex<CriticalSectionRawMutex, GameState>>,
) {
    let mut game_state = game_state.lock().await;

    game_state.ball.move_ball();

    match game_state.ball.position_x {
        0 | 120..=130 => game_state.ball.change_x_direction(),
        _ => {}
    }
    match game_state.ball.position_y {
        10 | 60 => game_state.ball.change_y_direction(),
        _ => {}
    }

    match game_state
        .ball
        .check_collision_with_bar(&game_state.player_1)
    {
        true => {
            game_state.ball.change_x_direction();

            game_state.ball.touches += 1;
        }
        false => {}
    }

    match game_state
        .ball
        .check_collision_with_bar(&game_state.player_2)
    {
        true => {
            game_state.ball.change_x_direction();

            game_state.ball.touches += 1;
        }
        false => {}
    }
}
