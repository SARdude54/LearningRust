use bitflags::bitflags; // macro that generates a type representing a set of bit flags

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)] // attribute that requests implementations of several standard traits
    pub struct StatusFlags: u32 {
        const TX_READY          = 1 << 0;
        const RX_READY          = 1 << 1;
        const TX_COMPLETE       = 1 << 2;
        const OVERRRUN_ERROR    = 1 << 3;
    }
}
