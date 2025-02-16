#![no_std]
#![no_main]
// #![feature(asm)]

use core::arch::asm;

use core::panic::PanicInfo;

#[used]
static mut RESULT: u32 = 0;

#[used]
static mut RESULT_2: u32 = 0;

#[used]
static mut VARIABLE_1: i32 = 0;

#[no_mangle]
#[inline(never)]
pub extern "C" fn _start() -> () {
    main();

    unsafe {
        RESULT = 1; // Store result in a static variable to prevent optimization
    }
}

#[no_mangle]
#[inline(never)]
fn main() {
    let number_1: u32 = 1;
    let number_2: u32 = 2;
    let number_3: u32 = 3;
    let sum_of_numbers = sum_2_number(number_1, number_2);
    let sum_of_numbers: u32 = sum_of_numbers + number_3;

    // let aaa = unsafe { add_vectors_rvv(&VARIABLE_1, &VARIABLE_1, &mut VARIABLE_1, 3) };

    unsafe {
        RESULT_2 = sum_of_numbers; // Store result in a static variable to prevent optimization
    }
}

#[no_mangle]
#[inline(never)]
fn sum_2_number(number_1: u32, number_2: u32) -> u32 {
    let variable_1 = number_1 + number_2;
    let variable_2 = if number_1 < 10 { 15 } else { 25 };

    variable_1 + variable_2
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
