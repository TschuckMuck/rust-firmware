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
 * SEGGER debugging tools


#### Rust (ARM Toolchain)
* rustup
* rust >= 1.34 (arm libs ->)

##### All non-Windows platforms
```shell
curl https://sh.rustup.rs -sSf | sh
```


#### C (ARM Toolchain)
[Download Page](https://developer.arm.com/open-source/gnu-toolchain/gnu-rm/downloads)

##### MacOsx
```shell
user@host ~$ brew install armmbed/formulae/arm-none-eabi-gcc
```
##### Linux
```shell
sudo apt install gcc-arm-none-eabi
```


#### SEGGER Debugging tools
[Download Page](https://www.segger.com/downloads/jlink/#J-LinkSoftwareAndDocumentationPack)

##### MacOsx
```shell
user@host ~$ brew cask install homebrew/cask-drivers/segger-jlink
user@host ~$ brew cask install segger-jlink
```

##### Linux
Go to the download page, choose the appropriate JLink Software & Documentation package, then install
similar to this example:
```shell
sudo dpkg -i ~/Downloads/JLink_Linux_V644i_x86_64.deb
```


#### Nordic nRF5x command line tools
[Download Page](https://www.nordicsemi.com/Software-and-Tools/Development-Tools/nRF5-Command-Line-Tools/Download#infotabs)

##### MacOsx
```shell
user@host ~$ brew cask install nordic-nrf5x-command-line-tools
```

##### Linux
Go to the download page, choose the appropriate SW package, then extract similar to this example:
```shell
tar xf ~/Downloads/nRF-Command-Line-Tools_9_8_1_Linux-x86_64.tar
```
Next move the extracted folders to your desired destination:
```shell
sudo mv nrfjprog/ /opt/nrfjprog
sudo mv mergehex/ /opt/mergehex
```
And set up your path to find the programs:
```shell
export PATH="$PATH:/opt/nrfjprog"
export PATH="$PATH:/opt/mergehex"
```


#### Open Ocd
[Project Homepage](http://openocd.org/)

##### MacOsx
```shell
user@host ~$ brew install open-ocd
```

##### Linux
No instructions yet - still working on it!


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




## Debug / Flash

This Section assumes you have connected a USB cable to the interface MCU of the of the nRF52840 dev board.

### Open OCD
```shell
user@host ~$ openocd -f board/nordic_nrf52_dk.cfg
```

### Connect to GDB Server

#### GDB

```shell
user@host ~$ gdb target/thumbv7em-none-eabihf/rust-firmware

(gdb) target remote 127.0.0.1:2331

```

#### LLDB
** Attention ** Currently it is easier to use GDB due to the fact that LLDB e.g. does not support the monitor command out of the box.


```shell
user@host ~$ lldb
(lldb) platform select remote-gdb-server
(lldb) platform connect connect://127.0.0.1:2331
```


### Erase Flash
```shell
user@host ~$  nrfjprog --eraseall -f nrf52
```


#### Convert .elf file to intel hex file
```shell
arm-none-eabi-objcopy -O ihex target/thumbv7em-none-eabihf/debug/rust-firmware image.bin
```
