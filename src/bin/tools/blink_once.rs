use super::wait;
use esp_hal::gpio::{Output};


pub fn blink_once(led: &mut Output<'_>) {
    led.set_high();
    wait(100);

    led.set_low();
    wait(100);
}