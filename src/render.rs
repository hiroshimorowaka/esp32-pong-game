use alloc::format;
use alloc::sync::Arc;
use core::ops::DerefMut;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embedded_graphics::geometry::{Point, Size};
use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::primitives::{Circle, Primitive, PrimitiveStyleBuilder, Rectangle};
use embedded_graphics::text::{Baseline, Text};
use embedded_graphics::Drawable;
use esp_hal::i2c::I2C;
use esp_hal::peripherals::I2C0;
use ssd1306::mode::{BufferedGraphicsMode, DisplayConfig};
use ssd1306::prelude::I2CInterface;
use ssd1306::size::DisplaySize128x64;
use ssd1306::Ssd1306;

use crate::GameState;

pub type DisplayController = Arc<
    Mutex<
        CriticalSectionRawMutex,
        Ssd1306<
            I2CInterface<I2C<'static, I2C0>>,
            DisplaySize128x64,
            BufferedGraphicsMode<DisplaySize128x64>,
        >,
    >,
>;

pub async fn render_game(
    display: DisplayController,
    game_state: &'static Arc<Mutex<CriticalSectionRawMutex, GameState>>,
) {
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    let bar_style = PrimitiveStyleBuilder::new()
        .fill_color(BinaryColor::On)
        .stroke_width(1)
        .stroke_color(BinaryColor::On)
        .build();

    let ball_style = PrimitiveStyleBuilder::new()
        .fill_color(BinaryColor::On)
        .stroke_width(1)
        .stroke_color(BinaryColor::On)
        .build();

    let mut dsp = display.lock().await;
    let game_state = game_state.lock().await;

    dsp.init().unwrap();

    //Scores
    Text::with_baseline(
        format!("{}", game_state.player_1.score).as_str(),
        Point::new(10, 0),
        text_style,
        Baseline::Top,
    )
    .draw(dsp.deref_mut())
    .unwrap();

    Text::with_baseline(
        format!("{}", game_state.player_2.score).as_str(),
        Point::new(110, 0),
        text_style,
        Baseline::Top,
    )
    .draw(dsp.deref_mut())
    .unwrap();

    //Ball
    Circle::new(
        Point::new(game_state.ball.position_x, game_state.ball.position_y),
        game_state.ball.diameter as u32,
    )
    .into_styled(ball_style)
    .draw(dsp.deref_mut())
    .unwrap();

    //Player 1 bar
    Rectangle::new(
        Point::new(
            game_state.player_1.bar.position_x,
            game_state.player_1.bar.position_y,
        ),
        Size::new(
            game_state.player_1.bar.width as u32,
            game_state.player_1.bar.height as u32,
        ),
    )
    .into_styled(bar_style)
    .draw(dsp.deref_mut())
    .unwrap();

    //Player 2 bar
    Rectangle::new(
        Point::new(
            game_state.player_2.bar.position_x,
            game_state.player_2.bar.position_y,
        ),
        Size::new(
            game_state.player_2.bar.width as u32,
            game_state.player_2.bar.height as u32,
        ),
    )
    .into_styled(bar_style)
    .draw(dsp.deref_mut())
    .unwrap();

    dsp.flush().unwrap();
}
