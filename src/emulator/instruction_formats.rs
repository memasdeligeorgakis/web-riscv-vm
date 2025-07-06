pub enum InstructionFormat {
    R,
    I,
    S,
    B,
    U,
    J,
    Unknown,
}

impl InstructionFormat {
    pub fn detect_format_from_opcode(opcode: u8) -> Self {
        match opcode {
            // R-format opcodes
            0x33 | 0x3B | 0x53 => InstructionFormat::R,

            // I-format opcodes
            0x03 | 0x13 | 0x1B | 0x67 | 0x73 => InstructionFormat::I,

            // S-format opcodes
            0x23 => InstructionFormat::S,

            // B-format opcodes
            0x63 => InstructionFormat::B,

            // U-format opcodes
            0x17 | 0x37 => InstructionFormat::U,

            // J-format opcodes
            0x6F => InstructionFormat::J,

            // Unknown opcode
            _ => InstructionFormat::Unknown,
        }
    }

    pub fn get_opcode_from_instruction(instruction: [u8; 4]) -> u8 {
        

        instruction[0] & 0x7F
    }
}

#[derive(Debug)]
pub struct InstructionFormatR {
    /// bits 25 to 31
    funct7: u8,

    /// bits 20 to 24
    rs2: u8,

    /// bits 15 to 19
    rs1: u8,

    /// bits 12 to 14
    funct3: u8,

    /// bits 7 to 11
    rd: u8,

    /// bits 0 to 6
    opcode: u8,
}

impl InstructionFormatR {
    pub fn new(instruction: u32) -> Self {
        Self {
            opcode: (instruction & 0b0111_1111) as u8,
            rd: ((instruction >> 7) & 0b0001_1111) as u8,
            funct3: ((instruction >> 12) & 0b0111) as u8,
            rs1: ((instruction >> 15) & 0b0001_1111) as u8,
            rs2: ((instruction >> 20) & 0b0001_1111) as u8,
            funct7: ((instruction >> 25) & 0b0111_1111) as u8,
        }
    }

    /// this is how the instructions are in elf file
    pub fn parse_instruction_from_bytes(bytes: &[u8]) -> Self {
        let instruction = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        

        Self::new(instruction)
    }
}

/// Instruction format I 32 bit layout
#[derive(Debug)]
pub struct InstructionFormatI {
    /// bits 20 to 31
    imm: u8,

    /// bits 15 to 19
    rs1: u8,

    /// bits 12 to 14
    funct3: u8,

    /// bits 7 to 11
    rd: u8,

    /// bits 0 to 6
    opcode: u8,
}

impl InstructionFormatI {
    pub fn new(instruction: u32) -> Self {
        // always shift right by the start index, then do bitwise AND to
        // collect bits
        Self {
            opcode: (instruction & 0b0111_1111) as u8,
            rd: ((instruction >> 7) & 0b0001_1111) as u8,
            funct3: ((instruction >> 12) & 0b0111) as u8,
            rs1: ((instruction >> 15) & 0b0001_1111) as u8,
            imm: ((instruction >> 20) & 0b1111_1111_1111) as u8,
        }
    }

    pub fn parse_instruction_from_bytes(bytes: &[u8]) -> Self {
        let instruction = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        

        Self::new(instruction)
    }
}

#[derive(Debug)]
pub struct InstructionFormatS {
    /// bits 25 to 31
    imm_5_to_11: u8,

    /// bits 20 to 24
    rs2: u8,

    /// bits 15 to 19
    rs1: u8,

    /// bits 12 to 14
    func3: u8,

    /// bits 7 to 11
    imm_0_to_4: u8,

    /// bits 0 to 6
    opcode: u8,
}

