# toy-chip-8
A Chip-8 interpreter written in Rust.

## References

 - [Cowgod's Chip-8 Technical Reference v1.0](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
 - [Guide to making a CHIP-8 emulator](https://tobiasvl.github.io/blog/write-a-chip-8-emulator)
 - [Chip 8 Instruction Scheduling and Frequency](https://jackson-s.me/2019/07/13/Chip-8-Instruction-Scheduling-and-Frequency.html)
Also thanks to [Timendus](https://github.com/Timendus) for [CHIP-8 test suite](https://github.com/Timendus/chip8-test-suite), such a great helper.
## Requirements
Follow installation guide of [rust-sdl2](https://github.com/Rust-SDL2/rust-sdl2). Then run following command.

    cargo build
## Usage

    toy-chip-8 0.1.0
    A toy chip-8 interpreter
    
    USAGE:
        toy-chip-8 [OPTIONS] <ROM_PATH>
    
    ARGS:
        <ROM_PATH>    Rom file path
    
    OPTIONS:
            --background-color <BACKGROUND_COLOR>    Background color of display [default: 0]
        -h, --help                                   Print help information
            --sprite-color <SPRITE_COLOR>            Sprite color [default: 16777215]
        -V, --version                                Print version information
