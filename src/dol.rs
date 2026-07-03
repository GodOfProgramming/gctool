use byteorder::ByteOrder;

use crate::util::{self, bytes};

pub const TEXT_SECTION_COUNT: usize = 7;
pub const DATA_SECTION_COUNT: usize = 11;

#[derive(Default)]
pub struct Dol {
    data: Vec<u8>,
    sections: [Section; TEXT_SECTION_COUNT + DATA_SECTION_COUNT],

    /// BSS segment address
    bss_addr: u32,

    /// BSS segment size
    bss_size: u32,

    /// Entry point address
    ep_addr: u32,
}

impl Dol {
    pub fn new(data: Vec<u8>) -> anyhow::Result<Self> {
        let mut sections = [Section::default(); TEXT_SECTION_COUNT + DATA_SECTION_COUNT];

        for (idx, section) in sections.iter_mut().enumerate() {
            let offset = bytes::read_u32(&data, 0x00 + idx * 4)?;
            let address = bytes::read_u32(&data, 0x48 + idx * 4)?;
            let size = bytes::read_u32(&data, 0x90 + idx * 4)?;

            *section = Section {
                offset,
                address,
                size,
            };
        }

        let bss_addr = bytes::read_u32(&data, 0xD8)?;
        let bss_size = bytes::read_u32(&data, 0xDC)?;
        let ep_addr = bytes::read_u32(&data, 0xE0)?;

        Ok(Self {
            data,
            sections,
            bss_addr,
            bss_size,
            ep_addr,
        })
    }

    pub fn convert_address_to_offset(&self, addr: u32) -> Option<u32> {
        self.sections.iter().find_map(|s| {
            s.contains_address(addr)
                .then(|| addr - s.address + s.offset)
        })
    }

    pub fn convert_offset_to_address(&self, offset: u32) -> Option<u32> {
        self.sections.iter().find_map(|s| {
            s.contains_offset(offset)
                .then(|| offset - s.offset + s.address)
        })
    }

    pub fn convert_offset_to_section_index(&self, offset: u32) -> Option<usize> {
        self.sections
            .iter()
            .enumerate()
            .find_map(|(i, s)| s.contains_offset(offset).then_some(i))
    }

    pub fn read_data(&self, addr: u32) {}

    pub fn write_data(&mut self, addr: u32) {}

    pub fn save_changes(&mut self) -> anyhow::Result<()> {
        for (idx, section) in self.sections.iter().enumerate() {
            bytes::write_u32(&mut self.data, 0x00 + idx * 4, section.offset)?;
            bytes::write_u32(&mut self.data, 0x48 + idx * 4, section.offset)?;
            bytes::write_u32(&mut self.data, 0x90 + idx * 4, section.offset)?;
        }

        bytes::write_u32(&mut self.data, 0xD8, self.bss_addr)?;
        bytes::write_u32(&mut self.data, 0xDC, self.bss_size)?;
        bytes::write_u32(&mut self.data, 0xE0, self.ep_addr)?;

        Ok(())
    }
}

#[derive(Default, Clone, Copy)]
pub struct Section {
    offset: u32,
    address: u32,
    size: u32,
}

impl Section {
    pub fn contains_address(&self, addr: u32) -> bool {
        self.address <= addr && addr < self.address + self.size
    }

    pub fn contains_offset(&self, offset: u32) -> bool {
        self.offset <= offset && offset < self.offset + self.size
    }
}
