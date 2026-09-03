use esp_hal::{
    gpio::{Level, Output, OutputConfig},
    peripherals::Peripherals,
    usb_serial_jtag::UsbSerialJtag,
};

use genius_core::{Color, Event, SEQ_MAX_LENGTH, State, Status, state_machine};
use rc_car::{blink_once, blink_once_for, wait};

struct ColorLedMap<'leds, 'pins> {
    blue_led: &'leds mut Output<'pins>,
    green_led: &'leds mut Output<'pins>,
    red_led: &'leds mut Output<'pins>,
    yellow_led: &'leds mut Output<'pins>,
}

fn map_color_to_led<'call, 'leds, 'pins>(
    color: Color,
    map: &'call mut ColorLedMap<'leds, 'pins>,
) -> &'call mut Output<'pins> {
    match color {
        Color::Blue => map.blue_led,
        Color::Red => map.red_led,
        Color::Green => map.green_led,
        Color::Yellow => map.yellow_led,
    }
}

pub fn init(peripherals: Peripherals) -> ! {
    let mut usb_serial = UsbSerialJtag::new(peripherals.USB_DEVICE);

    let output_config = OutputConfig::default();
    let mut blue_led = Output::new(peripherals.GPIO5, Level::Low, output_config);
    let mut green_led = Output::new(peripherals.GPIO6, Level::Low, output_config);
    let mut red_led = Output::new(peripherals.GPIO7, Level::Low, output_config);
    let mut yellow_led = Output::new(peripherals.GPIO15, Level::Low, output_config);

    let mut incorrect_led = Output::new(peripherals.GPIO16, Level::Low, output_config);
    let mut correct_led = Output::new(peripherals.GPIO17, Level::Low, output_config);

    // boot signature TODO: abstrair a boot signature pra uma function
    esp_println::println!("Initializing Boot Signature!");
    blink_once(&mut green_led);
    blink_once(&mut red_led);
    blink_once(&mut blue_led);
    blink_once(&mut yellow_led);

    blink_once(&mut incorrect_led);
    blink_once(&mut correct_led);
    wait(500);
    blink_once(&mut incorrect_led);
    blink_once(&mut correct_led);

    let mut map = ColorLedMap {
        blue_led: &mut blue_led,
        green_led: &mut green_led,
        red_led: &mut red_led,
        yellow_led: &mut yellow_led,
    };

    loop {
        esp_println::println!("Initializing Game Loop!");
        let mut state = State::new();
        let end_sequence = SEQ_MAX_LENGTH;
        while state.current_sequence_size() < end_sequence {
            esp_println::println!(
                "Initializing Round number {}",
                state.current_sequence_size()
            );
            state = state_machine(state, Event::ColorGiven(Color::Blue));
            state = state_machine(state, Event::GameStarted);

            // Round Starting Signal
            blink_once_for(&mut correct_led, 500, 500);
            wait(500);
            blink_once_for(&mut correct_led, 500, 500);

            wait(1000);

            let mut color_counter = 0;
            while color_counter < state.current_sequence_size() {
                let color = state.sequence()[color_counter].unwrap();
                esp_println::println!("Blinking led {:?} as number {}!", color, color_counter);
                let led = map_color_to_led(color, &mut map);
                blink_once_for(led, 1000, 500);
                color_counter = color_counter + 1;
            }

            state = state_machine(state, Event::SequencePlaybackCompleted);

            while state.status() == &Status::AwaitingInput {
                if let Ok(command) = usb_serial.read_byte() {
                    match command {
                        b'G' => {
                            esp_println::println!("pressed green !");
                            blink_once(map.green_led);
                            state = state_machine(state, Event::ColorPressed(Color::Green))
                        }
                        b'R' => {
                            esp_println::println!("pressed red !");
                            blink_once(map.red_led);
                            state = state_machine(state, Event::ColorPressed(Color::Red))
                        }
                        b'B' => {
                            esp_println::println!("pressed blue !");
                            blink_once(map.blue_led);
                            state = state_machine(state, Event::ColorPressed(Color::Blue))
                        }
                        b'Y' => {
                            esp_println::println!("Receiving reset");
                            blink_once(map.yellow_led);
                            state = state_machine(state, Event::ColorPressed(Color::Yellow))
                        }
                        b'S' => {
                            esp_println::println!("pressed start !");
                            state = state_machine(state, Event::ResetRequested);
                            continue;
                        }
                        0_u8..=82_u8 | 84_u8..=u8::MAX => {}
                    }
                }
            }

            while state.status() == &Status::GameCompleted {
                esp_println::println!("You Won!");
                blink_once(&mut correct_led);
                if let Ok(command) = usb_serial.read_byte() {
                    match command {
                        b'S' => {
                            state = state_machine(state, Event::ResetRequested);
                        }
                        0_u8..=82_u8 | 84_u8..=u8::MAX => {}
                    }
                }
            }

            while state.status() == &Status::GameOver {
                esp_println::println!("GameOver!");
                blink_once(&mut incorrect_led);
                if let Ok(command) = usb_serial.read_byte() {
                    match command {
                        b'S' => {
                            esp_println::println!("Receiving reset");
                            state = state_machine(state, Event::ResetRequested);
                        }
                        0_u8..=82_u8 | 84_u8..=u8::MAX => {}
                    }
                }
            }

            if state.status() == &Status::RoundSuccess {
                esp_println::println!("Acertou!");
                blink_once(&mut correct_led);
                wait(500);
                blink_once(&mut correct_led);
                wait(500);
            }
        }
    }
}
