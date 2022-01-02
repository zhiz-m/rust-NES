use crate::cartridge::Cartridge;

pub struct Bus {

}

impl Bus {
    pub fn write(addr: u16, data: u8) {
        // todo
    }

    pub fn read(addr: u16, read_only: bool) -> u8 {
        // todo

        return 0x0000;
    }

    pub fn insert_cartridge(cartridge: &Cartridge) {
        // todo
    }

    pub fn reset() {
        // todo
    }

    pub fn clock_tick() {
        // todo
    }
}
