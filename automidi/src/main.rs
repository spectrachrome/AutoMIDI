use midir::MidiInput;

pub mod midi_recorder;

fn main() {
    let port_name = "CASIO USB-MIDI:CASIO USB-MIDI MIDI 1 24:0";
    let recorder = midi_recorder::MidiRecorder::new(port_name);
    recorder.start_recording(MidiInput::new("AutoMIDI Listener").unwrap());
}