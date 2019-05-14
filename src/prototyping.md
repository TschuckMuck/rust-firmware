# Prototyping
This section of the book describes the process of prototyping the new yak keyboard based on the existing GH60.


## Prerequisite

### Documentation/Manuals/Datasheets
To get all relevant datasheets you might need to consult use the following command(s) to retrieve them:

```shell
user@host $  cd yak/book/datasheets/
user@host $  ./download_datasheets.sh
```
### Software
 * Rust toolchain
 * C toolchain
 * Segger debugging tools


#### MacOsx
##### Rust (ARM Toolchain)
* rust >= 1.34 (arm libs ->)

##### C (ARM Toolchain)
```shell
user@host ~$ brew install armmbed/formulae/arm-none-eabi-gcc
```

##### Segger Debugging tools
```shell
user@host ~$ brew cask install homebrew/cask-drivers/segger-jlink
user@host ~$ brew cask install segger-jlink
```

### Hardware

* nRF52840 Dev Board
* GH60 PCB
* USB to UART Adapter

#### nRF52840 Dev Board
The nRF5280 is a development board for the nordic nrf52840 mcu which also already includes a (segger) debugger on the dev board itself.
You can order the nRF52840 Development Kit for example [here](https://www.rutronik24.com/product/nordic/nrf52840-dk/10422794.html)

#### GH60 PCB
Due to the fact that the YAK keyboard will be built based on the well know GH60 it is recommended to get one to be able create a break-out-board
which can be used for HW testing until the first YAK PCB's will be available.
The GH60 PCB can be ordered for example [here](https://www.banggood.com/GH60-DIY-Mechanical-Keyboard-PCB-Support-Breathing-LED-60-Cherry-MX-Poker2-Poker3-p-1084998.html?cur_warehouse=CN)

#### USB to UART Adapter
In order to communicate with the uart on the dev board a adapter for the "PC" is needed you can order such an adapter for example [here](https://www.amazon.de/dp/B0753H4SQS/ref=cm_sw_em_r_mt_dp_U_uEdSCb45T73B2?th=1)


