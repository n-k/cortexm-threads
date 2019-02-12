#!/bin/bash
cargo objdump --bin cortexm-threads --release -- -disassemble -no-show-raw-insn -print-imm-hex | less
