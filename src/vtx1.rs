use byteorder::{ReadBytesExt, BE};
use crate::error::Error;
use crate::util::SeekExt;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug, PartialEq, Eq, FromPrimitive)]
pub enum VertexArrayType {
    PositionMatrixIndex = 0,
    Tex0MatrixIndex = 1,
    Tex1MatrixIndex = 2,
    Tex2MatrixIndex = 3,
    Tex3MatrixIndex = 4,
    Tex4MatrixIndex = 5,
    Tex5MatrixIndex = 6,
    Tex6MatrixIndex = 7,
    Tex7MatrixIndex = 8,
    Position = 9,
    Normal = 10,
    Color0 = 11,
    Color1 = 12,
    Tex0 = 13,
    Tex1 = 14,
    Tex2 = 15,
    Tex3 = 16,
    Tex4 = 17,
    Tex5 = 18,
    Tex6 = 19,
    Tex7 = 20,
    PositionMatrixArray = 21,
    NormalMatrixArray = 22,
    TextureMatrixArray = 23,
    LitMatrixArray = 24,
    NormalBinormalTangent = 25,
    NullAttr = 0xff,
}

#[derive(Debug, FromPrimitive)]
pub enum VertexScalarDataType {
    Unsigned8 = 0x0,
    Signed8 = 0x1,
    Unsigned16 = 0x2,
    Signed16 = 0x3,
    Float32 = 0x4,
}

#[derive(Debug, FromPrimitive)]
pub enum VertexColorDataType {
    RGB565 = 0x0,
    RGB8 = 0x1,
    RGBX8 = 0x2,
    RGBA4 = 0x3,
    RGBA6 = 0x4,
    RGBA8 = 0x5,
}

#[derive(Debug)]
pub enum VertexDataType {
    Scalar(VertexScalarDataType),
    Color(VertexColorDataType),
}

#[derive(Debug)]
struct VertexFormat {
    vertex_ty: VertexArrayType,
    data_ty: VertexDataType,
}

#[derive(Debug)]
pub struct Vtx1 {}

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
        let mut vertex_data_offsets = [0u32; 13];
        r.read_u32_into::<BE>(&mut vertex_data_offsets)?;

        r.seek(SeekFrom::Start(
            section_begin_offset + vertex_formats_offset as u64,
        ))?;

        loop {
            let vertex_ty = VertexArrayType::from_u32(r.read_u32::<BE>()?).unwrap();
            if vertex_ty == VertexArrayType::NullAttr {
                break;
            }

            let component_count = r.read_u32::<BE>()?;
            // let data_ty = VertexDataType::from_u32(r.read_u32::<BE>()?).unwrap();

            let data_ty = match vertex_ty {
                VertexArrayType::Position
                | VertexArrayType::Normal
                | VertexArrayType::NormalBinormalTangent
                | VertexArrayType::Tex0MatrixIndex
                | VertexArrayType::Tex1MatrixIndex
                | VertexArrayType::Tex2MatrixIndex
                | VertexArrayType::Tex3MatrixIndex
                | VertexArrayType::Tex4MatrixIndex
                | VertexArrayType::Tex5MatrixIndex
                | VertexArrayType::Tex6MatrixIndex
                | VertexArrayType::Tex7MatrixIndex
                | VertexArrayType::PositionMatrixArray
                | VertexArrayType::PositionMatrixIndex
                | VertexArrayType::NormalMatrixArray
                | VertexArrayType::TextureMatrixArray
                | VertexArrayType::LitMatrixArray => VertexDataType::Scalar(
                    VertexScalarDataType::from_u32(r.read_u32::<BE>()?).unwrap(),
                ),
                VertexArrayType::Color0
                | VertexArrayType::Color1
                | VertexArrayType::Tex0
                | VertexArrayType::Tex1
                | VertexArrayType::Tex2
                | VertexArrayType::Tex3
                | VertexArrayType::Tex4
                | VertexArrayType::Tex5
                | VertexArrayType::Tex6
                | VertexArrayType::Tex7 => VertexDataType::Color(
                    VertexColorDataType::from_u32(r.read_u32::<BE>()?).unwrap(),
                ),
                VertexArrayType::NullAttr => unreachable!(),
            };

            let fixpoint_mantissa_bits = r.read_u8()?;

            let vertex_format = VertexFormat { vertex_ty, data_ty };

            println!("{:#?}", vertex_format);

            // skip padding
            r.seek(SeekFrom::Current(3))?;
        }

        Ok(Vtx1 {})
    }
}
