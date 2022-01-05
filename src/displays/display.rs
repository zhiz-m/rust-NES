#[derive(Clone, Copy)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pixel{
    pub fn new(r: u8, g: u8, b: u8) -> Pixel{
        return Pixel { r, g, b }
    }
}

pub struct ScreenBuffer {
    buffer: [[Pixel; 256]; 240],
}

impl ScreenBuffer{
    pub fn new() -> ScreenBuffer{
        return ScreenBuffer{
            buffer: [[Pixel::new(0,0,0); 256]; 240],
        }
    }
    pub fn write_pixel(&mut self, row: usize, col: usize, pixel: Pixel){
        self.buffer[row][col] = pixel;
    }
    pub fn read_pixel(&self, row: usize, col: usize) -> Pixel{
        return self.buffer[row][col];
    }
}