#!/bin/bash
qemu-system-arm \
      -cpu cortex-m3 \
      -machine lm3s6965evb \
      -gdb tcp::3333 \
      -S \
      -nographic \
      -semihosting-config enable=on,target=native \
      -kernel target/thumbv6m-none-eabi/debug/cortex-m0-threads

