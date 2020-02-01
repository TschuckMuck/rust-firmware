# The YAK Keyboard
The YAK keyboard is a NRF52840 based open source 60% DYI keyboard with rust firmware.

## NRF52840 Overview
* ARM CortextM4
* 256 KB RAM
* 1 MB Flash
* Bluetooth
* USB

[more details](https://www.nordicsemi.com/Products/Low-power-short-range-wireless/nRF52840)

## Idea Incubator
### Hardware

#### Mark 0 (Prototype)
* NRF52840 MDK 
* Breakout Board

#### Mark 1
* Reworked GH60 PCB with NRF52840
* SWD Debug Connector
* UART PIN Connector
* USB-C Connector
* RGB LED's

### Software
#### Keyboard Features
* Serial Console (UART) Debug Output
* Serial Console input/output
* HID (USB - Keyboard)
* Backlight LEDs
* Backlight LEDs RGB
* Framebuffer based LED control
* CLI
* Rest Interface via Virtual Serial (USB)
* Bootloader
* Flash File System Flash driver -> store persistent config
* Layering

## Other OpenSource/DIY Keyboard Projects
* [GH60](http://blog.komar.be/projects/gh60-programmable-keyboard/)
* [Dactyl-Keyboard](https://github.com/adereth/dactyl-keyboard)
* [Dactyl-ManuForm-Keyboard](https://github.com/abstracthat/dactyl-manuform)
* [Colosseum](https://github.com/swanmatch/colosseum60)
* [Lets-Split-Keyboard](https://github.com/nicinabox/lets-split-guide)