impl InstructionFormatS {
    /// Create a new S-type instruction by extracting the specific fields from the 32-bit instruction.
    pub fn new(instruction: u32) -> Self {
        Self {
            // imm[11:5] are bits 25-31 (7 bits)
            imm_5_to_11: ((instruction >> 25) & 0b0111_1111) as u8,
            // rs2 are bits 20-24 (5 bits)
            rs2: ((instruction >> 20) & 0b0001_1111) as u8,
            // rs1 are bits 15-19 (5 bits)
            rs1: ((instruction >> 15) & 0b0001_1111) as u8,
            // funct3 are bits 12-14 (3 bits)
            func3: ((instruction >> 12) & 0b0111) as u8,
            // imm[4:0] are bits 7-11 (5 bits)
            imm_0_to_4: ((instruction >> 7) & 0b0001_1111) as u8,
            // opcode is bits 0-6 (7 bits)
            opcode: (instruction & 0b0111_1111) as u8,
        }
    }

    /// Parses an S-format instruction out of a 4-byte little-endian slice.
    pub fn parse_instruction_from_bytes(bytes: &[u8]) -> Self {
        let instruction = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        Self::new(instruction)
    }
}

/// The only difference between the S and B formats is that the 12-bit immediate
/// field is used to encode branch offsets in multiples of 2 in the B format.
/// Instead of shifting all bits in the instruction-encoded immediate left by one
/// in hardware as is conventionally done, the middle bits (imm[10:1]) and sign
/// bit stay in fixed positions, while the lowest bit in S format (inst[7])
/// encodes a high-order bit in B format.
#[derive(Debug)]
pub struct InstructionFormatB {
    /// bit 31
    sign_imm_5_to_11: u8,

    /// bits 25 to 31
    imm_5_to_11: u8,

    /// bits 20 to 24
    rs2: u8,

    /// bits 15 to 19
    rs1: u8,

    /// bits 12 to 14
    func3: u8,

    /// bits 8 to 11
    imm_0_to_4: u8,

    /// bit 7
    sign_imm_0_to_4: u8,

    /// bits 0 to 6
    opcode: u8,
}

impl InstructionFormatB {
    /// Create a new B-type instruction
    pub fn new(instruction: u32) -> Self {
        Self {
            // Bit 31: imm[12] (1 bit)
            sign_imm_5_to_11: ((instruction >> 31) & 0b0001) as u8,
            // Bits 30 to 25: imm[10:5] (6 bits)
            imm_5_to_11: ((instruction >> 25) & 0b0011_1111) as u8,
            // Bits 20 to 24: rs2 (5 bits)
            rs2: ((instruction >> 20) & 0b0001_1111) as u8,
            // Bits 15 to 19: rs1 (5 bits)
            rs1: ((instruction >> 15) & 0b0001_1111) as u8,
            // Bits 12 to 14: funct3 (3 bits)
            func3: ((instruction >> 12) & 0b0111) as u8,
            // Bits 11 to 8: imm[4:1] (4 bits)
            imm_0_to_4: ((instruction >> 8) & 0b1111) as u8,
            // Bit 7: imm[11] (1 bit)
            sign_imm_0_to_4: ((instruction >> 7) & 0b0001) as u8,
            // Bits 0 to 6: opcode (7 bits)
            opcode: (instruction & 0b0111_1111) as u8,
        }
    }

    /// Parse a B-type instruction from a 4-byte little-endian slice.
    pub fn parse_instruction_from_bytes(bytes: &[u8]) -> Self {
        let instruction = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        Self::new(instruction)
    }
}

#[derive(Debug)]
pub struct InstructionFormatU {
    /// bits 12 to 31
    imm: u8,

    /// bits 7 to 11
    rd: u8,

    /// bits 0 to 6
    opcode: u8,
}

impl InstructionFormatU {
    pub fn new(instruction: u32) -> Self {
        Self {
            // Bits 12 to 31: immediate (20 bits)
            // Using a binary mask for 20 bits: 0b1111_1111_1111_1111_1111
            imm: ((instruction >> 12) & 0b1111_1111_1111_1111_1111) as u8,
            // Bits 7 to 11: rd (5 bits): mask 0b1_1111
            rd: ((instruction >> 7) & 0b0001_1111) as u8,
            // Bits 0 to 6: opcode (7 bits): mask 0b111_1111
            opcode: (instruction & 0b0111_1111) as u8,
        }
    }

