# gb-rust

[![Build Status](https://travis-ci.org/JJag/gb-rust.svg?branch=master)](https://travis-ci.org/JJag/gb-rust)

~Work-In-Progress~ _Unfinished_ classic GameBoy emulator written for educational purposes.

![](img/sml.gif)

## Running

`cargo run --release path/to/game_file.gb`

Only MBC1 bank controller is implemented currently, so most games won't run.

Controls:
* D-pad - arrows
* A - Z
* B - X
* Start - Q
* Select - W
## Blargg's test ROMs status:

| Test                       | Status       |
|----------------------------|--------------|
| `01-special.gb`            | PASSED       |
| `02-interrupts.gb`         | PASSED       |
| `03-op sp,hl.gb`           | PASSED       |
| `04-op r,imm.gb`           | PASSED       |
| `05-op rp.gb`              | PASSED       |
| `06-ld r,r.gb`             | PASSED       |
| `07-jr,jp,call,ret,rst.gb` | PASSED       |
| `08-misc instrs.gb`        | PASSED       |
| `09-op r,r.gb`             | PASSED       |
| `10-bit ops.gb`            | PASSED       |
| `11-op a,(hl).gb`          | PASSED       |
