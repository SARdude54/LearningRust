#![no_std]
#![crate_type = "lib"]

#[unsafe(no_mangle)]
pub extern "C" fn set_bit(value: u32, bit: u32) -> u32 {
    value | (1u32 << bit)
}