    pub fn parse_instruction_from_bytes(bytes: &[u8]) -> Self {
        let instruction = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        Self::new(instruction)
    }
}

/// the only difference between the U and J formats is that the 20-bit immediate
///is shifted left by 12 bits to form U immediates and by 1 bit to form J
/// immediates. The location of instruction bits in the U and J format immediates
/// is chosen to maximize overlap with the other formats and with each other.
#[derive(Debug)]
pub struct InstructionFormatJ {
    /// bit 31
    sign_imm_21_30: u8,

    /// bits 21 to 30
    imm_21_30: u8,

    /// bit 20
    sign_imm_12_19: u8,

    /// bits 12 to 19
    imm_12_19: u8,

    /// bits 7 to 11
    rd: u8,

    /// bits 0 to 6
    opcode: u8,
}

impl InstructionFormatJ {
    pub fn new(instruction: u32) -> Self {
        Self {
            sign_imm_21_30: ((instruction >> 31) & 0b0001) as u8,
            imm_21_30: ((instruction >> 21) & 0b0011_1111_1111) as u8,
            sign_imm_12_19: ((instruction >> 20) & 0b0001) as u8,
            imm_12_19: ((instruction >> 12) & 0b1111_1111) as u8,
            rd: ((instruction >> 7) & 0b0001_1111) as u8,
            opcode: (instruction & 0b0111_1111) as u8,
        }
    }

    pub fn parse_instruction_from_bytes(bytes: &[u8]) -> Self {
        let instruction = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        Self::new(instruction)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        InstructionFormatB, InstructionFormatI, InstructionFormatJ, InstructionFormatR,
        InstructionFormatS, InstructionFormatU,
    };

    fn bits_to_u32(bits: &[u8]) -> u32 {
        bits.iter().fold(0, |acc, &bit| (acc << 1) | (bit as u32))
    }

    #[test]
    fn should_be_able_to_construct_r_instruction() {
        // funct7	rs2		rs1		funct3	rd		opcode
        // 0000000 	00011 	00011 	000 	00000	0110011
        let opcode: [u8; 7] = [0, 1, 1, 0, 0, 1, 1];
        let rd: [u8; 5] = [0, 0, 0, 0, 1];
        let func3: [u8; 3] = [0, 0, 0];
        let rs1: [u8; 5] = [0, 0, 0, 1, 0];
        let rs2: [u8; 5] = [0, 0, 0, 1, 1];
        let funct7: [u8; 7] = [0, 0, 0, 0, 0, 0, 0];

        let mut bits = Vec::with_capacity(32);
        bits.extend_from_slice(&funct7);
        bits.extend_from_slice(&rs2);
        bits.extend_from_slice(&rs1);
        bits.extend_from_slice(&func3);
        bits.extend_from_slice(&rd);
        bits.extend_from_slice(&opcode);

        let bits_as_u32 = bits_to_u32(&bits);
        let bits_as_little_endian = bits_as_u32.to_le_bytes();
        let instruction_format_r =
            InstructionFormatR::parse_instruction_from_bytes(&bits_as_little_endian);

        assert_eq!(instruction_format_r.opcode, 0b0110011);
        assert_eq!(instruction_format_r.rd, 0b00001);
        assert_eq!(instruction_format_r.funct3, 0b000);
        assert_eq!(instruction_format_r.rs1, 0b00010);
        assert_eq!(instruction_format_r.rs2, 0b00011);
        assert_eq!(instruction_format_r.funct7, 0b0000000);
    }

