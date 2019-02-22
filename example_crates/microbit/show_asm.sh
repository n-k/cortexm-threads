#!/bin/bash
cargo objdump --bin microbit -- -disassemble -no-show-raw-insn -print-imm-hex | less
