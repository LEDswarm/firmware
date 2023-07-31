use smart_leds_trait::{SmartLedsWrite, White};
use ws2812_esp32_rmt_driver::driver::color::LedPixelColorGrbw32;
use ws2812_esp32_rmt_driver::{LedPixelEsp32Rmt, RGBW8};

pub struct LedConfig {
    pub pin: u32,
}

/// Driver for the NeoPixel Jewel, a small two-inch circular PCB with seven SK6812 LEDs.
///
/// This struct abstracts the interface to the `ws2812_esp32_rmt_driver` to provide a method API
pub struct Led {
    driver: LedPixelEsp32Rmt::<RGBW8, LedPixelColorGrbw32>,
}

impl Led {
    pub fn new(config: LedConfig) -> Self {
        Self {
            driver: LedPixelEsp32Rmt::<RGBW8, LedPixelColorGrbw32>::new(0, config.pin).unwrap(),
        }
    }

    /// Write an RGBW color to our NeoPixel Jewel.
    pub fn set_rgbw(
        &mut self,
        red: u8,
        green: u8,
        blue: u8,
        white: u8
    ) {
        let pixels = std::iter::repeat(RGBW8::from((
            red,
            green,
            blue,
            White(white),
        ))).take(25);

        self.driver.write(pixels).unwrap();
    }
}