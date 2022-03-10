use core::iter;
use smart_leds::{hsv::hsv2rgb, hsv::Hsv, SmartLedsWrite, RGB8};

pub struct LEDs<T> {
    leds: T,
    num: usize,
}

impl<T> LEDs<T>
where
    T: SmartLedsWrite<Color = RGB8>,
{
    pub fn new(leds: T, num: usize) -> Self {
        Self { leds, num }
    }

    pub fn set_hsv(&mut self, color: Hsv) -> Result<(), <T as SmartLedsWrite>::Error> {
        self.set_rgb(hsv2rgb(color))
    }

    pub fn set_rgb(&mut self, color: RGB8) -> Result<(), <T as SmartLedsWrite>::Error> {
        self.set_colors(iter::repeat(color).take(self.num))
    }

    pub fn set_colors<I>(&mut self, colors: I) -> Result<(), <T as SmartLedsWrite>::Error>
    where
        I: Iterator<Item = RGB8>,
    {
        let gamma_corrected = smart_leds::gamma(colors);
        self.leds.write(gamma_corrected)
    }
}
