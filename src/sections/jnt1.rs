use byteorder::{ReadBytesExt, BE};
use std::io::{Read, Seek, SeekFrom};

use crate::error::Error;
use crate::util::SeekExt;

pub struct Jnt1 {}

impl Jnt1 {
    pub fn parse<R>(r: &mut R) -> Result<Jnt1, Error>
    where
        R: Read + Seek,
    {
        let section_begin_offset = r.whereami()?;

        // assert that we're starting in the right place
        if &{
            let mut x = [0u8; 4];
            r.read_exact(&mut x)?;
            x
        } != b"JNT1"
        {
            return Err(Error::InvalidMagic);
        }

        let section_size = r.read_u32::<BE>()?;

        r.seek(SeekFrom::Start(section_begin_offset + section_size as u64))?;

        Ok(Jnt1 {})
    }
}
