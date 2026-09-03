#![no_std]
use esp_hal::gpio::Output;
use esp_hal::time::{Duration, Instant};

pub fn wait(duration: u64) {
    let delay_start = Instant::now();
    while delay_start.elapsed() < Duration::from_millis(duration) {}
}

pub fn blink_once(led: &mut Output<'_>) {
    led.set_high();
    wait(100);

    led.set_low();
    wait(100);
}

pub fn blink_once_for(led: &mut Output<'_>, high_duration: u64, low_duration: u64) {
    led.set_high();
    wait(high_duration);

    led.set_low();
    wait(low_duration);
}
