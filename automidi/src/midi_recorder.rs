use midir::MidiInput;
use std::{sync::{Arc, Mutex}, thread, time::Duration, time::Instant};
use midly::{Smf, Track, live::LiveEvent, MidiMessage};

pub struct MidiRecorder {
    port_name: String,
    buffer: Arc<Mutex<Vec<MidiMessage>>>,
    last_received: Arc<Mutex<Instant>>,
}

impl MidiRecorder {
    pub fn new(port_name: &str) -> Self {
        MidiRecorder {
            port_name: port_name.to_string(),
            buffer: Arc::new(Mutex::new(Vec::new())),
            last_received: Arc::new(Mutex::new(Instant::now())),
        }
    }

    pub fn start_recording(&self, midi_input: MidiInput) {
        loop {
            let ports = midi_input.ports();
            match ports.iter().find(|&port| midi_input.port_name(&port).unwrap() == self.port_name) {
                Some(port) => {
                    println!("{} found! Listening...", midi_input.port_name(port).unwrap());

                    let buffer_clone = Arc::clone(&self.buffer);
                    let last_received_clone = Arc::clone(&self.last_received);

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

                        let elapsed = self.last_received.lock().unwrap().elapsed();
                        if elapsed >= Duration::from_secs(20) {
                            let mut buffer = self.buffer.lock().unwrap();
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
                    println!("{} not found. Retrying in 5 seconds...", self.port_name);
                    thread::sleep(Duration::from_secs(5));
                }
            }
        }
    }
}
