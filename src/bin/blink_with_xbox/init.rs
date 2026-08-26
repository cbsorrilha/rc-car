use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::time::{Duration, Instant};
use esp_hal::usb_serial_jtag::UsbSerialJtag;
use esp_hal::peripherals::Peripherals;

fn blink_once(led: &mut Output<'_>) {
    led.set_high();
    let delay_start = Instant::now();
    while delay_start.elapsed() < Duration::from_millis(10){}

    led.set_low();
    let delay_start = Instant::now();
    while delay_start.elapsed() < Duration::from_millis(10){}
}


pub fn init(peripherals: Peripherals) -> ! {
  let mut usb_serial = UsbSerialJtag::new(peripherals.USB_DEVICE);

  let output_config = OutputConfig::default();
  let mut blue_led = Output::new(peripherals.GPIO5, Level::Low, output_config);
  let mut green_led = Output::new(peripherals.GPIO6, Level::Low, output_config);
  let mut red_led = Output::new(peripherals.GPIO7, Level::Low, output_config);

  // boot signature
  blink_once(&mut green_led);
  blink_once(&mut red_led);
  blink_once(&mut blue_led);


  loop {
    if let Ok(command) = usb_serial.read_byte() {
        match command {
            b'G' => {
                blink_once(&mut green_led);
            }
            b'R' => {
                blink_once(&mut red_led);
            }
            b'B' => {
                blink_once(&mut blue_led);
            }
            _ => {}
        }
    }
  }
}