
use std::thread::sleep;
use std::time::Duration;

use smart_leds_trait::{SmartLedsWrite, White};
use ws2812_esp32_rmt_driver::driver::color::LedPixelColorGrbw32;
use ws2812_esp32_rmt_driver::{LedPixelEsp32Rmt, RGBW8};

use esp_idf_hal::i2c::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;

use adxl343::Adxl343;
use adxl343::accelerometer::Accelerometer;

fn round(x: f32, decimals: u32) -> f32 {
    let y = 10i32.pow(decimals) as f32;
    (x * y).round() / y
}

fn main() -> ! {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let i2c = peripherals.i2c0;
    let sda = peripherals.pins.gpio6;
    let scl = peripherals.pins.gpio7;

    println!("Starting I2C SSD1306 test");

    let config = I2cConfig::new().baudrate(100.kHz().into());
    let i2c    = I2cDriver::new(i2c, sda, scl, &config).unwrap();

    let mut accelerometer = Adxl343::new(i2c).unwrap();

    // LEDs

    let led_pin = 20;
    let mut ws2812 = LedPixelEsp32Rmt::<RGBW8, LedPixelColorGrbw32>::new(0, led_pin).unwrap();

    let mut average = vec![0.0; 5];
    let mut index = 0;
    let mut current_magnitude;
    let mut previous_magnitude = 0.0;

    let mut stay_red = false;

    loop {
        // Read acceleration
        let accel = accelerometer.accel_norm().unwrap();

        current_magnitude = (accel.x.powf(2.0) + accel.y.powf(2.0) + accel.z.powf(2.0)).sqrt();
        average[index] = (current_magnitude - previous_magnitude).abs();

        previous_magnitude = current_magnitude;

        let delta = average.iter().sum::<f32>() / average.len() as f32;
        let rounded_delta = round(delta, 1);
        let threshold = 0.6;
        let brightness = 5;

        //println!("{}", rounded_delta);

        let red = brightness - (brightness as f32 * (1.0 - rounded_delta / threshold)) as u8;
        let green = (brightness as f32 * (1.0 - rounded_delta / threshold)) as u8;

        //println!("red: {}", red);
        //println!("green: {}", green);

        //println!("Hue: {}", hue);
        if stay_red {
            let pixels = std::iter::repeat(RGBW8::from((brightness, 0, 0, White(0)))).take(25);
            ws2812.write(pixels).unwrap();
        } else {
            let pixels = std::iter::repeat(RGBW8::from((red, green, 0, White(0)))).take(25);
            ws2812.write(pixels).unwrap();
        }
        //let pixels = std::iter::repeat(RGBW8::from((0, 10, 0, White(0)))).take(25);

        if red > brightness - 2 {
            stay_red = true;
        }

        if index < average.len() - 1 {
            index += 1;
        } else {
            index = 0;
        }

        sleep(Duration::from_millis(40));
        /*
        let pixels = std::iter::repeat(RGBW8::from((0, 6, 0, White(0)))).take(25);
        ws2812.write(pixels).unwrap();
        sleep(Duration::from_millis(1000));

        let pixels = std::iter::repeat(RGBW8::from((0, 0, 6, White(0)))).take(25);
        ws2812.write(pixels).unwrap();
        sleep(Duration::from_millis(1000));

        let pixels = std::iter::repeat(RGBW8::from((0, 0, 0, White(6)))).take(25);
        ws2812.write(pixels).unwrap();
        sleep(Duration::from_millis(1000));
        */
    }
}

/*

//! I2C test with SSD1306
//!
//! Folowing pins are used:
//! SDA     GPIO5
//! SCL     GPIO6
//!
//! Depending on your target and the board you are using you have to change the pins.
//!
//! For this example you need to hook up an SSD1306 I2C display.
//! The display will flash black and white.

use esp_idf_hal::delay::{FreeRtos, BLOCK};
use esp_idf_hal::i2c::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;

use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};

use adxl343::Adxl343;

const SSD1306_ADDRESS: u8 = 0x3c;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let i2c = peripherals.i2c0;
    let sda = peripherals.pins.gpio6;
    let scl = peripherals.pins.gpio7;

    println!("Starting I2C SSD1306 test");

    let config = I2cConfig::new().baudrate(100.kHz().into());
    let mut i2c = I2cDriver::new(i2c, sda, scl, &config)?;

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(
        interface,
        DisplaySize128x64,
        DisplayRotation::Rotate0,
    ).into_buffered_graphics_mode();
    display.init().unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(BinaryColor::On)
        .build();

    /*Text::with_baseline("Hello world!", Point::zero(), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();*/

    Text::with_baseline("9", Point::new(0, 16), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    loop {
        // we are sleeping here to make sure the watchdog isn't triggered
        FreeRtos::delay_ms(500);
        display.flush().unwrap();
        FreeRtos::delay_ms(500);
        display.flush().unwrap();
    }
}

*/
