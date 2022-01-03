use crate::mappers::mapper::Mapper;

pub struct Mapper000 {
    num_prg_banks: u8,
    num_chr_banks: u8,
}

impl Mapper000 {
    pub fn new(num_prg_banks: u8, num_chr_banks: u8) -> Mapper000 {
        return Mapper000 {
            num_prg_banks,
            num_chr_banks,
        }
    }
}

impl Mapper for Mapper000 {
    fn cpu_map_read(&self, addr: u16) -> Option<u32> {
        if addr >= 0x8000 && addr <= 0xFFFF {
            if self.num_prg_banks > 1 {
                return Some((addr & 0x7FFF) as u32);
            } else {
                return Some((addr & 0x3FFF) as u32);
            }
        }

        return None;
    }

    fn cpu_map_write(&self, addr: u16) -> Option<u32> {
        return self.cpu_map_read(addr);
    }

    fn ppu_map_read(&self, addr: u16) -> Option<u32> {
        if addr >= 0x0000 && addr <= 0x1FFF {
            return Some(addr as u32);
        }

        return None;
    }

    fn ppu_map_write(&self, addr: u16) -> Option<u32> {
        if addr >= 0x0000 && addr <= 0x1FFF && self.num_chr_banks == 0 {
            return Some(addr as u32);
        }

        return None;
    }

    fn reset(&mut self) { }
}