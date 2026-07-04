use crate::{GcError, util::bytes};
use derive_more::{Deref, DerefMut};
use gc_gcm::{DolFile, DolHeader};

#[derive(Deref, DerefMut)]
pub struct Dol(DolFile);

impl Dol {
    pub fn new(data: impl AsRef<[u8]>) -> anyhow::Result<Self> {
        let mut cursor = std::io::Cursor::new(data);
        let dol = DolFile::from_reader(&mut cursor).map_err(|e| match e {
            gc_gcm::GcmError::ParseError(error) => GcError::generic(error),
            gc_gcm::GcmError::IoError(error) => GcError::generic(error),
        })?;
        Ok(Self(dol))
    }

    pub fn convert_address_to_offset(&self, addr: u32) -> Option<u32> {
        self.section_iter().find_map(|s| {
            s.contains_address(addr)
                .then(|| addr - s.address + s.offset)
        })
    }

    pub fn convert_offset_to_address(&self, offset: u32) -> Option<u32> {
        self.section_iter().find_map(|s| {
            s.contains_offset(offset)
                .then(|| offset - s.offset + s.address)
        })
    }

    pub fn convert_offset_to_section_index(&self, offset: u32) -> Option<usize> {
        self.section_iter()
            .enumerate()
            .find_map(|(i, s)| s.contains_offset(offset).then_some(i))
    }

    pub fn read_data(&self, addr: u32) {}

    pub fn write_data(&mut self, addr: u32) {}

    pub fn save_changes(&mut self) -> anyhow::Result<Vec<u8>> {
        let mut out = [0; size_of::<DolHeader>()];

        for (idx, section) in self.section_iter().enumerate() {
            bytes::write_u32(&mut out, 0x00 + idx * 4, section.offset)?;
            bytes::write_u32(&mut out, 0x48 + idx * 4, section.offset)?;
            bytes::write_u32(&mut out, 0x90 + idx * 4, section.offset)?;
        }

        bytes::write_u32(&mut out, 0xD8, self.header.bss_address)?;
        bytes::write_u32(&mut out, 0xDC, self.header.bss_address)?;
        bytes::write_u32(&mut out, 0xE0, self.header.entrypoint)?;

        let out = Vec::from_iter(out.into_iter().chain(self.raw_data.iter().cloned()));

        Ok(out)
    }

    pub fn section_iter(&self) -> impl Iterator<Item = Section> {
        self.header
            .section_offsets
            .iter()
            .cloned()
            .zip(self.header.section_addresses.iter().cloned())
            .zip(self.header.section_lengths.iter().cloned())
            .map(|((offset, address), length)| Section {
                offset,
                address,
                length,
            })
    }
}

#[derive(Default, Clone, Copy)]
pub struct Section {
    offset: u32,
    address: u32,
    length: u32,
}

impl Section {
    pub fn contains_address(&self, addr: u32) -> bool {
        self.address <= addr && addr < self.address + self.length
    }

    pub fn contains_offset(&self, offset: u32) -> bool {
        self.offset <= offset && offset < self.offset + self.length
    }
}
