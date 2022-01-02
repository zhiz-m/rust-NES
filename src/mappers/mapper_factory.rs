use crate::mappers::mapper::Mapper;
use crate::mappers::mapper_000::Mapper000;

pub fn create_mapper(mapper_id: u8, num_prg_banks: u8, num_chr_banks: u8) -> Box<dyn Mapper> {
    if mapper_id == 0 {
        return Box::new(Mapper000::new(num_prg_banks, num_chr_banks));
    } else {
        panic!("Unknown mapper of id {}", mapper_id);
    }
}