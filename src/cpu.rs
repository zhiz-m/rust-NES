// Emulates 6502 CPU

use std::{ptr::null_mut, convert::TryInto, cell::RefCell, rc::Rc};

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
    operation: fn(&mut CPU) -> u8,
    addr_mode: fn(&mut CPU) -> u8,
    cycles: u8,
}

type I = Instruction;
pub struct CPU {
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

    bus: Bus,
}

impl CPU {
    pub fn new(bus: Bus) -> CPU{
        return CPU{
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
                I{name: "BRK", operation: CPU::BRK, addr_mode: CPU::IMM, cycles: 7}
            ],
            bus,
        };
    }

    pub fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0;
        self.pc = 0;
        self.status = 0;
        self.fetched = 0;
        self.temp = 0;
        self.addr_abs = 0;
        self.addr_rel = 0;
        self.opcode = 0;
        self.cycles = 0;
        self.clock_count = 0;
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
        return self.bus.read(addr, false);
    }

    fn write(&mut self, addr: u16, val: u8){
        self.bus.write(addr, val);
    }

    fn fetch(&mut self) -> u8{
        // might be sketchy
        if self.lookup[self.opcode as usize].addr_mode as usize != CPU::IMP as usize{
            self.fetched = self.read(self.addr_abs);
        }
        return self.fetched;
    }
    
    // address modes
    fn IMP(&mut self) -> u8{
        self.fetched = self.a;
        return 0;
    }
    
    fn IMM(&mut self) -> u8{
        self.addr_abs = self.pc;
        self.pc += 1;
        return 0;
    }

    fn ZP0(&mut self) -> u8{
        self.addr_abs = self.read(self.pc) as u16;
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        return 0;
    }

    fn ZPX(&mut self) -> u8{
        self.addr_abs = self.read(self.pc + self.x as u16) as u16;
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        return 0;
    }

    fn ZPY(&mut self) -> u8{
        self.addr_abs = self.read(self.pc + self.y as u16) as u16;
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        return 0;
    }

    fn REL(&mut self) -> u8{
        self.addr_rel = self.read(self.pc) as u16;
        self.pc += 1;
        if self.addr_abs & 0x80 > 0{
            self.addr_rel |= 0xFF00;
        }
        return 0;
    }

    fn ABS(&mut self) -> u8{
        let lo = self.read(self.pc) as u16;
        self.pc += 1;
        let hi = self.read(self.pc) as u16;
        self.pc += 1;
        
        self.addr_abs = (hi << 8) | lo;

        return 0;
    }

    fn ABX(&mut self) -> u8{
        let lo = self.read(self.pc) as u16;
        self.pc += 1;
        let hi = self.read(self.pc) as u16;
        self.pc += 1;
        
        self.addr_abs = (hi << 8) | lo + self.x as u16;
        if (self.addr_abs >> 8) != hi{
            return 1;
        }
        return 0;
    }

    fn ABY(&mut self) -> u8{
        let lo = self.read(self.pc) as u16;
        self.pc += 1;
        let hi = self.read(self.pc) as u16;
        self.pc += 1;
        
        self.addr_abs = (hi << 8) | lo + self.y as u16;
        if (self.addr_abs >> 8) != hi{
            return 1;
        }
        return 0;
    }

    fn IND(&mut self) -> u8{
        let lo = self.read(self.pc) as u16;
        self.pc += 1;
        let hi = self.read(self.pc) as u16;
        self.pc += 1;

        let ptr = (hi << 8) | lo;
        if lo == 0x00FF{
            self.addr_abs = ((self.read(ptr & 0xFF00) as u16) << 8) | self.read(ptr) as u16;
        }
        else{
            self.addr_abs = ((self.read(ptr + 1) as u16) << 8) | self.read(ptr) as u16;
        }
        return 0;
    }

    fn IZX(&mut self) -> u8{
        let t = self.read(self.pc) as u16;
        self.pc += 1;

        let lo = self.read((t + self.x as u16) & 0x00FF) as u16;
        let hi = self.read((t + self.x as u16 + 1) & 0x00FF) as u16;

        self.addr_abs = (hi << 8) + lo;
        return 0;
    }

    fn IZY(&mut self) -> u8{
        let t = self.read(self.pc) as u16;
        self.pc += 1;

        let lo = self.read((t + self.y as u16) & 0x00FF) as u16;
        let hi = self.read((t + self.y as u16 + 1) & 0x00FF) as u16;

        self.addr_abs = (hi << 8) + lo;
        return 0;
    }

    // operations
    fn ADC(&mut self) -> u8{
        self.fetch();
        self.temp = self.a as u16 + self.fetched as u16 + self.read_flag(StatusFlag::C) as u16;
        self.set_flag(StatusFlag::C, self.temp > 255);
        self.set_flag(StatusFlag::Z, (self.temp & 0x00FF) == 0);
        self.set_flag(StatusFlag::V, !(self.a as u16 ^ self.fetched as u16) & (self.a as u16 ^ self.temp) & 0x0080 > 0);
        self.set_flag(StatusFlag::N, self.temp & 0x0080 > 0);

        self.a = (self.temp & 0x00FF).try_into().unwrap();

        return 1;
    }

    fn SBC(&mut self) -> u8{
        self.fetch();

        let value = (self.fetched ^ 0x00FF) as u16;
        self.temp = self.a as u16 + value as u16 + self.read_flag(StatusFlag::C) as u16;
        self.set_flag(StatusFlag::C, self.temp > 255);
        self.set_flag(StatusFlag::Z, (self.temp & 0x00FF) == 0);
        self.set_flag(StatusFlag::V, (self.a as u16 ^ self.temp) & (value ^ self.temp) & 0x0080 > 0);
        self.set_flag(StatusFlag::N, self.temp & 0x0080 > 0);

        self.a = (self.temp & 0x00FF).try_into().unwrap();

        return 1;
    }

    fn AND(&mut self) -> u8 {
        self.fetch();
        self.a = self.a & self.fetched;
        self.set_flag(StatusFlag::Z, self.a == 0x00);
        self.set_flag(StatusFlag::N, self.a & 0x80 > 0);
        return 1;
    }

    fn ASL(&mut self) -> u8{
        self.fetch();
        self.temp = (self.fetched as u16) << 1;
        self.set_flag(StatusFlag::C, self.temp > 0x00FF);
        self.set_flag(StatusFlag::Z, self.temp & 0x00FF == 0x00);
        self.set_flag(StatusFlag::N, self.temp & 0x80 > 0);
        let ret = (self.temp & 0x00FF) as u8;
        if self.lookup[self.opcode as usize].addr_mode as usize != CPU::IMP as usize{
            self.a = ret;
        }
        else{
            self.write(self.addr_abs, ret);
        }
        return 0;
    }

    fn BCC(&mut self) -> u8{
        if !self.read_flag(StatusFlag::C){
            self.cycles += 1;
            self.addr_abs = self.pc + self.addr_rel;
            if self.addr_abs & 0xFF00 != self.pc & 0xFF00{
                self.cycles += 1;
            }
            self.pc = self.addr_abs;
        }
        return 0;
    }

    fn BCS(&mut self) -> u8{
        if self.read_flag(StatusFlag::C){
            self.cycles += 1;
            self.addr_abs = self.pc + self.addr_rel;
            if self.addr_abs & 0xFF00 != self.pc & 0xFF00{
                self.cycles += 1;
            }
            self.pc = self.addr_abs;
        }
        return 0;
    }

    fn BEQ(&mut self) -> u8{
        if self.read_flag(StatusFlag::Z){
            self.cycles += 1;
            self.addr_abs = self.pc + self.addr_rel;
            if self.addr_abs & 0xFF00 != self.pc & 0xFF00{
                self.cycles += 1;
            }
            self.pc = self.addr_abs;
        }
        return 0;
    }

    fn BIT(&mut self) -> u8 {
        self.fetch();
        self.temp = (self.a & self.fetched) as u16;
        self.set_flag(StatusFlag::Z, self.temp == 0x00);
        self.set_flag(StatusFlag::N, self.temp & (1<<7) > 0);
        self.set_flag(StatusFlag::V, self.temp & (1<<6) > 0);
        return 0;
    }

    fn BMI(&mut self) -> u8{
        if self.read_flag(StatusFlag::N){
            self.cycles += 1;
            self.addr_abs = self.pc + self.addr_rel;
            if self.addr_abs & 0xFF00 != self.pc & 0xFF00{
                self.cycles += 1;
            }
            self.pc = self.addr_abs;
        }
        return 0;
    }

    fn BNE(&mut self) -> u8{
        if !self.read_flag(StatusFlag::Z){
            self.cycles += 1;
            self.addr_abs = self.pc + self.addr_rel;
            if self.addr_abs & 0xFF00 != self.pc & 0xFF00{
                self.cycles += 1;
            }
            self.pc = self.addr_abs;
        }
        return 0;
    }

    fn BPL(&mut self) -> u8{
        if !self.read_flag(StatusFlag::N){
            self.cycles += 1;
            self.addr_abs = self.pc + self.addr_rel;
            if self.addr_abs & 0xFF00 != self.pc & 0xFF00{
                self.cycles += 1;
            }
            self.pc = self.addr_abs;
        }
        return 0;
    }

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

        return 0;
    }

    fn BVC(&mut self) -> u8{
        if !self.read_flag(StatusFlag::V){
            self.cycles += 1;
            self.addr_abs = self.pc + self.addr_rel;
            if self.addr_abs & 0xFF00 != self.pc & 0xFF00{
                self.cycles += 1;
            }
            self.pc = self.addr_abs;
        }
        return 0;
    }

    fn BVS(&mut self) -> u8{
        if self.read_flag(StatusFlag::V){
            self.cycles += 1;
            self.addr_abs = self.pc + self.addr_rel;
            if self.addr_abs & 0xFF00 != self.pc & 0xFF00{
                self.cycles += 1;
            }
            self.pc = self.addr_abs;
        }
        return 0;
    }

    fn CLC(&mut self) -> u8{
        self.set_flag(StatusFlag::C, false);
        return 0;
    }

    fn CLD(&mut self) -> u8{
        self.set_flag(StatusFlag::D, false);
        return 0;
    }

    fn CLI(&mut self) -> u8{
        self.set_flag(StatusFlag::I, false);
        return 0;
    }

    fn CLV(&mut self) -> u8{
        self.set_flag(StatusFlag::V, false);
        return 0;
    }

    fn CMP(&mut self) -> u8{
        self.fetch();
        self.temp = self.a as u16 - self.fetched as u16;
        self.set_flag(StatusFlag::C, self.a >= self.fetched);
        self.set_flag(StatusFlag::Z, self.temp & 0x00FF == 0);
        self.set_flag(StatusFlag::N, self.temp & 0x0080 > 0);
        return 0;
    }

    fn CPX(&mut self) -> u8{
        self.fetch();
        self.temp = self.x as u16 - self.fetched as u16;
        self.set_flag(StatusFlag::C, self.x >= self.fetched);
        self.set_flag(StatusFlag::Z, self.temp & 0x00FF == 0);
        self.set_flag(StatusFlag::N, self.temp & 0x0080 > 0);
        return 0;
    }

    fn CPY(&mut self) -> u8{
        self.fetch();
        self.temp = self.y as u16 - self.fetched as u16;
        self.set_flag(StatusFlag::C, self.y >= self.fetched);
        self.set_flag(StatusFlag::Z, self.temp & 0x00FF == 0);
        self.set_flag(StatusFlag::N, self.temp & 0x0080 > 0);
        return 0;
    }

    fn DEC(&mut self) -> u8{
        self.fetch();
        self.temp = self.fetched as u16 - 1;
        self.write(self.addr_abs, (self.temp & 0x00FF).try_into().unwrap());
        self.set_flag(StatusFlag::Z, self.temp & 0x00FF == 0);
        self.set_flag(StatusFlag::N, self.temp & 0x0080 > 0);
        return 0;
    }

    fn DEX(&mut self) -> u8{
        self.x -= 1;
        self.set_flag(StatusFlag::Z, self.x == 0);
        self.set_flag(StatusFlag::N, self.x & 0x0080 > 0);
        return 0;
    }

    fn DEY(&mut self) -> u8{
        self.y -= 1;
        self.set_flag(StatusFlag::Z, self.y == 0);
        self.set_flag(StatusFlag::N, self.y & 0x0080 > 0);
        return 0;
    }

    fn EOR(&mut self) -> u8{
        self.fetch();
        self.a ^= self.fetched;
        self.set_flag(StatusFlag::Z, self.a == 0);
        self.set_flag(StatusFlag::N, self.a & 0x0080 > 0);
        return 0;
    }

    fn INC(&mut self) -> u8{
        self.fetch();
        self.temp = self.fetched as u16 + 1;
        self.write(self.addr_abs, (self.temp & 0x00FF).try_into().unwrap());
        self.set_flag(StatusFlag::Z, self.temp & 0x00FF == 0);
        self.set_flag(StatusFlag::N, self.temp & 0x0080 > 0);
        return 0;
    }

    fn INX(&mut self) -> u8{
        self.x += 1;
        self.set_flag(StatusFlag::Z, self.x == 0);
        self.set_flag(StatusFlag::N, self.x & 0x0080 > 0);
        return 0;
    }

    fn INY(&mut self) -> u8{
        self.y += 1;
        self.set_flag(StatusFlag::Z, self.y == 0);
        self.set_flag(StatusFlag::N, self.y & 0x0080 > 0);
        return 0;
    }

    fn JMP(&mut self) -> u8{
        self.pc = self.addr_abs;
        return 0;
    }

    fn JSR(&mut self) -> u8{
        self.pc -= 1;
        self.write(0x0100 + self.sp as u16, ((self.pc >> 8) & 0x00FF).try_into().unwrap());
        self.sp -= 1;
        self.write(0x0100 + self.sp as u16, (self.pc & 0x00FF).try_into().unwrap());
        self.sp -= 1;

        self.pc = self.addr_abs;
        return 0;
    }

    fn LDA(&mut self) -> u8{
        self.fetch();
        self.a = self.fetched;
        self.set_flag(StatusFlag::Z, self.a == 0);
        self.set_flag(StatusFlag::N, self.a & 0x0080 > 0);
        return 0;
    }

    fn LDX(&mut self) -> u8{
        self.fetch();
        self.x = self.fetched;
        self.set_flag(StatusFlag::Z, self.x == 0);
        self.set_flag(StatusFlag::N, self.x & 0x0080 > 0);
        return 0;
    }

    fn LDY(&mut self) -> u8{
        self.fetch();
        self.y = self.fetched;
        self.set_flag(StatusFlag::Z, self.y == 0);
        self.set_flag(StatusFlag::N, self.y & 0x0080 > 0);
        return 0;
    }

    fn LSR(&mut self) -> u8{
        self.fetch();
        self.temp = (self.fetched as u16) >> 1;
        self.set_flag(StatusFlag::C, self.fetched & 1 > 0);
        self.set_flag(StatusFlag::Z, self.temp & 0x00FF == 0x00);
        self.set_flag(StatusFlag::N, self.temp & 0x80 > 0);
        let ret = (self.temp & 0x00FF) as u8;
        if self.lookup[self.opcode as usize].addr_mode as usize != CPU::IMP as usize{
            self.a = ret;
        }
        else{
            self.write(self.addr_abs, ret);
        }
        return 0;
    }

    fn NOP(&mut self) -> u8{
        if self.opcode == 0xFC{
            return 1;
        }
        return 0;
    }

    fn ORA(&mut self) -> u8{
        self.fetch();
        self.a = self.a | self.fetched;
        self.set_flag(StatusFlag::Z, self.a == 0x00);
        self.set_flag(StatusFlag::N, self.a & 0x80 > 0);
        return 1;
    }

    fn PHA(&mut self) -> u8{
        self.write(0x0100 + self.sp as u16, self.a);
        self.sp -= 1;
        return 0;
    }

    fn PHP(&mut self) -> u8{
        self.write(0x0100 + self.sp as u16, self.status | StatusFlag::B as u8 | StatusFlag::U as u8);
        self.sp -= 1;
        self.set_flag(StatusFlag::B, false);
        self.set_flag(StatusFlag::U, false);

        return 0;
    }

    fn PLA(&mut self) -> u8{
        self.sp += 1;
        self.a = self.read(0x0100 + self.sp as u16);
        self.set_flag(StatusFlag::Z, self.a == 0);
        self.set_flag(StatusFlag::N, self.a & 0x80 > 0);

        return 0;
    }

    fn PLP(&mut self) -> u8{
        self.sp += 1;
        self.status = self.read(0x0100 + self.sp as u16);
        self.set_flag(StatusFlag::U, true);

        return 0;
    }

    fn ROL(&mut self) -> u8{
        self.fetch();
        self.temp = ((self.fetched as u16) << 1) | self.read_flag(StatusFlag::C) as u16;
        self.set_flag(StatusFlag::C, self.temp & 0xFF00 > 0);
        self.set_flag(StatusFlag::Z, self.temp & 0x00FF == 0);
        self.set_flag(StatusFlag::N, self.temp & 0x80 > 0);
        let ret = (self.temp & 0x00FF) as u8;
        if self.lookup[self.opcode as usize].addr_mode as usize != CPU::IMP as usize{
            self.a = ret;
        }
        else{
            self.write(self.addr_abs, ret);
        }
        return 0;
    }

    fn ROR(&mut self) -> u8{
        self.fetch();
        self.temp = ((self.fetched as u16) >> 1) | ((self.read_flag(StatusFlag::C) as u16) << 7);
        self.set_flag(StatusFlag::C, self.fetched & 1 > 0);
        self.set_flag(StatusFlag::Z, self.temp & 0x00FF == 0);
        self.set_flag(StatusFlag::N, self.temp & 0x80 > 0);
        let ret = (self.temp & 0x00FF) as u8;
        if self.lookup[self.opcode as usize].addr_mode as usize != CPU::IMP as usize{
            self.a = ret;
        }
        else{
            self.write(self.addr_abs, ret);
        }
        return 0;
    }

    fn RTI(&mut self) -> u8{
        self.sp += 1;
        self.status = self.read(0x0100 + self.sp as u16);
        self.status &= !(StatusFlag::B as u8);
        self.status &= !(StatusFlag::U as u8);
        
        self.sp += 1;
        self.pc = self.read(0x0100 + self.sp as u16) as u16;
        self.sp += 1;
        self.pc |= (self.read(0x0100 + self.sp as u16) as u16) << 8;

        return 0;
    }

    fn RTS(&mut self) -> u8{
        self.sp += 1;
        self.pc = self.read(0x0100 + self.sp as u16) as u16;
        self.sp += 1;
        self.pc |= (self.read(0x0100 + self.sp as u16) as u16) << 8;
        
        self.pc += 1;
        return 0;
    }

    fn SEC(&mut self) -> u8{
        self.set_flag(StatusFlag::C, true);
        return 0;
    }

    fn SED(&mut self) -> u8{
        self.set_flag(StatusFlag::D, true);
        return 0;
    }

    fn SEI(&mut self) -> u8{
        self.set_flag(StatusFlag::I, true);
        return 0;
    }

    fn STA(&mut self) -> u8{
        self.write(self.addr_abs, self.a);
        return 0;
    }

    fn STX(&mut self) -> u8{
        self.write(self.addr_abs, self.x);
        return 0;
    }

    fn STY(&mut self) -> u8{
        self.write(self.addr_abs, self.y);
        return 0;
    }

    fn TAX(&mut self) -> u8{
        self.x = self.a;
        self.set_flag(StatusFlag::Z, self.x == 0);
        self.set_flag(StatusFlag::N, self.x & 0x80 > 0);
        return 0;
    }

    fn TAY(&mut self) -> u8{
        self.y = self.a;
        self.set_flag(StatusFlag::Z, self.y == 0);
        self.set_flag(StatusFlag::N, self.y & 0x80 > 0);
        return 0;
    }

    fn TSX(&mut self) -> u8{
        self.x = self.sp;
        self.set_flag(StatusFlag::Z, self.x == 0);
        self.set_flag(StatusFlag::N, self.x & 0x80 > 0);
        return 0;
    }

    fn TXA(&mut self) -> u8{
        self.a = self.x;
        self.set_flag(StatusFlag::Z, self.a == 0);
        self.set_flag(StatusFlag::N, self.a & 0x80 > 0);
        return 0;
    }

    fn TXS(&mut self) -> u8{
        self.sp = self.x;
        return 0;
    }

    fn TYA(&mut self) -> u8{
        self.a = self.y;
        self.set_flag(StatusFlag::Z, self.a == 0);
        self.set_flag(StatusFlag::N, self.a & 0x80 > 0);
        return 0;
    }

    fn UNK(&mut self) -> u8{
        return 0;
    }

    // public methods
    pub fn irq(&mut self){
        if self.read_flag(StatusFlag::I){
            self.write(0x0100 + self.sp as u16, ((self.pc >> 8) & 0x00FF).try_into().unwrap());
            self.sp -= 1;
            self.write(0x0100 + self.sp as u16, (self.pc & 0x00FF).try_into().unwrap());
            self.sp -= 1;

            self.set_flag(StatusFlag::B, false);
            self.set_flag(StatusFlag::U, true);
            self.set_flag(StatusFlag::I, true);
            self.write(0x0100 + self.sp as u16, self.status);
            self.sp -= 1;

            self.addr_abs = 0xFFFE;
            let lo = self.read(self.addr_abs) as u16;
            let hi = self.read(self.addr_abs+1) as u16;
            self.pc = (hi << 8) | lo;

            self.cycles = 7;
        }
    }

    pub fn nmi(&mut self){
        if self.read_flag(StatusFlag::I){
            self.write(0x0100 + self.sp as u16, ((self.pc >> 8) & 0x00FF).try_into().unwrap());
            self.sp -= 1;
            self.write(0x0100 + self.sp as u16, (self.pc & 0x00FF).try_into().unwrap());
            self.sp -= 1;

            self.set_flag(StatusFlag::B, false);
            self.set_flag(StatusFlag::U, true);
            self.set_flag(StatusFlag::I, true);
            self.write(0x0100 + self.sp as u16, self.status);
            self.sp -= 1;

            self.addr_abs = 0xFFFA;
            let lo = self.read(self.addr_abs) as u16;
            let hi = self.read(self.addr_abs+1) as u16;
            self.pc = (hi << 8) | lo;

            self.cycles = 7;
        }
    }

    pub fn clock(&mut self){
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

    pub fn complete(&self) -> bool{
        return self.cycles == 0;
    }
}
