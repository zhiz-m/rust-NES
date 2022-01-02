pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

pub trait Display {
    fn write_pixel(row: usize, col: usize, color: Color);
}