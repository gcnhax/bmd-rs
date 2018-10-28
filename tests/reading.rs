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
    // println!("inf1: {:?}", inf1);

    let vtx1 = bmd::Vtx1::parse(&mut file).unwrap();
    // println!("vtx1: {:#?}", vtx1);

    let evp1 = bmd::Evp1::parse(&mut file).unwrap();
    let drw1 = bmd::Drw1::parse(&mut file).unwrap();
    let jnt1 = bmd::Jnt1::parse(&mut file).unwrap();

    let shp1 = bmd::Shp1::parse(&mut file).unwrap();
}
