extern crate bmd;

use std::fs::File;
use std::io::{self, Seek, SeekFrom};

pub trait SeekExt: Seek {
    fn whereami(&mut self) -> io::Result<u64> {
        self.seek(SeekFrom::Current(0))
    }
}

impl<S: Seek + ?Sized> SeekExt for S {}

#[test]
fn smoketest() {
    let mut file = File::open("data/map.bmd").unwrap();

    let hdr = bmd::Header::parse(&mut file).unwrap();
    println!("header: {:?}", hdr);

    let inf1 = bmd::Inf1::parse(&mut file).unwrap();
    println!("inf1: <elided>");

    let vtx1 = bmd::Vtx1::parse(&mut file).unwrap();
    println!("vtx1: {:?}", vtx1);
}