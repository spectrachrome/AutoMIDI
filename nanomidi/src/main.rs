use midir::{MidiInput, Ignore};
use std::{thread, time};

fn main() {
    let mut midi_in = MidiInput::new("midir reading input").unwrap();
    midi_in.ignore(Ignore::None);

    loop {
        let in_ports = midi_in.ports();
        println!("Available ports:");
        for port in &in_ports {
            println!("{:?}", midi_in.port_name(port).unwrap());
        }

        if let Some(in_port) = in_ports.iter().find(|p| midi_in.port_name(p).unwrap() == "CASIO USB-MIDI:CASIO USB-MIDI MIDI 1 24:0") {
            println!("Found port: {:?}", midi_in.port_name(in_port).unwrap());

            let _conn_in = midi_in.connect(in_port, "midir-read-input", move |_, message, _| {
                println!("{:?}", message);
            }, ()).unwrap();

            // Prevent exiting immediately
            thread::sleep(time::Duration::from_millis(10000));
            break;
        } else {
            println!("Port not found, retrying in 5 seconds...");
            thread::sleep(time::Duration::from_secs(5));
        }
    }
}
