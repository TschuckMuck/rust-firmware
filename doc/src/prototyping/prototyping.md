# Prototyping

This section of the book describes the process of prototyping the YAK keyboard based on the existing
GH60.

## Prerequisites
### Documentation/Manuals/Datasheets
To get all relevant datasheets for the hardware used in this project you can execute the following
command(s) to retrieve them:

```shell
user@host $  cd yak/book/datasheets/
user@host $  ./download_datasheets.sh
```

### Tooling / Software
For detailed instructions on how to install the tools mentioned below check your operating
system-specific instructions (Windows, Linux, macOS).

#### C (ARM Toolchain)
[Download Page](https://developer.arm.com/open-source/gnu-toolchain/gnu-rm/downloads)
#### Rust Toolchain
[Download Page](https://rustup.rs/)
#### SEGGER Debugging tools
[Download Page](https://www.segger.com/downloads/jlink/#J-LinkSoftwareAndDocumentationPack)
#### Nordic nrf5x command line tools
[Download Page](https://infocenter.nordicsemi.com/topic/ug_nrf5x_cltools/UG/cltools/nrf5x_command_line_tools_lpage.html)
#### OpenOCD
[Project Homepage](http://openocd.org/)


### Hardware

* nRF52840 Dev Board
* GH60 PCB
* USB to UART Adapter

#### nRF52840 Dev Board
The nRF5280 is a development board for the nordic nrf52840 mcu which also already includes a (segger) debugger on the dev board itself.
One place you can order the nRF52840 Development Kit is [here](https://www.rutronik24.com/product/nordic/nrf52840-dk/10422794.html)

#### GH60 PCB
Due to the fact that the YAK keyboard will be built based on the well-known GH60 it is recommended to get one to be able to create a breakout board
which can be used for HW testing until the first YAK PCBs will be available.
One place you can order the GH60 PCB is [here](https://www.banggood.com/GH60-DIY-Mechanical-Keyboard-PCB-Support-Breathing-LED-60-Cherry-MX-Poker2-Poker3-p-1084998.html?cur_warehouse=CN)

#### USB to UART Adapter
In order to communicate with the UART on the development board an adapter for the "PC" is needed.
One place you can order such an adapter is [here](https://www.amazon.de/dp/B0753H4SQS/ref=cm_sw_em_r_mt_dp_U_uEdSCb45T73B2?th=1)
