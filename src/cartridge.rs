use std::string::String;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use crate::mappers::mapper::Mapper;
use crate::mappers::mapper_factory::create_mapper;

pub struct Cartridge {
    program_mem: Vec<u8>,
    character_mem: Vec<u8>,
    mapper: Box<dyn Mapper>,
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

            if name != [0x4E, 0x45, 0x53, 0x1A] {
                panic!("The file supplied is not in the iNES format");
            }

            let prg_rom_chunks = bytes[4];
            let chr_rom_chunks = bytes[5];
            let flags6 = bytes[6];
            let flags7 = bytes[7];
            let prg_ram_size = bytes[8];
            let tv_system1 = bytes[9];
            let tv_system2 = bytes[10];

            INesHeader {
                prg_rom_chunks,
                chr_rom_chunks,
                flags6,
                flags7,
                prg_ram_size,
                tv_system1,
                tv_system2,
            }
        };

        if header.mapper1 & 0x04 {
            // Ignore the training data if it exists
            f.seek(SeekFrom::Current(512));
        }

        let mapper_id = ((header.flags7 >> 4) << 4) | (header.flags6 >> 4);
        let mapper = create_mapper(mapper_id, header.prg_rom_chunks, header.chr_rom_chunks);

        return Cartridge {
            program_mem: vec![0x00, 16384],
            character_mem: vec![0x00, 8192],
        }
    }

    // Read and write functions return booleans which state whether
    // the cartridge's mapper has decided to take ownership of a referenced address

    pub fn cpu_read(&self, addr: u16, data: &mut u8) -> bool {
        return true;
    }

    pub fn cpu_write(&mut self, addr: u16, data: u8) -> bool {
        return true;
    }

    pub fn ppu_read(&self, addr: u16, data: &mut u8) -> bool {
        return true;
    }

    pub fn ppu_write(&mut self, addr: u16, data: u8) -> bool {
        return true;
    }

    pub fn reset(&mut self) {

    }
}

struct INesHeader {
    prg_rom_chunks: u8,
    chr_rom_chunks: u8,
    flags6: u8,
    flags7: u8,
    prg_ram_size: u8,
    tv_system1: u8,
    tv_system2: u8,
}
