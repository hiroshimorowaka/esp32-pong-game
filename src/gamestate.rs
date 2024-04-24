use crate::{ball::Ball, players::Player};

pub struct GameState {
    pub ball: Ball,
    pub player_1: Player,
    pub player_2: Player,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            player_1: Player {
                bar: Bar {
                    position_x: 10,
                    position_y: 32,
                    width: 4,
                    height: 15,
                },
                score: 0,
            },
            player_2: Player {
                bar: Bar {
                    position_x: 110,
                    position_y: 32,
                    width: 4,
                    height: 15,
                },
                score: 0,
            },

            ball: Ball {
                position_x: 60,
                position_y: 32,
                diameter: 4,
                direction_x: 1,
                direction_y: 1,
                speed: 4,
                touches: 0,
            },
        }
    }
}

pub struct Bar {
    pub position_x: i32,
    pub position_y: i32,
    pub width: i32,
    pub height: i32,
}
