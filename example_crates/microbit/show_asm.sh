#!/bin/bash
cargo objdump --bin microbit --release -- -disassemble -no-show-raw-insn -print-imm-hex | less
