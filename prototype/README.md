# Prototype
The development board used for the YAK prototype is the
[nRF52840 DK](https://www.nordicsemi.com/Software-and-Tools/Development-Kits/nRF52840-DK)

# Required Software
First off, you'll need the `thumbe7vm-none-eabihf` target for Rust, so let's add it as a new rustup
target:
```
rustup target add thumbv7em-none-eabihf
```

Second, you're going to need the ARM GCC package:
```
sudo apt install gcc-arm-none-eabi
```

Thirdly, you'll need the Nordic nRF Command Line Tools. You can find the latest version of the
appropriate set of tools for your OS:
[here on Nordic's website](https://www.nordicsemi.com/Software-and-Tools/Development-Tools/nRF-Command-Line-Tools/Download#infotabs)

You'll need both the JLink package and the Command Line Tools package included in the download.
Here's how you can install them (if you use Debian):
```
sudo dpkg -i JLink_Linux_V650b_x86_64.deb
sudo dpkg -i nRF-Command-Line-Tools_10_4_0_Linux-amd64.deb
```
