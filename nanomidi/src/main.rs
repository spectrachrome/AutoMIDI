use midir::{MidiInput, Ignore};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    loop {
        let midi_input = MidiInput::new("MIDI Listener").unwrap();
        let port_name = "CASIO USB-MIDI MIDI 1";

        match midi_input.ports().iter().find(|port| port.name().unwrap().contains(port_name)) {
            Some(port) => {
                println!("{} found! Listening...", port.name().unwrap());
                let in_port = midi_input.connect(port, "midir-read-input", |_, message, _| {
                    println!("{:?}", message);
                }, (), Ignore::None).unwrap();

                // Keep the program running to continue listening to MIDI messages
                loop {
                    sleep(Duration::from_secs(10));
                }
            },
            None => {
                println!("{} not found. Retrying in 10 seconds...", port_name);
                sleep(Duration::from_secs(10));
            }
        }
    }
}

