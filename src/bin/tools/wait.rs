use esp_hal::time::{Duration, Instant};

pub fn wait(duration: u64) {
  let delay_start = Instant::now();
  while delay_start.elapsed() < Duration::from_millis(duration){}
}
