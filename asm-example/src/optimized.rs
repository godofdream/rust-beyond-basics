use std::arch::asm;

// A simple function using assembler to make it complicated
#[inline(always)] // we force inling to make it faster
pub(crate) fn add_two_numbers(a: i32, b: i32) -> i32 {
    let mut res: i32 = a;
    unsafe {
        asm!(
            "add {res:e}, {b:e}",
            res = inout(reg) res,
            b = in(reg) b,
        );
    }
    res
}