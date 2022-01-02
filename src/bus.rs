use crate::cartridge::Cartridge;
use crate::cpu::Cpu;

pub struct Bus {
    cpu: Cpu,
    cpu_ram: Vec<u8>
}

impl Bus {
    pub fn new() -> Bus{
        return Bus {
            cpu: Cpu::new(), 
            cpu_ram: vec![0x00; 2048],
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        // always evaluates to True for time being
        if addr <= 0x1FFF {
            self.cpu_ram[(addr & 0x1FFF) as usize] = data;
        }
    }

    pub fn read(&self, addr: u16, read_only: bool) -> u8 {
        // todo
        if addr <= 0x0800 {
            return self.cpu_ram[(addr & 0x1FFF) as usize]
        }
        else {
            return 0x0000
        }
    }

    pub fn insert_cartridge(&mut self, cartridge: &Cartridge) {
        // todo
    }

    pub fn reset(&mut self) {
        // todo
    }

    pub fn clock_tick(&mut self) {
        // todo
    }
}
