use midir::{MidiInput, Ignore};

fn main() {
    let midi_input = MidiInput::new("MIDI Listener").unwrap();
    let port_name = "CASIO USB-MIDI MIDI 1";

    loop {
        let ports = midi_input.ports();
        
        match ports.iter().find(|&&port| midi_input.port_name(&port).unwrap().contains(port_name)) {
            Some(&port) => {
                println!("{} found! Listening...", midi_input.port_name(&port).unwrap());

                let in_port = midi_input.connect(&port, "midir-read-input", move |_, message, _| {
                    println!("{:?}", message);
                }, ()).unwrap();

                // Keep the program running to continue listening to MIDI messages
                loop {
                    std::thread::sleep(std::time::Duration::from_secs(10));
                }
            },
            None => {
                println!("{} not found. Retrying in 10 seconds...", port_name);
                std::thread::sleep(std::time::Duration::from_secs(10));
            }
        }
    }
}
