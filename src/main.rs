mod ppu;
mod bus;
mod cpu;
mod cartridge;
mod mappers;
mod displays;
mod frontends;

use crate::frontends::{
    frontend::{Frontend},
    frontend01::{Frontend01}
};
use crate::displays::display::{ScreenBuffer, Pixel};

fn main() {
    let mut frontend = Frontend01::new();
    frontend.start().unwrap();
    let mut test_buf = ScreenBuffer::new();
    for i in 50..100{
        for j in 100..150{
            test_buf.write_pixel(i, j, Pixel::new(100,150,120));
        }
    }
    while frontend.render(&test_buf).unwrap(){

    }
}
