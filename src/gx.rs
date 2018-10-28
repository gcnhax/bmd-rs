use num_derive::FromPrimitive;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FromPrimitive)]
pub enum VertexArrayType {
    PositionMatrixIndex = 0,
    Tex0MatrixIndex = 1,
    Tex1MatrixIndex = 2,
    Tex2MatrixIndex = 3,
    Tex3MatrixIndex = 4,
    Tex4MatrixIndex = 5,
    Tex5MatrixIndex = 6,
    Tex6MatrixIndex = 7,
    Tex7MatrixIndex = 8,
    Position = 9,
    Normal = 10,
    Color0 = 11,
    Color1 = 12,
    Tex0 = 13,
    Tex1 = 14,
    Tex2 = 15,
    Tex3 = 16,
    Tex4 = 17,
    Tex5 = 18,
    Tex6 = 19,
    Tex7 = 20,
    PositionMatrixArray = 21,
    NormalMatrixArray = 22,
    TextureMatrixArray = 23,
    LitMatrixArray = 24,
    NormalBinormalTangent = 25,
    NullAttr = 0xff,
}

#[derive(Debug, Copy, Clone, FromPrimitive)]
pub enum VertexScalarDataType {
    Unsigned8 = 0x0,
    Signed8 = 0x1,
    Unsigned16 = 0x2,
    Signed16 = 0x3,
    Float32 = 0x4,
}

#[derive(Debug, Copy, Clone, FromPrimitive)]
pub enum VertexColorDataType {
    RGB565 = 0x0,
    RGB8 = 0x1,
    RGBX8 = 0x2,
    RGBA4 = 0x3,
    RGBA6 = 0x4,
    RGBA8 = 0x5,
}

#[derive(Debug, Copy, Clone)]
pub enum VertexDataType {
    Scalar(VertexScalarDataType),
    Color(VertexColorDataType),
}
