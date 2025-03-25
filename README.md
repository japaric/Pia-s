# Pia's

![Web application showing a musical keyboard with some notes highlighted](./images/screenshot.png)

`Pia's` stands for "Pia's Interval Analysis Screen". `Pia's` is a MIDI analysis web application. 

In Japanese, `Pia's` can be written as ぴあの which is pronounced "piano" (🎹).

`Pia's` is also my first try at writting a small WASM web application.

## Features

- Full size (88-key) keyboard visualization.
- Listens to a single MIDI input device, like a MIDI keyboard.
- Understands common musical scales. Notes outside the scale are highlighted differently. A text overlay indicates the scale degree of each pressed note.
- Understands the use of the sustain pedal. Notes sustained by the pedal are colored differently than held notes.
- Identifies common chords and their extensions.

**NOTE** `Pia's` does not produce audio nor MIDI output. Do not expect sound to come from it..

## Build dependencies

See [`shell.nix`](./shell.nix).

## Usage

0. Connect a MIDI input device to your computer. Note that you can also connect a virtual MIDI loopback device, like 'MIDI Through' on Linux, to the web application and drive it entirely by software, e.g. a DAW.

1. Build the web application and start a web server that listens on port 8000.

``` console
$ just serve
```


2. Browse to `http://localhost:8000`.

3. Grant the web application access to your MIDI devices.

4. Select your MIDI input device from the drop-down menu in the web application.

5. (Optionally) change the Scale using the drop down menus.

6. Input MIDI using your MIDI input device.

**NOTE** Some DAWs take exclusive control of input devices. If you want to use this web application alongside a DAW, you'll need to set up a virtual MIDI loopback device and do some additional routing to duplicate the MIDI input onto a second MIDI device; then each application needs to connect to a different MIDI device. Exact instructions on how to do that are out of scope for this document.
