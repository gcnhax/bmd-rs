use byteorder::{ReadBytesExt, BE};
use indextree::{Arena, NodeId};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;
use std::io::{Read, Seek, SeekFrom};

use crate::error::Error;
use crate::util::SeekExt;

#[derive(Debug, FromPrimitive, ToPrimitive)]
enum GraphStreamTag {
    End = 0x00,
    Open = 0x01,
    Close = 0x02,
    Joint = 0x10,
    Material = 0x11,
    Shape = 0x12,
}

#[derive(Debug)]
pub enum GraphNode {
    Root,
    Joint(u16),
    Material(u16),
    Shape(u16),
}

#[derive(Debug)]
struct SceneGraph {
    arena: Arena<GraphNode>,
    root_id: NodeId,
}

#[derive(Debug)]
pub struct Inf1 {
    scene_graph: SceneGraph,
}

impl Inf1 {
    pub fn parse<R>(r: &mut R) -> Result<Inf1, Error>
    where
        R: Read + Seek,
    {
        let section_begin_offset = r.whereami()?;

        // assert that we're starting in the right place
        if &{
            let mut x = [0u8; 4];
            r.read_exact(&mut x)?;
            x
        } != b"INF1"
        {
            return Err(Error::InvalidMagic);
        }

        let section_len = r.read_u32::<BE>()? as u64;

        r.seek(SeekFrom::Current(0x8))?; // seek to start of data

        let _vert_count = r.read_u32::<BE>()?;
        let hierarchy_offset = r.read_u32::<BE>()?;

        r.seek(SeekFrom::Start(
            section_begin_offset + hierarchy_offset as u64,
        ))?;

        let mut arena = Arena::new();
        let root = arena.new_node(GraphNode::Root);
        let mut last_node: Option<NodeId> = None;
        let mut parent_stack: Vec<NodeId> = vec![root];
        loop {
            let tag =
                GraphStreamTag::from_u16(r.read_u16::<BE>()?).ok_or(Error::InvalidInfPacket)?;
            let index = r.read_u16::<BE>()?;

            match tag {
                GraphStreamTag::End => break,
                GraphStreamTag::Open => {
                    parent_stack.push(last_node.ok_or(Error::NodeHierarchyMismatch)?)
                }
                GraphStreamTag::Close => last_node = parent_stack.pop(),
                GraphStreamTag::Joint => {
                    last_node = Some(arena.new_node(GraphNode::Joint(index)));
                    parent_stack
                        .last()
                        .ok_or(Error::NodeHierarchyMismatch)?
                        .append(last_node.ok_or(Error::NodeHierarchyMismatch)?, &mut arena);
                }
                GraphStreamTag::Material => {
                    last_node = Some(arena.new_node(GraphNode::Material(index)));
                    parent_stack
                        .last()
                        .ok_or(Error::NodeHierarchyMismatch)?
                        .append(last_node.ok_or(Error::NodeHierarchyMismatch)?, &mut arena);
                }
                GraphStreamTag::Shape => {
                    last_node = Some(arena.new_node(GraphNode::Shape(index)));
                    parent_stack
                        .last()
                        .ok_or(Error::NodeHierarchyMismatch)?
                        .append(last_node.ok_or(Error::NodeHierarchyMismatch)?, &mut arena);
                }
            }
        }

        r.seek(SeekFrom::Start(section_begin_offset + section_len))?;

        let scene_graph = SceneGraph {
            arena,
            root_id: root,
        };

        Ok(Inf1 { scene_graph })
    }
}
