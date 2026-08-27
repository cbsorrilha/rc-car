#![no_std]
use esp_hal::time::{Duration, Instant};
use esp_hal::gpio::{Output};

pub fn wait(duration: u64) {
  let delay_start = Instant::now();
  while delay_start.elapsed() < Duration::from_millis(duration){}
}

pub fn blink_once(led: &mut Output<'_>) {
    led.set_high();
    wait(100);

    led.set_low();
    wait(100);
}