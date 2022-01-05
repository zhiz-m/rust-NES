use std::cell::RefCell;
use std::rc::Rc;

use crate::cartridge::Cartridge;
use crate::cpu::Cpu;

pub struct Bus {
    pub cpu: Option<Cpu>,
    pub cpu_ram: Vec<u8>,
    pub cartridge: Cartridge,
}

impl Bus {
    pub fn new(cartridge: Cartridge) -> Rc<RefCell<Bus>> {
        let bus = Rc::new(RefCell::new(Bus { 
            cpu: None,
            cpu_ram: vec![0x00; 2048],
            cartridge,
        }));

        let cpu = Cpu::new(bus.clone());
        bus.borrow_mut().cpu = Some(cpu);
        return bus;
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
        match &mut self.cpu {
            Some(cpu) => cpu.reset(),
            None => {},
        }

        self.cartridge.reset();
    }

    pub fn clock_tick(&mut self) {
        // todo
    }
}
