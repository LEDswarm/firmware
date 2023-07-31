
use std::thread::sleep;
use std::time::Duration;

use esp_idf_hal::i2c::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;

pub mod led;
pub mod moving_average;

use led::{Led, LedConfig};
use moving_average::MovingAverage;

use adxl343::Adxl343;
use adxl343::accelerometer::Accelerometer;

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
    let mut moving_avg    = MovingAverage::new();

    // LEDs

    // let led_pin = 20;
    // let mut ws2812 = LedPixelEsp32Rmt::<RGBW8, LedPixelColorGrbw32>::new(0, led_pin).unwrap();

    let mut led = Led::new(LedConfig {
        pin: 20,
    });

    let mut stay_red = false;

    loop {
        // Read acceleration
        let accel = accelerometer.accel_norm().unwrap();

        moving_avg.add(accel);
        let delta = moving_avg.get_average_delta();
        println!("{}", delta);
        let threshold = 0.6;
        let brightness = 5;

        let red = brightness - (brightness as f32 * (1.0 - delta / threshold)) as u8;
        let green = (brightness as f32 * (1.0 - delta / threshold)) as u8;

        //println!("red: {}", red);
        //println!("green: {}", green);

        //println!("Hue: {}", hue);
        if stay_red {
            led.set_rgbw(brightness, 0, 0, 0);
        } else {
            led.set_rgbw(red, green, 0, 0);
        }

        if red > brightness - 2 {
            stay_red = true;
        }

        sleep(Duration::from_millis(30));
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
