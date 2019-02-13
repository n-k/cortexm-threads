#!/bin/bash
cargo objdump --bin lm3s6965evb --release -- -disassemble -no-show-raw-insn -print-imm-hex | less
