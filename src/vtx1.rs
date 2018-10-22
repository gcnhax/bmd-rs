use byteorder::{ReadBytesExt, BE};
use std::io::{Read, Seek, SeekFrom};
use crate::error::Error;
use crate::util::SeekExt;

#[derive(Debug)]
pub struct Vtx1 {

}

impl Vtx1 {
    pub fn parse<R>(r: &mut R) -> Result<Vtx1, Error>
    where
        R: Read + Seek,
    {
        let section_begin_offset = r.whereami()?;

        // assert that we're starting in the right place
        if &{
            let mut x = [0u8; 4];
            r.read_exact(&mut x)?;
            x
        } != b"VTX1"
        {
            return Err(Error::InvalidMagic);
        }

        let section_size = r.read_u32::<BE>()?;
        let vertex_formats_offset = r.read_u32::<BE>()?;
        let data_pointer_table_offset = 0x0c;

        Ok(Vtx1 {})
    }
}
