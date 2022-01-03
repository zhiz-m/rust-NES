use std::cell::RefCell;
use std::rc::Rc;

use crate::cartridge::Cartridge;
use crate::cpu::Cpu;

pub struct Bus {
    pub cpu: Option<Cpu>,
    pub cpu_ram: Vec<u8>
}

impl Bus {
    pub fn new() -> Rc<RefCell<Bus>> {
        let bus = Rc::new(RefCell::new(Bus { 
            cpu: None,
            cpu_ram: vec![0x00; 2048],
        }));
        let cpu = Cpu::new(bus.clone());
        bus.borrow_mut().cpu = Some(cpu);
        return bus;
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        // always evaluates to True for time being
        if addr <= 0x0800{
            self.cpu_ram[addr as usize] = data;
        }
    }

    pub fn read(&self, addr: u16, read_only: bool) -> u8 {
        // todo
        if addr <= 0x0800{
            return self.cpu_ram[addr as usize];
        }
        return 0x00;
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
