# InFerm
In the Fermenter, this code goes. This purpose of this project is to have a SG meter.

# Prerequisits
The rust target thumbv6m-none-eabi needs to be installed:
```console
rustup target add thumbv6m-none-eabi
```
Install the arm-none-eabi tools:
```console
sudo apt install binutils-arm-none-eabi
```

# Compiling
```console
cargo build
```
If you need an hex of binary file use:
```console
arm-none-eabi-objcopy -O hex target/thumbv6m-none-eabi/release/in_ferm target/in_ferm.hex
```

```console
arm-none-eabi-objcopy -O binary target/thumbv6m-none-eabi/release/in_ferm target/in_ferm.bin
```

# Flash & Debug

Install OpenOCD and GDB-multiarch
```console
sudo apt install   gdb-multiarch   openocd 
```

In a terminal run the following command:
```console
openocd -f openocd.cfg
```
This should have a output similar to this:
```console
Open On-Chip Debugger 0.10.0
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
none separate
adapter speed: 400 kHz
cortex_m reset_config sysresetreq
Info : No device selected, using first device.
Info : J-Link EDU Mini V1 compiled Jan  4 2021 16:18:35
Info : Hardware version: 1.00
Info : VTarget = 3.289 V
Info : clock speed 400 kHz
Info : SWD DPIDR 0x0bc11477
Info : at91samd.cpu: hardware has 4 breakpoints, 2 watchpoints
```

In another termianal run:
```console
cargo run
```
This will automatically load the binary and start debugging at main