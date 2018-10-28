use byteorder::{ReadBytesExt, BE};
use std::io::{Read, Seek, SeekFrom};

use crate::error::Error;
use crate::util::SeekExt;

pub struct Shp1 {}

#[derive(Debug)]
pub struct Offsets {
    batch_table: u32,
    index_table: u32,
    attrib_table: u32,
    matrix_table: u32,
    primitive_data: u32,
    matrix_data: u32,
    packet_table: u32,
}

impl Offsets {
    fn parse<R>(r: &mut R) -> Result<Offsets, Error>
    where R: Read + Seek {
        let batch_table = r.read_u32::<BE>()?;
        let index_table = r.read_u32::<BE>()?;
        r.seek(SeekFrom::Current(0x4))?;
        let attrib_table = r.read_u32::<BE>()?;
        let matrix_table = r.read_u32::<BE>()?;
        let primitive_data = r.read_u32::<BE>()?;
        let matrix_data = r.read_u32::<BE>()?;
        let packet_table = r.read_u32::<BE>()?;

        Ok(Offsets {
            batch_table,
            index_table,
            attrib_table,
            matrix_table,
            primitive_data,
            matrix_data,
            packet_table,
        })
    }
}

impl Shp1 {
    pub fn parse<R>(r: &mut R) -> Result<Shp1, Error>
    where
        R: Read + Seek,
    {
        let section_begin_offset = r.whereami()?;

        // assert that we're starting in the right place
        if &{
            let mut x = [0u8; 4];
            r.read_exact(&mut x)?;
            x
        } != b"SHP1"
        {
            return Err(Error::InvalidMagic);
        }

        let section_size = r.read_u32::<BE>()?;
        let batch_count = r.read_u16::<BE>()?;
        r.seek(SeekFrom::Current(0x2))?;
        let offsets = Offsets::parse(r)?;

        println!("offsets: {:?}", offsets);

        r.seek(SeekFrom::Start(section_begin_offset + section_size as u64))?;

        Ok(Shp1 {})
    }
}
