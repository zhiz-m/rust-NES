pub trait Mapper {
    fn cpu_map_read(&self, addr: u16, mapped_addr: &mut u32) -> bool;
    fn cpu_map_write(&self, addr: u16, mapped_addr: &mut u32) -> bool;

    fn ppu_map_read(&self, addr: u16, mapped_addr: &mut u32) -> bool;
    fn ppu_map_write(&self, addr: u16, mapped_addr: &mut u32) -> bool;
}