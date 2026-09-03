use gilrs::{Button, Event, EventType, GamepadId, Gilrs};
use std::{
  io::{self, Read, Write}, 
  time::Duration
};

fn no_op(_a: Option<GamepadId>) {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut gilrs = Gilrs::new().unwrap();

    let ports = serialport::available_ports()?;

    for port in ports {
        println!("port: {}", port.port_name)
    }

    let mut active_gamepad = None;

    //TODO pensar num jeito de achar o path automaticamente.
    let mut open_port = serialport::new("/dev/cu.usbmodem2101", 115200)
        .timeout(Duration::from_millis(100))
        .open()?;

    loop {
        // Examine new events
        while let Some(Event { id, event, .. }) = gilrs.next_event() {
          // println!("Enviei: {:?}", event);
            let command = match event {
                EventType::ButtonPressed(Button::South, _) => b'G', // A → verde
                EventType::ButtonPressed(Button::East, _) => b'R',  // B → vermelho
                EventType::ButtonPressed(Button::West, _) => b'B',  // X → azul
                EventType::ButtonPressed(Button::North, _) => b'Y', // Y → Amarelo
                EventType::ButtonPressed(Button::Start, _) => b'S', // S → Start
                _ => continue,
            };

            open_port.write_all(&[command])?;
            open_port.flush()?;

            println!("Enviei: {}", command as char);
            active_gamepad = Some(id);
        }

        let mut buffer = [0_u8; 256];

        match open_port.read(&mut buffer) {
            Ok(size) => {
                print!("{}", String::from_utf8_lossy(&buffer[..size]));
            }
            Err(error) if error.kind() == io::ErrorKind::TimedOut => {}
            Err(error) => return Err(error.into()),
        }

        no_op(active_gamepad)
    }
}
