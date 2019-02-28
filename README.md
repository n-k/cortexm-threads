# CORTEXM_THREADS

A simple implementation of thread context switching for 
ARM Cortex-M series microcontrollers.

## Examples
The `example_crates` folder contains crates showing how to 
use cortxm-threads for different boards.

Available examples:
 - [stm32f3](./example_crates/stm32f3) - 2 threads with one 
 thread running an LED roulette, and the other periodically
 printing magnetometer readings. Currently compiles for target
 thumbv7em-none-eabi instead of thumbv7em-none-eabihf. See Roadmap#1
 - [microbit](./example_crates/microbit) - 2 threads printing
 messages with co-operative context switching
 - [lm3s6965evb](./example_crates/lm3s6965evb) - (set up to run
 on qemu) 2 threads printing messages via semi-hosting.
 Run `cargo run` from `example_crates/lm3s6965evb` directory
 to see it running. You must have qemu-system-arm on the system PATH.

## Roadmap
 - Implement PendSV handler for thumbv7em-none-eabihf. thumbv7em-none-eabihf
 has more registers for FPU which must be saved/loaded during context switch.
 Switch stm32f3 example to thumbv7em-none-eabihf target after this item.
 - Implement thread priorities, idle thread, efficient sleep
 - Implement some form of IPC
