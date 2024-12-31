use std::usize;
use thiserror::Error;

#[derive(Error, Debug)]
enum Rv32iInstructionError {
    #[error("the instruction `{0}` is not implemented yet")]
    InstructionNotImplemented(String),
}

#[derive(Debug)]
pub struct Registers {
    registers: [i32; 32],
}

#[derive(Debug)]
struct VmState {
    registers: [i32; 32],
    sp: i32,
    pc: u8,
}

#[derive(Debug)]
pub struct DestinationSource1Source2 {
    rd: u8,
    rs1: u8,
    rs2: u8,
}

#[derive(Debug)]
pub struct Source1Source2Immediate {
    rs1: u8,
    rs2: u8,
    imm: i16,
}

#[derive(Debug)]
pub struct DestinationSource1Immediate {
    rd: u8,
    rs1: u8,
    imm: i16,
}

#[derive(Debug)]
pub struct DestinationImmediate {
    rd: u8,
    imm: i16,
}

#[derive(Debug)]
struct PseudoLiSignature {
    rd: u8,
    imm: i32,
}

#[derive(Debug)]
pub enum Instruction {
    PseudoInstruction(PseudoInstruction),
    Rv32iInstruction(Rv32iInstruction),
}

impl Instruction {
    /// this executes the instruction. Well the execution is only possible if
    /// we have creates the extension and if in that extension we have
    /// implemented the instruction.
    ///
    /// So the most top level enum Instruction contains extensions such as RV32I,
    /// RV64I, RV32V, ...
    ///
    /// So as this will execute the instruction, we will likely need to mutate
    /// the register or other values, so we pass in the VM state where the registers
    /// PC, ... live.
    ///
    /// The VM state is not part of the instruction, instruction is just the
    /// instruction and its execution.
    ///
    /// So to put it simply the responsibility of this function is to
    /// map the right instruction execution function based on the instruction
    /// variant and pass in the VM state.
    pub fn execute_instruction(&self, vm_state: &mut VmState) -> Result<(), Rv32iInstructionError> {
        let registers = vm_state.registers;
        match self {
            // RV32I extension
            Self::Rv32iInstruction(rv32i_instruction) => match rv32i_instruction {
                Rv32iInstruction::Add(destination_source1_source2) => {
                    Rv32iInstruction::rv32i_instruction_add(destination_source1_source2, vm_state);
                    Ok(())
                }
                _ => {
                    println!("not implemented yet: {self:?}");
                    Err(Rv32iInstructionError::InstructionNotImplemented(
                        "aaa".to_string(),
                    ))
                }
            },
            // pseudo instructions extension
            Self::PseudoInstruction(pseudo_instruction) => match pseudo_instruction {
                PseudoInstruction::Li(DestinationImmediate { rd, imm }) => {
                    println!("{self:?}");
                    vm_state.registers[*rd as usize] = *imm as i32;
                    Ok(())
                }
                _ => {
                    println!("not implemented yet: {self:?}");
                    Err(Rv32iInstructionError::InstructionNotImplemented(
                        "aaa".to_string(),
                    ))
                }
            },
        }
    }
}

#[derive(Debug)]
pub enum PseudoInstruction {
    Ret,
    Li(DestinationImmediate),
}

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
    fn aaa(&self) {
        match self {
            Self::Add(signature) => {}
            _ => {
                println!("not implemented yet")
            }
        }
    }
    /// Implements the add instruction
    fn rv32i_instruction_add(
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
}

pub struct Emulator {
    registers: [u32; 32],
    pc: u32,
}

impl Emulator {
    /// creates a new instance of emulator
    /// TODO: take as a parameter whether this is a 32bit or 64bits instruction set
    pub fn new() -> Self {
        Self {
            registers: [0; 32],
            pc: 0,
        }
    }

    /// returns a value at the register index `register_index`
    pub fn get_register_value(&self, register_index: usize) -> i32 {
        self.registers[register_index] as i32
    }

    /// ADD - ADD
    pub fn add(&mut self, rd: usize, rs1: usize, rs2: usize) {
        if rd == 0 {
            return;
        }

        let sum = self.registers[rs1].wrapping_add(self.registers[rs2]);
        self.registers[rd] = sum;
    }

    /// ADDI - ADD Immediate
    pub fn addi(&mut self, rd: usize, rs1: usize, imm: i32) {
        if rd == 0 {
            return;
        }

        let sum = self.registers[rs1].wrapping_add(imm as u32);
        self.registers[rd] = sum;
    }

