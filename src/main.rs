//! This example test the RP Pico on board LED.
//!
//! It does not work with the RP Pico W board.

#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut led = Output::new(p.PIN_25, Level::Low);
    let delay: Duration = Duration::from_secs(1);

    loop {
        info!("led on!");
        led.set_high();
        Timer::after(delay).await;

        info!("led off!");
        led.set_low();

        Timer::after(delay).await;
    }
}
