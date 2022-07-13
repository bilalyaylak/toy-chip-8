use sdl2::pixels::{Color, PixelFormat, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::chip8;

pub struct DisplayDevice {
    canvas: Canvas<Window>,
    background_color: Color,
    sprite_color: Color,
}
const SCALE_X: u8 = 12;
const SCALE_Y: u8 = 12;

impl DisplayDevice {
    pub fn new(sdl_context: &sdl2::Sdl, background_color: u32, sprite_color: u32) -> Self {
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
        let background_color = Color::from_u32(
            &PixelFormat::try_from(PixelFormatEnum::RGB24).unwrap(),
            background_color,
        );
        let sprite_color = Color::from_u32(
            &PixelFormat::try_from(PixelFormatEnum::RGB24).unwrap(),
            sprite_color,
        );
        canvas.set_draw_color(background_color);
        canvas.clear();
        canvas.present();

        DisplayDevice {
            canvas,
            background_color,
            sprite_color,
        }
    }

    pub fn draw(&mut self, pixels: &[bool; chip8::DISPLAY_WIDTH * chip8::DISPLAY_HEIGHT]) {
        self.canvas.set_draw_color(self.background_color);
        self.canvas.clear();
        self.canvas.set_draw_color(self.sprite_color);
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
