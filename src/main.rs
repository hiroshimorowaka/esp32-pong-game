#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

mod render;

use gamestate::GameState;
use render::DisplayController;

mod ball;
mod gamestate;
mod players;

use alloc::boxed::Box;
use alloc::sync::Arc;
use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use embassy_time::{Duration, Timer};

use esp_backtrace as _;
use esp_hal::i2c::I2C;
use esp_hal::macros::main;
use esp_hal::system::SystemExt;
use esp_hal::timer::TimerGroup;
use esp_hal::IO;
use esp_hal::{
    clock::ClockControl, embassy, entry, peripherals::Peripherals, prelude::_fugit_RateExtU32,
};

use ssd1306::{rotation::DisplayRotation, size::DisplaySize128x64, I2CDisplayInterface, Ssd1306};
extern crate alloc;

use core::mem::MaybeUninit;

#[global_allocator]
static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

fn init_heap() {
    const HEAP_SIZE: usize = 32 * 1024;
    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

    unsafe {
        ALLOCATOR.init(HEAP.as_mut_ptr() as *mut u8, HEAP_SIZE);
    }
}

#[main]
async fn main(_spawner: Spawner) {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();

    init_heap();

    esp_println::logger::init_logger_from_env();

    let timg0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    embassy::init(&clocks, timg0);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let i2c = I2C::new(
        peripherals.I2C0,
        io.pins.gpio21,
        io.pins.gpio22,
        100.kHz(),
        &clocks,
    );

    let interface = I2CDisplayInterface::new(i2c);

    let display: DisplayController = Arc::new(Mutex::new(
        Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode(),
    ));

    let game_state: &'static mut Arc<Mutex<CriticalSectionRawMutex, GameState>> =
        Box::leak(Box::new(Arc::new(Mutex::new(GameState::default()))));

    let p1_up_button = Arc::new(io.pins.gpio14.into_pull_down_input());
    let p1_down_button = Arc::new(io.pins.gpio12.into_pull_down_input());

    let p2_up_button = Arc::new(io.pins.gpio32.into_pull_down_input());
    let p2_down_button = Arc::new(io.pins.gpio33.into_pull_down_input());

    loop {
        let display = Arc::clone(&display);

        ball::calcule_ball_position(game_state).await;
        players::calcule_scores(game_state).await;

        players::move_player(
            game_state,
            Arc::clone(&p1_up_button),
            Arc::clone(&p1_down_button),
            Arc::clone(&p2_up_button),
            Arc::clone(&p2_down_button),
        )
        .await;

        render::render_game(display, game_state).await;
        Timer::after(Duration::from_millis(34)).await;
    }
}
