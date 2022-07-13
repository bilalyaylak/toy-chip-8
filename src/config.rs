use clap::Parser;

/// A toy chip-8 interpreter
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    /// Rom file path
    #[clap(value_parser)]
    pub rom_path: String,

    /// Background color of display
    #[clap(long, value_parser, default_value_t = 0x000000)]
    pub background_color: u32,
    /// Sprite color
    #[clap(long, value_parser, default_value_t = 0xFFFFFF)]
    pub sprite_color: u32,
}
