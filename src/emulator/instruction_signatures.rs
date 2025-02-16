#[derive(Debug)]
pub struct DestinationSource1Source2 {
    pub rd: u8,
    pub rs1: u8,
    pub rs2: u8,
}

#[derive(Debug)]
pub struct Source1Source2Immediate {
    pub rs1: u8,
    pub rs2: u8,
    pub imm: i16,
}

#[derive(Debug)]
pub struct DestinationSource1Immediate {
    pub rd: u8,
    pub rs1: u8,
    pub imm: i16,
}

#[derive(Debug)]
pub struct DestinationImmediate {
    pub rd: u8,
    pub imm: i16,
}

#[derive(Debug)]
pub struct PseudoLiSignature {
    rd: u8,
    imm: i32,
}
