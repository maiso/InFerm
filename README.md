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

# Reflashing the Arduino bootloader
Connect the SEGGER Jlink Mini Edu to the Arduino Nano 33 IoT
[Link](https://wiki.segger.com/Arduino_Nano_33_IOT)

Retrieve a bootloader hex: [samd21_sam_ba_arduino_nano_33_iot.hex](https://github.com/arduino/ArduinoCore-samd/blob/master/bootloaders/nano_33_iot/samd21_sam_ba_arduino_nano_33_iot.hex)

Install the JLink tools provided by segger. 
Open a terminal and start JlinkExe
```console
opt/SEGGER/JLink/JLinkExe 
```

The output should look something like this:
```console
SEGGER J-Link Commander V6.95a (Compiled Jan 15 2021 16:29:10)
DLL version V6.95a, compiled Jan 15 2021 16:28:54

Connecting to J-Link via USB...O.K.
Firmware: J-Link EDU Mini V1 compiled Jan  4 2021 16:18:35
Hardware version: V1.00
S/N: 801030203
License(s): FlashBP, GDB
VTref=3.284V


Type "connect" to establish a target connection, '?' for help
J-Link>con
Please specify device / core. <Default>: CORTEX-M0+
Type '?' for selection dialog
Device>
Please specify target interface:
  J) JTAG (Default)
  S) SWD
  T) cJTAG
TIF>s
Specify target interface speed [kHz]. <Default>: 4000 kHz
Speed>
Device "CORTEX-M0+" selected.


Connecting to target via SWD
Found SW-DP with ID 0x0BC11477
DPIDR: 0x0BC11477
Scanning AP map to find all available APs
AP[1]: Stopped AP scan as end of AP map has been reached
AP[0]: AHB-AP (IDR: 0x04770031)
Iterating through AP map to find AHB-AP to use
AP[0]: Core found
AP[0]: AHB-AP ROM base: 0x41003000
CPUID register: 0x410CC601. Implementer code: 0x41 (ARM)
Found Cortex-M0 r0p1, Little endian.
FPUnit: 4 code (BP) slots and 0 literal slots
CoreSight components:
ROMTbl[0] @ 41003000
ROMTbl[0][0]: E00FF000, CID: B105100D, PID: 000BB4C0 ROM Table
ROMTbl[1] @ E00FF000
ROMTbl[1][0]: E000E000, CID: B105E00D, PID: 000BB008 SCS
ROMTbl[1][1]: E0001000, CID: B105E00D, PID: 000BB00A DWT
ROMTbl[1][2]: E0002000, CID: B105E00D, PID: 000BB00B FPB
ROMTbl[0][1]: 41006000, CID: B105900D, PID: 001BB932 MTB-M0+
Cortex-M0 identified.
J-Link>loadfile samd21_sam_ba_arduino_nano_33_iot.hex
Downloading file [samd21_sam_ba_arduino_nano_33_iot.hex]...
O.K.
J-Link>q
```