extern crate bmd;

use std::fs::File;

#[test]
fn smoketest() {
    let mut file = File::open("data/map.bmd").unwrap();

    let hdr = bmd::Header::parse(&mut file).unwrap();
    println!("header: {:?}", hdr);

    let inf1 = bmd::Inf1::parse(&mut file).unwrap();
    println!("inf1: {:?}", inf1);
}