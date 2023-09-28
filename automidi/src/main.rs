use midir::MidiInput;
use std::{sync::{Arc, Mutex}, thread, time::Duration, time::Instant};
use midly::{Smf, Track, live::LiveEvent, MidiMessage};

fn main() {
    let midi_input = MidiInput::new("MIDI Listener").unwrap();
    let port_name = "CASIO USB-MIDI:CASIO USB-MIDI MIDI 1 24:0";

    let buffer = Arc::new(Mutex::new(Vec::new()));
    let last_received = Arc::new(Mutex::new(Instant::now()));

    loop {
        let ports = midi_input.ports();
        match ports.iter().find(|&port| midi_input.port_name(&port).unwrap() == port_name) {
            Some(port) => {
                println!("{} found! Listening...", midi_input.port_name(port).unwrap());

                let buffer_clone = Arc::clone(&buffer);
                let last_received_clone = Arc::clone(&last_received);

                let _conn_in = midi_input.connect(port, "midir-read-input", move |_, message, _| {
                    let mut buffer = buffer_clone.lock().unwrap();
                    if let Ok(event) = LiveEvent::parse(message) {
                        if let LiveEvent::Midi { message, .. } = event {
                            buffer.push(message);
                        }
                    }
                    *last_received_clone.lock().unwrap() = Instant::now();
                }, ()).unwrap();

                loop {
                    thread::sleep(Duration::from_secs(1));

                    let elapsed = last_received.lock().unwrap().elapsed();
                    if elapsed >= Duration::from_secs(20) {
                        let mut buffer = buffer.lock().unwrap();
                        if !buffer.is_empty() {
                            let timestamp = Instant::now().duration_since(Instant::now() - elapsed).as_secs();
                            let filename = format!("{}.mid", timestamp);

                            let track = Track::new();
                            let smf = Smf {
                                header: midly::Header {
                                    format: midly::Format::SingleTrack,
                                    timing: midly::Timing::Metrical(96.into()),
                                },
                                tracks: vec![track],
                            };

                            smf.save(&filename).unwrap();
                            buffer.clear();
                        }
                    }
                }
            },
            None => {
                println!("{} not found. Retrying in 5 seconds...", port_name);
                thread::sleep(Duration::from_secs(5));
            }
        }
    }
}
