# Approtect

If you get an error like `Error connecting DP: cannot read IDR`, this is likely related to a nrf52840 feature called
APPROTECT. This feature is designed to protect deployed code against debugging or readout and prevents the debugger from
attaching until the chip is reset.

## Resetting the chip
In general, APPROTECT can be reset via `openocd`'s `nrf52_recover` command.

**Important: This will erase the entire chip.**

As the implementations of APPROTECT have changed across revisions, please check how it is implemented for your specific
nRF52840 variant.