    #[test]
    fn should_be_able_to_construct_i_instruction() {
        // imm			rs1		funct3	rd		opcode
        // 000000000001 00010 	000 	00000	0110011
        let opcode: [u8; 7] = [0, 1, 1, 0, 0, 1, 1];
        let rd: [u8; 5] = [0, 0, 0, 0, 1];
        let func3: [u8; 3] = [0, 0, 0];
        let rs1: [u8; 5] = [0, 0, 0, 1, 0];
        let imm: [u8; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];

        let mut bits = Vec::with_capacity(32);
        bits.extend_from_slice(&imm);
        bits.extend_from_slice(&rs1);
        bits.extend_from_slice(&func3);
        bits.extend_from_slice(&rd);
        bits.extend_from_slice(&opcode);

        let bits_as_u32 = bits_to_u32(&bits);
        let bits_as_little_endian = bits_as_u32.to_le_bytes();
        let instruction_format_i =
            InstructionFormatI::parse_instruction_from_bytes(&bits_as_little_endian);

        assert_eq!(instruction_format_i.opcode, 0b0110011);
        assert_eq!(instruction_format_i.rd, 0b00001);
        assert_eq!(instruction_format_i.funct3, 0b000);
        assert_eq!(instruction_format_i.rs1, 0b00010);
        assert_eq!(instruction_format_i.imm, 0b000000000001);
    }

    #[test]
    fn should_be_able_to_construct_s_instruction() {
        // S‑type instruction fields order:
        // imm[11:5]   rs2      rs1     funct3    imm[4:0]    opcode

        let imm_5_to_11: [u8; 7] = [0, 0, 0, 0, 0, 0, 0];
        let rs2: [u8; 5] = [0, 0, 0, 1, 1]; // 00011 -> r3
        let rs1: [u8; 5] = [0, 0, 0, 1, 0]; // 00010 -> r2
        let func3: [u8; 3] = [0, 0, 0];
        let imm_0_to_4: [u8; 5] = [0, 0, 0, 1, 0]; // 00010 -> immediate low: 2
        let opcode: [u8; 7] = [0, 1, 0, 0, 0, 1, 1]; // 0100011 (store)

        let mut bits = Vec::with_capacity(32);
        bits.extend_from_slice(&imm_5_to_11);
        bits.extend_from_slice(&rs2);
        bits.extend_from_slice(&rs1);
        bits.extend_from_slice(&func3);
        bits.extend_from_slice(&imm_0_to_4);
        bits.extend_from_slice(&opcode);

        let bits_as_u32 = bits_to_u32(&bits);
        let bits_as_little_endian = bits_as_u32.to_le_bytes();
        let instruction_format_s =
            InstructionFormatS::parse_instruction_from_bytes(&bits_as_little_endian);

        // Assertions based on our test bit values:
        assert_eq!(instruction_format_s.opcode, 0b0100011);
        assert_eq!(instruction_format_s.imm_0_to_4, 0b00010);
        assert_eq!(instruction_format_s.func3, 0b000);
        assert_eq!(instruction_format_s.rs1, 0b00010);
        assert_eq!(instruction_format_s.rs2, 0b00011);
        assert_eq!(instruction_format_s.imm_5_to_11, 0b0000000);
    }

    #[test]
    fn should_be_able_to_construct_b_instruction() {
        let sign_imm_5_to_11: [u8; 1] = [0];
        let imm_5_to_11: [u8; 6] = [0, 0, 0, 0, 1, 1];
        let rs2: [u8; 5] = [0, 0, 0, 1, 1];
        let rs1: [u8; 5] = [0, 0, 0, 1, 0];
        let func3: [u8; 3] = [0, 0, 0];
        let imm_0_to_4: [u8; 4] = [0, 0, 1, 0];
        let sign_imm_0_to_4: [u8; 1] = [0];
        let opcode: [u8; 7] = [1, 1, 0, 0, 0, 1, 1];

        let mut bits = Vec::with_capacity(32);
        bits.extend_from_slice(&sign_imm_5_to_11);
        bits.extend_from_slice(&imm_5_to_11);
        bits.extend_from_slice(&rs2);
        bits.extend_from_slice(&rs1);
        bits.extend_from_slice(&func3);
        bits.extend_from_slice(&imm_0_to_4);
        bits.extend_from_slice(&sign_imm_0_to_4);
        bits.extend_from_slice(&opcode);

        let bits_as_u32 = bits_to_u32(&bits);
        let bits_as_little_endian = bits_as_u32.to_le_bytes();
        let instruction_format_b =
            InstructionFormatB::parse_instruction_from_bytes(&bits_as_little_endian);

        assert_eq!(instruction_format_b.sign_imm_5_to_11, 0000); // imm[12]
        assert_eq!(instruction_format_b.imm_5_to_11, 0b0000_0011); // imm[10:5] = 3
        assert_eq!(instruction_format_b.rs2, 0b0000_0011); // 3
        assert_eq!(instruction_format_b.rs1, 0b0000_0010); // 2
        assert_eq!(instruction_format_b.func3, 0b0000);
        assert_eq!(instruction_format_b.imm_0_to_4, 0b0010); // 2
        assert_eq!(instruction_format_b.sign_imm_0_to_4, 0000); // imm[11]
        assert_eq!(instruction_format_b.opcode, 0b0110_0011); // 0x63
    }

