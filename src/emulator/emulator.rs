use super::instruction_signatures::{
    DestinationImmediate, DestinationSource1Immediate, DestinationSource1Source2,
    PseudoLiSignature, Source1Source2Immediate,
};
use super::rv32i::Rv32iInstruction;
use std::usize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Rv32iInstructionError {
    #[error("the instruction `{0}` is not implemented yet")]
    InstructionNotImplemented(String),
}

#[derive(Debug)]
pub struct Registers {
    registers: [i32; 32],
}

#[derive(Debug)]
pub struct VmState {
    pub registers: [i32; 32],

    /// stack pointer
    // This should be register x2 by the specs (page 41)
    // https://drive.google.com/file/d/1uviu1nH-tScFfgrovvFCrj7Omv8tFtkp/view
    pub sp: i32,

    /// program counter
    pub pc: i32,
}

impl Default for VmState {
    fn default() -> Self {
        let mut initial_registers = [0; 32];
        Self {
            pc: 1000,
            registers: initial_registers,
            sp: 0,
        }
    }
}

struct Vm {
    vm_state: VmState,
}

impl Vm {
    pub fn execute_instructions(
        &mut self,
        instructions: Vec<Instruction>,
    ) -> Result<(), Rv32iInstructionError> {
        for instruction in instructions {
            let _ = instruction.execute_instruction(&mut self.vm_state);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Instruction {
    PseudoInstruction(i32, PseudoInstruction),
    Rv32iInstruction(i32, Rv32iInstruction),
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
        let _ = match self {
            // RV32I extension
            Self::Rv32iInstruction(memory_address, rv32i_instruction) => match rv32i_instruction {
                Rv32iInstruction::Add(destination_source1_source2) => {
                    Rv32iInstruction::rv32i_instruction_add(destination_source1_source2, vm_state);
                    Ok(())
                }
                Rv32iInstruction::Bltu(source1_source2_immediate) => {
                    Rv32iInstruction::rv32i_instruction_bltu(source1_source2_immediate, vm_state);
                    Ok(())
                }
                // if we end up here, it means that the instruction is not
                // implemented yet
                _ => {
                    println!("not implemented yet: {self:?}");
                    Err(Rv32iInstructionError::InstructionNotImplemented(
                        "aaa".to_string(),
                    ))
                }
            },

            // pseudo instructions extension
            Self::PseudoInstruction(memory_address, pseudo_instruction) => match pseudo_instruction
            {
                PseudoInstruction::Li(DestinationImmediate { rd, imm }) => {
                    vm_state.registers[*rd as usize] = *imm as i32;
                    Ok(())
                }
                PseudoInstruction::Ret => Ok(()),
                _ => {
                    println!("not implemented yet: {self:?}");
                    Err(Rv32iInstructionError::InstructionNotImplemented(
                        "aaa".to_string(),
                    ))
                }
            },
        };

        vm_state.pc += 4;

        Ok(())
    }
}

#[derive(Debug)]
pub enum PseudoInstruction {
    Ret,
    Li(DestinationImmediate),
}

pub struct Emulator {
    // index 1 to hold the return address for a call
    // index 2 should be stack pointer
    registers: [u32; 32],
    pc: u32,
}

impl Emulator {
    /// creates a new instance of emulator
    /// TODO: take as a parameter whether this is a 32bit or 64bits instruction set
    pub fn new() -> Self {
        Self {
            registers: [0; 32],
            pc: 0x1000,
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
    use super::Rv32iInstruction;
    use super::Source1Source2Immediate;
    use super::Vm;
    use super::VmState;

    #[test]
    fn test_a_whole_function_for_correct_program_counter_and_return_value() {
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
            Instruction::PseudoInstruction(
                0x1000,
                PseudoInstruction::Li(DestinationImmediate { rd: 13, imm: 10 }),
            ),
            Instruction::PseudoInstruction(
                0x1004,
                PseudoInstruction::Li(DestinationImmediate { rd: 12, imm: 15 }),
            ),
            Instruction::Rv32iInstruction(
                0x1008,
                Rv32iInstruction::Bltu(Source1Source2Immediate {
                    rs1: 10,
                    rs2: 11,
                    imm: 0x10,
                }),
            ),
            Instruction::PseudoInstruction(
                0x100c,
                PseudoInstruction::Li(DestinationImmediate { rd: 12, imm: 25 }),
            ),
            Instruction::Rv32iInstruction(
                0x1010,
                Rv32iInstruction::Add(DestinationSource1Source2 {
                    rd: 10,
                    rs1: 10,
                    rs2: 11,
                }),
            ),
            Instruction::Rv32iInstruction(
                0x1014,
                Rv32iInstruction::Add(DestinationSource1Source2 {
                    rd: 10,
                    rs1: 10,
                    rs2: 12,
                }),
            ),
            Instruction::PseudoInstruction(0x1000, PseudoInstruction::Ret),
        ];

        // this is our new fresh registers
        let mut initial_registers = [0; 32];

        // function input values
        initial_registers[10] = 50;
        initial_registers[11] = 55;

        //
        let vm_state = VmState {
            registers: initial_registers,
            pc: 0x1000,
            sp: 0,
        };

        let mut vm = Vm { vm_state };

        let _ = vm.execute_instructions(instructions);

        let expected_vm_registers_state = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 130, 55, 25, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
        ];

        let return_value = vm.vm_state.registers[10];
        assert_eq!(return_value, 130);

        // assert register state
        assert_eq!(vm.vm_state.registers, expected_vm_registers_state);

        // assert program counter
        assert_eq!(vm.vm_state.pc, 4140);
    }

    #[test]
    fn should_initiate_vm_with_correct_default_values() {
        // let mut emulator = Emulator::new();

        let vm_state = VmState::default();

        dbg!(vm_state);

        // so effectively it becomes
        // in 16 bits
        // 0001 0000 0000 0000
        // assert_eq!(
        //     emulator.get_register_value(register_index),
        //     0b0001_0000_0000_0000
        // );
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
