use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct DisplayDevice {
    canvas: Canvas<Window>,
}

impl DisplayDevice {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Chip-8", 640, 320)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.present();

        DisplayDevice { canvas }
    }

    pub fn draw(&mut self, pixels: &[bool; 64 * 32]) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
        self.canvas.set_draw_color(Color::WHITE);
        for (index, pixel) in pixels.iter().enumerate() {
            if *pixel {
                let x = index % 64;
                let y = index / 64;
                let _ = self
                    .canvas
                    .fill_rect(Rect::new(x as i32 * 10, y as i32 * 10, 10, 10));
            }
        }
        self.canvas.present();
    }
}
