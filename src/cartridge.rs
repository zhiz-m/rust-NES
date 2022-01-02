use std::string::String;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::mem::transmute;

pub struct Cartridge {
    program_mem: Vec<u8>,
    character_mem: Vec<u8>,

    mapper_id: u8,
    num_prg_banks: u8,
    num_chr_banks: u8,
}

impl Cartridge {
    pub fn new(rom_path: &String) -> Cartridge {
        let mut f = File::open(&rom_path)
            .expect("ROM file should exist");

        let header: INesHeader = {
            let mut bytes = [0x00; 16];

            f.read_exact(&mut bytes[..])?;

            let mut name = [0x00; 4];
            name.clone_from_slice(&bytes[..4]);

            let prg_rom_chunks = bytes[4];
            let chr_rom_chunks = bytes[5];
            let mapper1 = bytes[6];
            let mapper2 = bytes[7];
            let prg_ram_size = bytes[8];
            let tv_system1 = bytes[9];
            let tv_system2 = bytes[10];

            INesHeader {
                name,
                prg_rom_chunks,
                chr_rom_chunks,
                mapper1,
                mapper2,
                prg_ram_size,
                tv_system1,
                tv_system2,
            }
        };

        if header.mapper1 & 0x04 {
            // Ignore the training data if it exists
            f.seek(SeekFrom::Current(512));
        }

        let mapper_id =

        return Cartridge {
            program_mem: vec![0x00, 16384],
            character_mem: vec![0x00, 8192],
        }
    }

    // Read and write functions return booleans which state whether
    // the cartridge's mapper has decided to take ownership of a referenced address

    pub fn cpu_read(addr: u16, data: &mut u8) -> bool {
        return true;
    }

    pub fn cpu_write(addr: u16, data: u8) -> bool {
        return true;
    }

    pub fn ppu_read(addr: u16, data: &mut u8) -> bool {
        return true;
    }

    pub fn ppu_write(addr: u16, data: u8) -> bool {
        return true;
    }

    pub fn reset() {

    }
}

struct INesHeader {
    name: [u8; 4],
    prg_rom_chunks: u8,
    chr_rom_chunks: u8,
    mapper1: u8,
    mapper2: u8,
    prg_ram_size: u8,
    tv_system1: u8,
    tv_system2: u8,
}
