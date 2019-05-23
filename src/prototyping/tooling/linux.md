# Linux

## C (ARM Toolchain)
```shell
user@host ~$ sudo apt install gcc-arm-none-eabi
```
#### Rust Toolchain
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
No instructions yet - still working on it!
