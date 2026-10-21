mod bitwise;

use rust_workspace::bitwise::{clear_bit, read_bit, set_bit, toggle_bit};

fn main() {
    let value: u32 = 0b0000_1010;

    println!("Original: {:#010b}", value);

    let result = set_bit(value, 2);
    println!("Set bit 2: {:#010b}", result);

    let result = clear_bit(value, 1);
    println!("Clear bit 1: {:#010b}", result);

    let result = toggle_bit(value, 3);
    println!("Toggle bit 3: {:#010b}", result);

    let result = read_bit(value, 3);
    println!("Read bit 3: {}", result);
}