#[inline(never)] // avoid inlining the function
#[unsafe(no_mangle)] // preserve symbol name "set_bit"
pub extern "C" fn set_bit(value: u32, bit: u32) -> u32 {
    value | (1u32 << bit)
}

fn main() {
    let value: u32 = 0b0000_1010;

    let result = set_bit(value, 2);

    println!("Result: {:#010b}", result);
}