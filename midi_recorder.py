import mido
import time
from mido import MidiFile, MidiTrack, Message

def get_port_by_name(port_name):
    for port in mido.get_input_names():
        if port_name in port:
            return port
    return None

def main():
    target_port_name = "1: CASIO USB-MIDI:CASIO USB-MIDI MIDI 1 24:0"
    
    while True:
        port_name = get_port_by_name(target_port_name)
        
        if port_name:
            print(f"{port_name} found! Listening...")
            with mido.open_input(port_name) as inport:
                for msg in inport:
                    if msg.type == 'note_on' or msg.type == 'note_off':
                        print(msg)
        else:
            print(f"{target_port_name} not found. Retrying in 10 seconds...")
            time.sleep(10)

if __name__ == "__main__":
    main()
