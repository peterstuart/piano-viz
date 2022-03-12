#![no_main]
#![no_std]

use piano_viz as _; // global logger + panicking-behavior + memory layout
use piano_viz::LEDs;
use smart_leds::hsv::Hsv;
use stm32f4xx_hal::{pac, prelude::*, rcc::SYSCLK_MAX, time::Hertz};
use ws2812_timer_delay::Ws2812;

const NUM_LEDS: usize = 144;

const YELLOW: Hsv = Hsv {
    hue: 31,
    sat: 255,
    val: 150,
};

#[cortex_m_rt::entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Constrain clocking registers
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(Hertz::Hz(SYSCLK_MAX)).freeze();

        let mut timer = cp.SYST.counter_hz(&clocks);
        timer.start(Hertz::MHz(3)).unwrap();

        let gpioa = dp.GPIOA.split();

        let ws2812 = Ws2812::new(timer, gpioa.pa7.into_push_pull_output());

        let mut leds = LEDs::new(ws2812, NUM_LEDS);
        leds.set_hsv(YELLOW).unwrap();
    }

    loop {
        continue;
    }
}
