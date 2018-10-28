use byteorder::{ReadBytesExt, BE};
use num_traits::FromPrimitive;
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom};

use crate::error::Error;
use crate::gx::{VertexArrayType, VertexColorDataType, VertexDataType, VertexScalarDataType};
use crate::util::SeekExt;

impl VertexArrayType {
    /// Data tables are stored in a fixed order based on their type; this function returns
    /// the index of the data table for a given vertex array type.
    fn data_index(&self) -> Option<usize> {
        match self {
            VertexArrayType::Position => Some(0),
            VertexArrayType::Normal => Some(1),
            VertexArrayType::NormalBinormalTangent => Some(2),
            VertexArrayType::Color0 => Some(3),
            VertexArrayType::Color1 => Some(4),
            VertexArrayType::Tex0 => Some(5),
            VertexArrayType::Tex1 => Some(6),
            VertexArrayType::Tex2 => Some(7),
            VertexArrayType::Tex3 => Some(8),
            VertexArrayType::Tex4 => Some(9),
            VertexArrayType::Tex5 => Some(10),
            VertexArrayType::Tex6 => Some(11),
            VertexArrayType::Tex7 => Some(12),
            _ => None,
        }
    }

    fn parse_data_type(&self, dtype: u32) -> VertexDataType {
        // TODO: Result<T, E>.
        match self {
            VertexArrayType::Position
            | VertexArrayType::Normal
            | VertexArrayType::NormalBinormalTangent => {
                VertexDataType::Scalar(VertexScalarDataType::from_u32(dtype).unwrap())
            }
            VertexArrayType::Color0
            | VertexArrayType::Color1
            | VertexArrayType::Tex0
            | VertexArrayType::Tex1
            | VertexArrayType::Tex2
            | VertexArrayType::Tex3
            | VertexArrayType::Tex4
            | VertexArrayType::Tex5
            | VertexArrayType::Tex6
            | VertexArrayType::Tex7 => {
                VertexDataType::Color(VertexColorDataType::from_u32(dtype).unwrap())
            }
            // Everything else shouldn't get serialized into BMD files
            _ => panic!("TODO: error here!"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct VertexFormat {
    pub ty: VertexArrayType,
    pub data_ty: VertexDataType,
    pub component_count: u32,
    pub fixpoint_mantissa_bits: u8,
}

#[derive(Debug)]
pub struct VertexArray {
    pub format: VertexFormat,
    pub raw_data: Vec<u8>,
}

pub enum VertexData {
    Unsigned8(Vec<u8>),
    Signed8(Vec<i8>),
    Unsigned16(Vec<u16>),
    Signed16(Vec<u16>),
    Float32(Vec<f32>),

    RGB565(Vec<u8>),
    RGB8(Vec<u8>),
    RGBX8(Vec<u8>),
    RGBA4(Vec<u8>),
    RGBA6(Vec<u8>),
    RGBA8(Vec<u8>),
}

/// Vertex data of the model.
#[derive(Debug)]
pub struct Vtx1 {
    pub arrays: HashMap<VertexArrayType, VertexArray>,
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
        let mut vertex_data_offsets = [0u32; 13];
        r.read_u32_into::<BE>(&mut vertex_data_offsets)?;

        r.seek(SeekFrom::Start(
            section_begin_offset + vertex_formats_offset as u64,
        ))?;

        let mut vertex_formats = Vec::new();
        loop {
            let ty = VertexArrayType::from_u32(r.read_u32::<BE>()?).unwrap();
            if ty == VertexArrayType::NullAttr {
                break;
            }

            let component_count = r.read_u32::<BE>()?;
            let data_ty = ty.parse_data_type(r.read_u32::<BE>()?);

            let fixpoint_mantissa_bits = r.read_u8()?;

            let vertex_format = VertexFormat {
                ty,
                data_ty,
                component_count,
                fixpoint_mantissa_bits,
            };

            // skip padding
            r.seek(SeekFrom::Current(3))?;

            vertex_formats.push(vertex_format);
        }

        let mut arrays: HashMap<VertexArrayType, VertexArray> = HashMap::new();
        for vtx_format in vertex_formats {
            // find the vertex list offset
            let offset = vertex_data_offsets[vtx_format.ty.data_index().unwrap()];

            r.seek(SeekFrom::Start(section_begin_offset + offset as u64))?;
            let mut data: Vec<u8> = Vec::new();

            let data_len = Self::vertex_data_length(
                &vertex_data_offsets,
                vtx_format.ty.data_index().unwrap(),
                section_size,
            );

            r.take(data_len as u64).read_to_end(&mut data)?;

            let array = VertexArray {
                format: vtx_format,
                raw_data: data,
            };

            arrays.insert(vtx_format.ty, array);
        }

        r.seek(SeekFrom::Start(section_begin_offset + section_size as u64))?;

        Ok(Vtx1 { arrays })
    }

    fn vertex_data_length(offsets: &[u32], current: usize, section_size: u32) -> u32 {
        let next = Self::next_data_offset(offsets, current).unwrap_or(section_size);
        next - offsets[current]
    }

    fn next_data_offset(offsets: &[u32], k: usize) -> Option<u32> {
        offsets
            .iter()
            .cloned()
            .skip(k + 1)
            .skip_while(|&x| x == 0)
            .next()
    }
}
