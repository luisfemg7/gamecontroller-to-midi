# gamecontroller-to-midi
A program using SDL2 and midir to map a game controller input into MIDI messages

## TO-DO
### MVP
- Create a virtual MIDI port
    - Test with DAW
- Handle incoming and outgoing MIDI messages
- Handle mapping for different controller-based input into different MIDI messages (Note On/Off, Control Change, Pitch Bend)
    - Start off with Note On/Off messages

### Wish list
- Map controller gyroscope (testing with Dualshock 4)
- Create a UI to remap controller input to different MIDI messages
