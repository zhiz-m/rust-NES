use crate::cartridge::Cartridge;
use crate::apu::APU;
use crate::ppu::PPU;

pub struct Bus {
    pub cpu_ram: Vec<u8>,
    pub cartridge: Cartridge,

    pub apu: APU,
    pub ppu: PPU,
}

impl Bus {
    pub fn new(cartridge: Cartridge, apu: APU, ppu: PPU) -> Self {
        return Bus { 
            cpu_ram: vec![0x00; 2048],
            cartridge,
            apu,
            ppu
        };
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        if self.cartridge.cpu_write(addr, data) {
            return;
        }

        if addr <= 0x0800 {
            self.cpu_ram[addr as usize] = data;
        }
    }

    pub fn read(&self, addr: u16, read_only: bool) -> u8 {
        let cartridge_read = self.cartridge.cpu_read(addr);
        if !cartridge_read.is_none() {
            return cartridge_read.unwrap();
        }

        if addr <= 0x0800 {
            return self.cpu_ram[addr as usize];
        }

        return 0x00;
    }

    pub fn reset(&mut self) {
        self.cartridge.reset();
    }

    pub fn clock_tick(&mut self) {
        // todo
    }
}
