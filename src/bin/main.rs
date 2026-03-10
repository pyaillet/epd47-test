#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use alloc::boxed::Box;
use embedded_graphics::prelude::RgbColor;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::main;
use lilygo_epd47::display::DisplayRotation;
use lilygo_epd47::{pin_config, Display, DrawMode};
use mousefood::prelude::Rgb888;
use mousefood::{ColorTheme, EmbeddedBackend, EmbeddedBackendConfig};
use ratatui::Terminal;

extern crate alloc;

use epd47_test::app::App;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // generator version: 1.0.0

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::_240MHz);
    let peripherals = esp_hal::init(config);

    esp_alloc::psram_allocator!(peripherals.PSRAM, esp_hal::psram);

    let mut display = Display::new(
        pin_config!(peripherals),
        peripherals.DMA_CH0,
        peripherals.LCD_CAM,
        peripherals.RMT,
    )
    .expect("to initialize correctly");

    display.set_rotation(DisplayRotation::Rotate90);

    let delay = Delay::new();
    display.power_on();
    delay.delay_millis(10);
    display.clear().expect("to clear display");

    let theme = ColorTheme {
        background: Rgb888::WHITE,
        foreground: Rgb888::BLACK,
        ..ColorTheme::ansi()
    };

    // setup mousefood
    let backend = EmbeddedBackendConfig {
        color_theme: theme,
        font_regular: mousefood::fonts::mono_10x20_atlas(),
        flush_callback: Box::new(move |display: &mut Display| {
            // display.clear().expect("to clear display");
            display
                .flush(DrawMode::BlackOnWhite)
                .expect("to flush to the display");
        }),
        ..Default::default()
    };

    let backend = EmbeddedBackend::new(&mut display, backend);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut app = App::new();
    app.run(&mut terminal, delay);

    loop {}
}
