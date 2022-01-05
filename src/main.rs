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
use crate::displays::display::{ScreenBuffer};

fn main() {
    let mut frontend = Frontend01::new();
    frontend.start().unwrap();
    let dummy_buf = ScreenBuffer::new();
    while frontend.render(&dummy_buf).unwrap(){

    }
}
