use anyhow::Result;
use minifb::Window;

const WIDTH: usize = 160;
const HEIGHT: usize = 100;

pub struct Screen {
    pub fb: [[u8; WIDTH]; HEIGHT],
    pub window: Window,
    palette: Vec<u32>,
}

impl Screen {
    pub fn new() -> Self {
        let mut palette = Vec::new();
        for i in 0..=255 {
            palette.push((i << 16) | (i << 8) | i); // RGB
        }

        let window = Window::new(
            "VM",
            WIDTH,
            HEIGHT,
            minifb::WindowOptions {
                resize: false,
                scale: minifb::Scale::X4,
                ..minifb::WindowOptions::default()
            },
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        Screen {
            fb: [[0; WIDTH]; HEIGHT],
            window,
            palette,
        }
    }

    pub fn update(&mut self) -> Result<()> {
        let mut buffer: Vec<u32> = Vec::with_capacity(WIDTH * HEIGHT);

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let pixel = self.fb[y][x];
                buffer.push(self.palette[pixel as usize]);
            }
        }

        self.window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();

        Ok(())
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, palette_index: u8) {
        self.fb[y][x] = palette_index;
    }

    pub fn set_palette(&mut self, palette: Vec<u32>) {
        self.palette = palette;
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn clear(&mut self) {
        self.fb = [[0; WIDTH]; HEIGHT];
    }
}
