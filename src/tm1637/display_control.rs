
use crate::tm1637::CommandByte;

pub enum DisplaySwitch {
    On = 0b1000_1000,
    Off = 0b1000_0000
}

impl CommandByte for DisplaySwitch {
    fn to_u8(self) -> u8 {
        self as u8
    }
}

///pulse width set between 1-16
pub enum PulseWidth {
    One = 0b1000_0000,
    Two = 0b1000_0001,
    Four = 0b1000_0010,
    Ten = 0b1000_0011,
    Eleven = 0b1000_0100,
    Twelve = 0b1000_0101,
    Thirteen = 0b1000_0110,
    Fourteen = 0b1000_0111
}

impl CommandByte for PulseWidth {
    fn to_u8(self) -> u8 {
        self as u8
    }
}