#![feature(try_from)]
#![allow(unreachable_code)]

mod error;
mod gx;
mod header;
mod sections;
mod util;

pub use crate::header::{FileType, Header};
pub use crate::sections::inf1::Inf1;
pub use crate::sections::vtx1::Vtx1;
pub use crate::sections::evp1::Evp1;
pub use crate::sections::drw1::Drw1;
pub use crate::sections::jnt1::Jnt1;
pub use crate::sections::shp1::Shp1;