    #[test]
    fn should_be_able_to_construct_u_instruction() {
        // U‑type instruction fields order:
        // [imm (20 bits)] + [rd (5 bits)] + [opcode (7 bits)]
        //
        // For this test:
        // imm (20 bits) = 00000000000000000001 (just value 1)
        // rd (5 bits) = 00001  (register x1, value 1)
        // opcode (7 bits) = 0010111 (0x17, one of the U‑type opcodes)
        let imm: [u8; 20] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        let rd: [u8; 5] = [0, 0, 0, 0, 1];
        let opcode: [u8; 7] = [0, 0, 1, 0, 1, 1, 1]; // binary 0010111 equals 0x17

        let mut bits = Vec::with_capacity(32);
        bits.extend_from_slice(&imm); // bits 12–31
        bits.extend_from_slice(&rd); // bits 7–11
        bits.extend_from_slice(&opcode); // bits 0–6

        let bits_as_u32 = bits_to_u32(&bits);
        let bits_as_little_endian = bits_as_u32.to_le_bytes();

        let instruction_format_u =
            InstructionFormatU::parse_instruction_from_bytes(&bits_as_little_endian);

        // Check that the fields are parsed correctly.
        // Note: since we cast imm to u8, only the lowest 8 bits of the 20-bit field are stored.
        // In our case the 20-bit immediate is 1, so its lower 8 bits are also 1.
        assert_eq!(instruction_format_u.imm, 0b00000001);
        assert_eq!(instruction_format_u.rd, 0b00001);
        assert_eq!(instruction_format_u.opcode, 0b0010111);
    }

    #[test]
    fn should_be_able_to_construct_j_instruction() {
        let sign_imm_21_30: [u8; 1] = [0];
        let imm_21_30: [u8; 10] = [0, 0, 0, 0, 1, 0, 1, 0, 1, 0];
        let sign_imm_12_19: [u8; 1] = [1];
        let imm_12_19: [u8; 8] = [1, 0, 1, 0, 1, 0, 1, 0];
        let rd: [u8; 5] = [0, 0, 1, 0, 1];
        let opcode: [u8; 7] = [1, 1, 0, 1, 1, 1, 1];

        let mut bits = Vec::with_capacity(32);
        bits.extend_from_slice(&sign_imm_21_30);
        bits.extend_from_slice(&imm_21_30);
        bits.extend_from_slice(&sign_imm_12_19);
        bits.extend_from_slice(&imm_12_19);
        bits.extend_from_slice(&rd);
        bits.extend_from_slice(&opcode);

        assert_eq!(bits.len(), 32);

        let bits_as_u32 = bits_to_u32(&bits);
        let bits_as_little_endian = bits_as_u32.to_le_bytes();
        let instruction_format_j =
            InstructionFormatJ::parse_instruction_from_bytes(&bits_as_little_endian);

        assert_eq!(instruction_format_j.sign_imm_21_30, 0);
        assert_eq!(instruction_format_j.imm_21_30, 0b0000101010);
        assert_eq!(instruction_format_j.sign_imm_12_19, 1);
        assert_eq!(instruction_format_j.imm_12_19, 0b10101010);
        assert_eq!(instruction_format_j.rd, 0b00101);
        assert_eq!(instruction_format_j.opcode, 0b1101111);
    }
}