    /// SUB - SUBtract
    pub fn sub(&mut self, rd: usize, rs1: usize, rs2: usize) {
        if rd == 0 {
            return;
        }

        let sum = self.registers[rs1].wrapping_sub(self.registers[rs2]);
        self.registers[rd] = sum;
    }

    /// LUI -  Load Upper Imm
    pub fn lui(&mut self, rd: usize, imm: i32) {
        if rd == 0 {
            return;
        }

        // shift the bits by 12 bits
        let imm_as_bits = imm << 12;

        self.registers[rd] = imm_as_bits as u32;
    }

    /// AUIPC - Add Upper Imm to PC
    pub fn auipc(&mut self, rd: usize, imm: i32) {
        if rd == 0 {
            return;
        }

        // shift the bits by 12 bits
        let imm_as_bits = imm << 12;

        self.registers[rd] = self.pc + imm_as_bits as u32;
    }
}

#[cfg(test)]
mod test {

    use super::DestinationImmediate;
    use super::DestinationSource1Immediate;
    use super::DestinationSource1Source2;
    use super::Emulator;
    use super::Instruction;
    use super::PseudoInstruction;
    use super::PseudoLiSignature;
    use super::Registers;
    use super::Rv32iInstruction;
    use super::Source1Source2Immediate;
    use super::VmState;

    #[test]
    fn sketch() {
        // This Rust code
        //
        // #[no_mangle]
        // #[inline(never)]
        // fn sum_2_number(number_1: u32, number_2: u32) -> u32 {
        //     	let variable_1 = number_1 + number_2;
        //     	let variable_2 = if number_1 < 10 {
        //      	15
        //     	} else {
        //      	25
        //     	};
        //     variable_1 + variable_2
        // }
        //
        // translates into this RISC-V assembly
        //
        // 	<sum_2_number>:
        // 		46a9     li a3,10
        // 		463d     li a2,15
        // 		00d56363 bltu a0,a3,<sum_2_number+0xa>
        // 		4665     li a2,25
        // 		952e     add a0,a0,a1
        // 		9532     add a0,a0,a2
        // 		8082     ret

        // this is the program
        let instructions = vec![
            Instruction::PseudoInstruction(PseudoInstruction::Li(DestinationImmediate {
                rd: 13,
                imm: 10,
            })),
            Instruction::PseudoInstruction(PseudoInstruction::Li(DestinationImmediate {
                rd: 12,
                imm: 15,
            })),
            Instruction::Rv32iInstruction(Rv32iInstruction::Bltu(Source1Source2Immediate {
                rs1: 10,
                rs2: 11,
                imm: 11,
            })),
            Instruction::PseudoInstruction(PseudoInstruction::Li(DestinationImmediate {
                rd: 12,
                imm: 25,
            })),
            Instruction::Rv32iInstruction(Rv32iInstruction::Add(DestinationSource1Source2 {
                rd: 10,
                rs1: 10,
                rs2: 11,
            })),
            Instruction::Rv32iInstruction(Rv32iInstruction::Add(DestinationSource1Source2 {
                rd: 10,
                rs1: 10,
                rs2: 12,
            })),
            Instruction::PseudoInstruction(PseudoInstruction::Ret),
        ];

        // this is our new fresh registers
        let mut initial_registers = [0; 32];
        initial_registers[10] = 2;
        initial_registers[11] = 3;
        let mut vm_state = VmState {
            registers: initial_registers,
            pc: 0,
            sp: 0,
        };

        for instruction in instructions {
            let execute_instruction_result = instruction.execute_instruction(&mut vm_state);
        }

        let expected_vm_registers_state = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 3, 25, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0,
        ];

        assert_eq!(vm_state.registers, expected_vm_registers_state);
    }

    #[test]
    fn lui_should_work_correctly() {
        let mut emulator = Emulator::new();

        // 20 bits
        // 0000 0000 0000 0000 0001
        let register_index = 1;

        // 32 bits
        // 0000 0000 0000 0000 0001 0000 0000 0000
        emulator.lui(register_index, 1);

        assert_eq!(emulator.get_register_value(register_index), 4096);

        // so effectively it becomes
        // in 16 bits
        // 0001 0000 0000 0000
        assert_eq!(
            emulator.get_register_value(register_index),
            0b0001_0000_0000_0000
        );
    }
}
