use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::chip8;

pub struct DisplayDevice {
    canvas: Canvas<Window>,
}
const SCALE_X: u8 = 12;
const SCALE_Y: u8 = 12;

impl DisplayDevice {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(
                "Chip-8",
                chip8::DISPLAY_WIDTH as u32 * SCALE_X as u32,
                chip8::DISPLAY_HEIGHT as u32 * SCALE_Y as u32,
            )
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.present();

        DisplayDevice { canvas }
    }

    pub fn draw(&mut self, pixels: &[bool; chip8::DISPLAY_WIDTH * chip8::DISPLAY_HEIGHT]) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
        self.canvas.set_draw_color(Color::WHITE);
        for (index, pixel) in pixels.iter().enumerate() {
            if *pixel {
                let x = index % chip8::DISPLAY_WIDTH;
                let y = index / chip8::DISPLAY_WIDTH;
                let _ = self.canvas.fill_rect(Rect::new(
                    x as i32 * SCALE_X as i32,
                    y as i32 * SCALE_Y as i32,
                    SCALE_X as u32,
                    SCALE_Y as u32,
                ));
            }
        }
        self.canvas.present();
    }
}
