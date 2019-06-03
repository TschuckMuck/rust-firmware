# Linux

## C (ARM Toolchain)
```shell
user@host ~$ sudo apt install gcc-arm-none-eabi
user@host ~$ sudo apt install gdb-arm-none-eabi
```
## Rust Toolchain
```shell
user@host ~$ curl https://sh.rustup.rs -sSf | sh
```

## SEGGER Debugging tools
Go to the download page, choose the appropriate JLink Software & Documentation package, then install
similar to this example:
```shell
user@host ~$ sudo dpkg -i ~/Downloads/JLink_Linux_V644i_x86_64.deb
```
## Nordic nRF5x command line tools
Go to the download page, choose the appropriate SW package, then extract similar to this example:
```shell
user@host ~$ tar xf ~/Downloads/nRF-Command-Line-Tools_9_8_1_Linux-x86_64.tar
```
Next move the extracted folders to your desired destination:
```shell
user@host ~$ sudo mv nrfjprog/ /opt/nrfjprog
user@host ~$ sudo mv mergehex/ /opt/mergehex
```
And set up your path to find the programs:
```shell
user@host ~$ export PATH="$PATH:/opt/nrfjprog"
user@host ~$ export PATH="$PATH:/opt/mergehex"
```

## Open Ocd
The openocd package provided to Ubuntu 16.04 sucks, so we'll build it on our own.
Building on our own also kind of sucks, but at least we get a proper OpenOCD installation when we're
done.

If you have a pre-existing version of OpenOCD coming from apt, we'll get rid of it
```shell
sudo apt purge openocd
```

Packages you need to have installed to configure OpenOCD:
```shell
sudo apt install autotools-dev automake libtool
```

Now, let's set up a proper OpenOCD installation
```shell
# clone the OpenOCD git repo
git clone https://github.com/ntfreak/openocd.git

cd openocd
# configure
./bootstrap
./configure
# build
make -j$(nrpoc)
# install
make install -j$(nproc)
```
