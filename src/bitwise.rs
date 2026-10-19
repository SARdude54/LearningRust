pub fn set_bit(value: u32, bit: u8) -> u32 {
    value | (1u32 << bit)
}

pub fn clear_bit(value: u32, bit: u8) -> u32 {
    value & !(1u32 << bit)
}

pub fn toggle_bit(value: u32, bit: u8) -> u32 {
    value ^ (1u32 << bit)
}

pub fn read_bit(value: u32, bit: u8) -> bool {
    (value & (1u32 << bit)) != 0
}