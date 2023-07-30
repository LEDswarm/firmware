/*
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Hello, world!");
}
*/

use smart_leds_trait::{SmartLedsWrite, White};
use std::thread::sleep;
use std::time::Duration;
use ws2812_esp32_rmt_driver::driver::color::LedPixelColorGrbw32;
use ws2812_esp32_rmt_driver::{LedPixelEsp32Rmt, RGBW8};

fn main() -> ! {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    let led_pin = 20;
    let mut ws2812 = LedPixelEsp32Rmt::<RGBW8, LedPixelColorGrbw32>::new(0, led_pin).unwrap();

    loop {
        let pixels = std::iter::repeat(RGBW8::from((6, 0, 0, White(0)))).take(25);
        ws2812.write(pixels).unwrap();
        sleep(Duration::from_millis(1000));

        let pixels = std::iter::repeat(RGBW8::from((0, 6, 0, White(0)))).take(25);
        ws2812.write(pixels).unwrap();
        sleep(Duration::from_millis(1000));

        let pixels = std::iter::repeat(RGBW8::from((0, 0, 6, White(0)))).take(25);
        ws2812.write(pixels).unwrap();
        sleep(Duration::from_millis(1000));

        let pixels = std::iter::repeat(RGBW8::from((0, 0, 0, White(6)))).take(25);
        ws2812.write(pixels).unwrap();
        sleep(Duration::from_millis(1000));
    }
}
