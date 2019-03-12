# CORTEXM_THREADS

A simple library for context-switching on ARM Cortex-M ( 0, 0+, 3, 4, 4F ) micro-processors

Supports pre-emptive, priority based switching

This project is meant for learning and should be used only at the user's risk. For practical and mature
rust alternatives, see [Awesome Embedded Rust](https://github.com/rust-embedded/awesome-embedded-rust)


## Current State
Processor support:

 - [x] Cortex-M0
 - [x] Cortex-M0+
 - [x] Cortex-M3
 - [ ] Cortex-M4
 - [ ] Cortex-M4F

Features:
 - [x] Preemptive, priority-based switching
 - [x] Efficient sleep
 - [ ] Non-privileged mode
 - [ ] Mutex implementation aware of thread scheduling


## Examples
The `example_crates` folder contains crates showing how to 
use cortexm-threads for different boards.

Available examples:
 - [stm32f3](./example_crates/stm32f3) - 2 threads with one 
 thread running an LED roulette, and the other periodically
 printing magnetometer readings. Currently compiles for target
 thumbv7m-none-eabi instead of thumbv7em-none-eabihf. See Roadmap#1
 - [microbit](./example_crates/microbit) - 2 threads printing
 messages with co-operative context switching
 - [qemu-m4](./example_crates/qemu-m4) - (set up to run
 on qemu) 2 threads printing messages via semi-hosting.
 Run `cargo run` from `example_crates/qemu-m4` directory
 to see it running. You must have qemu-system-arm on the system PATH.

