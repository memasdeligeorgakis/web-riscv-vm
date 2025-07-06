use super::emulator::VmState;
use super::instruction_formats::{InstructionFormat, InstructionFormatR};
use super::instruction_signatures::{
    DestinationImmediate, DestinationSource1Immediate, DestinationSource1Source2,
    PseudoLiSignature, Source1Source2Immediate,
};

#[derive(Debug)]
pub enum Rv32iInstruction {
    // regulars
    /// ADD
    Add(DestinationSource1Source2),
    /// SUB
    Sub(DestinationSource1Source2),
    /// XOR
    Xor(DestinationSource1Source2),
    /// OR
    Or(DestinationSource1Source2),
    /// AND
    And(DestinationSource1Source2),
    /// Shift Left Logical
    Sll(DestinationSource1Source2),
    /// Shift Right Logical
    Srl(DestinationSource1Source2),
    /// Shift Right Arith
    Sra(DestinationSource1Source2),
    /// Set Less Than
    Slt(DestinationSource1Source2),
    /// Set Less Than
    Sltu(DestinationSource1Source2),

    // immediates
    /// ADD Immediate
    Addi(DestinationSource1Immediate),
    /// XOR Immediate
    Xori(DestinationSource1Immediate),
    /// OR Immediate
    Ori(DestinationSource1Immediate),
    /// AND Immediate
    Andi(DestinationSource1Immediate),
    /// Shift Left Logical
    Slli(DestinationSource1Immediate),
    /// Shift Right Logical
    Srli(DestinationSource1Immediate),
    /// Shift Right Arith
    Srai(DestinationSource1Immediate),
    /// Set Less Than Imm
    Slti(DestinationSource1Immediate),
    /// Set Less Than Imm
    Sltiu(DestinationSource1Immediate),

    // loads
    /// Load Byte
    Lb(DestinationSource1Immediate),
    /// Load Half Word
    Lh(DestinationSource1Immediate),
    /// Load Word
    Lw(DestinationSource1Immediate),
    /// Load Byte Upper
    Lbu(DestinationSource1Immediate),
    /// Load Half Word Upper
    Lhu(DestinationSource1Immediate),

    // storing
    /// Store Byte
    Sb(DestinationSource1Immediate),
    /// Store Half Word
    Sh(DestinationSource1Immediate),
    /// Store Word
    Sw(DestinationSource1Immediate),

    // branching
    /// Branch Equal (==)
    Beq(Source1Source2Immediate),
    /// Branch Not Equal (!=)
    Bne(Source1Source2Immediate),
    /// Branch Less Than (<)
    Blt(Source1Source2Immediate),
    /// Branch Greater Equal (>=)
    Bge(Source1Source2Immediate),
    /// Branch Less Than Upper (<)
    Bltu(Source1Source2Immediate),
    /// Branch Greater Equal Upper (<=)
    Bgeu(Source1Source2Immediate),

    // jumps
    /// Jump And Link
    Jal(DestinationImmediate),
    /// Jump And Link Reg
    Jalr(DestinationSource1Immediate),

    /// Load Upper Immediate
    Lui(DestinationImmediate),
    /// Add Upper Imm to PC
    Auipc(DestinationImmediate),
    /// Environment Call
    Ecall,
    /// Environment Break
    Ebreak,
}

/// the implementations of the instructions for RV32I are in this block
impl Rv32iInstruction {
    /// Implements the add instruction
    pub fn rv32i_instruction_add(
        destination_source1_source2: &DestinationSource1Source2,
        vm_state: &mut VmState,
    ) {
        if destination_source1_source2.rd == 0 {
            return;
        }

        let sum = vm_state.registers[destination_source1_source2.rs1 as usize]
            .wrapping_add(vm_state.registers[destination_source1_source2.rs2 as usize]);
        vm_state.registers[destination_source1_source2.rd as usize] = sum;
    }

    pub fn rv32i_instruction_bltu(
        source1_source2_immediate: &Source1Source2Immediate,
        vm_state: &mut VmState,
    ) {
        if source1_source2_immediate.rs1 < source1_source2_immediate.rs2 {
            vm_state.pc += source1_source2_immediate.imm as i32;
        }
    }

    pub fn rv32i_instruction_jal(
        destination_immediate: &DestinationImmediate,
        vm_state: &mut VmState,
    ) {
        // if source1_source2_immediate.rs1 < source1_source2_immediate.rs2 {
        //     vm_state.pc += source1_source2_immediate.imm as i32;
        // }
        vm_state.pc = destination_immediate.imm as i32;
        unimplemented!()
    }

    pub fn rv32i_instruction_jalr(
        destination_source1_immediate: &DestinationSource1Immediate,
        vm_state: &mut VmState,
    ) {
        // if source1_source2_immediate.rs1 < source1_source2_immediate.rs2 {
        //     vm_state.pc += source1_source2_immediate.imm as i32;
        // }

        unimplemented!()
    }

    /// 4 * 8bits = 32bits
    pub fn from_core_instruction_format(instruction: [u8; 4]) -> Self {
        // turn [u8; 4] to opcode
        let opcode = InstructionFormat::get_opcode_from_instruction(instruction);

        // figure out which RV32I instruction it is
        let instruction_format = InstructionFormat::detect_format_from_opcode(opcode);

        // try to cast to that RV32I
        let instruction_as_u32 = u32::from_le_bytes(instruction);

        let casted_instruction_maybe = match instruction_format {
            InstructionFormat::R => Some(InstructionFormatR::new(instruction_as_u32)),
            _ => None,
        };

        // Return
        unimplemented!()
    }
}
