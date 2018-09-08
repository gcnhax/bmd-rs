use byteorder::{ReadBytesExt, BE};
use crate::error::Error;
use std::io::{Read, Seek, SeekFrom};

enum GraphNodeType {
    Joint,
    Material,
    Shape,
    Terminate
}

struct GraphNode {
    ty: GraphNodeType,
    children: Vec<Box<GraphNode>>,
    index: Option<u16>,
}

struct Inf1 {
    scene_graph: GraphNode,
}

impl Inf1 {
    pub fn parse<R>(r: &mut R) -> Result<Inf1, Error>
    where
        R: Read + Seek,
    {
        // assert that we're starting in the right place
        if &{let mut x = [0u8;4]; r.read_exact(&mut x)?; x} != b"INF1" {
            return Err(Error::InvalidMagic);
        }

        r.seek(SeekFrom::Current(0x10))?; // seek to start of data

        let vert_count = r.read_u32::<BE>()?;

        Ok(Inf1 {
            scene_graph: GraphNode { ty: GraphNodeType::Terminate, children: vec![], index: None }
        })
    }
}