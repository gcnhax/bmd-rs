use byteorder::{ReadBytesExt, BE};
use crate::error::Error;
use std::io::{Read, Seek};

enum FileType {
    BMD,
    BDL,
}

struct Header {
    ty: FileType,
    len: u32,
    n_sections: u32,
}

impl Header {
    pub fn parse<R>(r: &mut R) -> Result<Header, Error>
    where
        R: Read + Seek,
    {
        let magic = {
            let mut x = [0u8; 8];
            r.read_exact(&mut x)?;
            x
        };

        let ty = match magic.as_ref() {
            b"J3D2bmd3" => FileType::BMD,
            b"J3D2bdl4" => FileType::BDL,
            _ => return Err(Error::InvalidMagic),
        };

        let len = r.read_u32::<BE>()?;
        let n_sections = r.read_u32::<BE>()?;

        Ok(Header {
            ty,
            len,
            n_sections,
        })
    }
}
