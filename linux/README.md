This folder holds configurations for devices that `eterm` can run
on. Right now, it's just the SPI1 device tree overlay (and source) for
the Quartz64 Model B. 

# Device Tree
To enable SPI1 on the Quartz, copy `quartz64b_spi1.dtbo` to
`/boot/dtbo` and run `sudo u-boot-update` (under Plebian). Follow the
device tree steps for your distro if you aren't on Plebian.

To compile the overlay, use CounterPillow's
[https://github.com/CounterPillow/overlay-examples](overlay-examples)
repository and place the dts file in the `quartz64b` directory.

# spidev Buffer Size
The `spidev` module defaults to a measly 4096-byte buffer, which isn't
big enough for the 2.66" display. Under Plebian and probably other
distros, this module is compiled into the kernel, so you can't add a
`modprobe.d` configuration file to change the buffer size. Instead, add
`spidev.bufsiz=65536` to the command line for your kernel, in u-boot.
