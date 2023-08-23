# Approtect

If you get an error like `Error connecting DP: cannot read IDR`, this is likely related to a nrf52840 feature called
APPROTECT. This feature is designed to protect deployed code against debugging or readout and prevents the debugger from
attaching until the chip is reset.

## Resetting the chip
1. Connect OpenOCD to your nrf52840 board
2. Connect to OpenOCD via `telnet ADDRESS PORT`; e.g. `telnet localhost 4444`
3. Check if APPROTECT is enabled: `nrf52.dap apreg 1 0x04`
   1. If the return value is `0x00000000`, then APPROTECT is enabled
4. Resetting the chip to disable APPROTECT: `nrf52.dap apreg 1 0x04 0x01`
   1. If you run `nrf52.dap apreg 1 0x04` again, the return value should be `0x00000001` and the chip should be
      programmable and debuggable
