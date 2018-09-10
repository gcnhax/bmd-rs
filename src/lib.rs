#![feature(try_from)]
#![allow(unreachable_code)]

mod error;
mod header;
mod inf1;
mod util;

pub use crate::header::{Header, FileType};
pub use crate::inf1::Inf1;