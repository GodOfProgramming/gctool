use crate::GcError;
use byteorder::{BigEndian, ByteOrder};
use thiserror::Error;

macro_rules! impl_write {
    ($($datatype: ty),+) => {
        paste::paste! {
          $(
            pub fn [<write_ $datatype>](mut data: impl AsMut<[u8]>, offset: usize, value: $datatype) -> anyhow::Result<()> {
                let range = data_range_mut::<$datatype>(data.as_mut(), offset)?;
                BigEndian::[<write_ $datatype>](range, value);
                Ok(())
            }
          )*
        }
    };
}

macro_rules! impl_read {
    ($($datatype: ty),+) => {
        paste::paste! {
          $(
            pub fn [<read_ $datatype>](data: impl AsRef<[u8]>, offset: usize) -> anyhow::Result<$datatype> {
                let range = data_range::<$datatype>(data.as_ref(), offset)?;
                Ok(BigEndian::[<read_ $datatype>](range))
            }
          )*
        }
    };
}

pub fn read_u8(data: impl AsRef<[u8]>, offset: usize) -> anyhow::Result<u8> {
    let byte = data
        .as_ref()
        .get(offset)
        .ok_or(GcError::OutOfRange(offset, offset))?;
    Ok(*byte)
}

pub fn read_i8(data: impl AsRef<[u8]>, offset: usize) -> anyhow::Result<i8> {
    let byte = data
        .as_ref()
        .get(offset)
        .ok_or(GcError::OutOfRange(offset, offset))?;
    Ok(i8::from_be_bytes([*byte]))
}

impl_read!(u16, u32, u64, u128, i16, i32, i64, i128);

pub fn write_u8(mut data: impl AsMut<[u8]>, offset: usize, value: u8) -> anyhow::Result<()> {
    let byte = data
        .as_mut()
        .get_mut(offset)
        .ok_or(GcError::OutOfRange(offset, offset))?;
    *byte = value;
    Ok(())
}

pub fn write_i8(mut data: impl AsMut<[u8]>, offset: usize, value: i8) -> anyhow::Result<()> {
    let byte = data
        .as_mut()
        .get_mut(offset)
        .ok_or(GcError::OutOfRange(offset, offset))?;
    *byte = u8::from_be_bytes(value.to_be_bytes());
    Ok(())
}

impl_write!(u16, u32, u64, u128, i16, i32, i64, i128);

pub fn data_range<T>(data: &[u8], offset: usize) -> anyhow::Result<&[u8]> {
    let range = data
        .get(offset..offset + size_of::<T>())
        .ok_or(GcError::OutOfRange(offset, offset + size_of::<T>()))?;
    Ok(range)
}

pub fn data_range_mut<T>(data: &mut [u8], offset: usize) -> anyhow::Result<&mut [u8]> {
    let range = data
        .get_mut(offset..offset + size_of::<T>())
        .ok_or(GcError::OutOfRange(offset, offset + size_of::<T>()))?;
    Ok(range)
}
