use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        IO(err: std::io::Error) {
            from()
        }

        InvalidMagic {
            description("Invalid header magic")
        }

        InvalidInfPacket {}
        NodeHierarchyMismatch {}
    }
}
