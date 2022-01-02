// Emulates 6502 CPU

use std::{ptr::null_mut, convert::TryInto};

use crate::bus::Bus;

enum StatusFlag{
    C = 1 << 0,
    Z = 1 << 1,
    I = 1 << 2,
    D = 1 << 3,
    B = 1 << 4,
    U = 1 << 5,
    V = 1 << 6,
    N = 1 << 7,
}

#[derive(Clone)]
struct Instruction{
    name: &'static str,
    operation: fn(&mut Cpu) -> u8,
    addr_mode: fn(&mut Cpu) -> u8,
    cycles: u8,
}

type I = Instruction;
pub struct Cpu {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub pc: u16,
    pub status: u8,

    fetched: u8,
    temp: u16,
    addr_abs: u16,
    addr_rel: u16,
    opcode: u8,
    cycles: u8,
    clock_count: u8,

    lookup: Vec<Instruction>,

    bus_ptr: *mut Bus,
}

impl Cpu {
    pub fn new() -> Cpu{
        return Cpu{
            a: 0,
            x: 0,
            y: 0,
            sp: 0,
            pc: 0,
            status: 0,
            fetched: 0,
            temp: 0,
            addr_abs: 0,
            addr_rel: 0,
            opcode: 0,
            cycles: 0,
            clock_count: 0,
            lookup: vec![
                I{name: "BRK", operation: Cpu::BRK, addr_mode: Cpu::IMM, cycles: 7}
            ],
            bus_ptr: null_mut(),
        };
    }

    pub fn attach_bus(&mut self, bus_ptr: *mut Bus){
        self.bus_ptr = bus_ptr;
    }

    // helper methods
    fn set_flag(&mut self, f: StatusFlag, val: bool){
        if val{
            self.status |= f as u8;
        }
        else{
            self.status &= u8::MAX - f as u8;
        }
    }

    fn read_flag(&self, f: StatusFlag) -> bool{
        return self.status & f as u8 > 0;
    }

    fn read(&self, addr: u16) -> u8{
        unsafe{
            return (*self.bus_ptr).cpu_ram[addr as usize];
        }
    }

    fn write(&self, addr: u16, val: u8){
        unsafe{
            (*self.bus_ptr).cpu_ram[addr as usize] = val;
        }
    }
    
    // address modes
    fn IMM(&mut self) -> u8{
        self.addr_abs = self.pc;
        self.pc += 1;
        return 0;
    }

    // operations
    fn BRK(&mut self) -> u8{
        self.pc += 1;

        self.set_flag(StatusFlag::I, true);
        self.write(0x0100 + self.sp as u16, ((self.pc >> 8) & 0x00FF).try_into().unwrap());
        self.sp -= 1;
        self.write(0x0100 + self.sp as u16, (self.pc & 0x00FF).try_into().unwrap());
        self.sp -= 1;

        self.set_flag(StatusFlag::B, true);
        self.write(0x0100 + self.sp as u16, self.status);
        self.sp -= 1;
        self.set_flag(StatusFlag::B, false);

        self.pc = self.read(0xFFFE) as u16 | ((self.read(0xFFFF) as u16) << 8);

        return 0x00;
    }

    // public methods
    fn clock(&mut self){
        if self.cycles == 0{
            self.opcode = self.read(self.pc);

            self.set_flag(StatusFlag::U, true);
            self.pc+=1;
            let instr = self.lookup[self.opcode as usize].clone();
            self.cycles = instr.cycles;
            let extra_cycle1 = (instr.addr_mode)(self);
            let extra_cycle2 = (instr.operation)(self);
            self.cycles += extra_cycle1 & extra_cycle2;
            self.set_flag(StatusFlag::U, true);
        }
    }
}
