## To use this, run the following in a separate shell:
## openocd -f interface/cmsis-dap.cfg -f target/nrf51.cfg

# Disable "Inferior 1 [Remote target] will be detached; Quit anyway?"
define hook-quit
    set confirm off
end

# Connects GDB to OpenOCD server port
target extended-remote :3333
# (optional) Unmangle function names when debugging
set print asm-demangle on
# set backtrace limit to not have infinite backtrace loops
set backtrace limit 32
# Enable semihosting
monitor arm semihosting enable

## Uncomment this to automatically break on panic.
## See https://github.com/rust-embedded/cortex-m-rt/issues/139#issuecomment-518337332
# break core::panicking::panic

# Load your program, breaks at entry
load

# Continue with execution
continue
