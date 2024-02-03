# LED Controller

This application changes the LED pattern found in the T9 and S1 mini-pc series.

The idea to write this tool came from a Reddit thread [1] after googling the very same issue.

If you want to have some additional background, have a look at [deep-dive.md](./deep-dive.md)

## Build the Project

By default this project builds for the target `x86_64-unknown-linux-musl` on Linux to create a static binary that is portable across multiple versions and Linux distributions.
Please make sure that you are able to build the target or have provided an alternative one.

## Usage

The usage of this application is as follows (copied from the help command):

```
Control the LED ring found in the T9 or S1 mini-pc series.

Usage: s1t-ledcontrol [OPTIONS]

Options:
  -d, --device <DEVICE>          The device the data is written to [default: /dev/ttyUSB0]
  -m, --mode <MODE>              Set the LED mode [default: off] [possible values: off, auto, rainbow, breathing, colour-cycle]
  -b, --brightness <BRIGHTNESS>  Set the brightness of the LEDs between 1 and 5 [default: 1]
  -s, --speed <SPEED>            Set the speed of the animations between 1 and 5 [default: 1]
  -r, --baud-rate <BAUD_RATE>    Set the baud rate [default: 9600]
  -h, --help                     Print help
  -V, --version                  Print version
```

The baud rate is the one officially supported by the USB-to-UART chip (CH340N).
In the official driver the baud rate is 10000, which is not supported by the CH340N.

[1] https://old.reddit.com/r/MiniPCs/comments/18icusg/t9_plus_n100_how_to_control_led/