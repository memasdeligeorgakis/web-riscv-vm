#![no_main]
use libfuzzer_sys::fuzz_target;
use riscv_emulator::instruction_formats::InstructionFormatR;

fuzz_target!(|data: &[u8]| {
    // Only consider inputs that are exactly 4 bytes,
    // because parse_instruction_from_bytes expects a 4-byte slice.
    if data.len() != 4 {
        return;
    }

    // just fuzz for panics
    let _ = InstructionFormatR::parse_instruction_from_bytes(data);
});
