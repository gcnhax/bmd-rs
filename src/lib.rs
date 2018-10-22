#![feature(try_from)]
#![allow(unreachable_code)]

mod error;
mod header;
mod inf1;
mod vtx1;
mod util;

pub use crate::header::{Header, FileType};
pub use crate::inf1::Inf1;
pub use crate::vtx1::Vtx1;