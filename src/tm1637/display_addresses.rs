
use crate::tm1637::CommandByte;

pub enum Address {
    C0H = 0b1100_0000,
    C1H = 0b1100_0001,
    C2H = 0b1100_0010,
    C3H = 0b1100_0011,
    C4H = 0b1100_0100,
    C5H = 0b1100_0101
}

impl CommandByte for Address {
    fn to_u8(self) -> u8 {
        self as u8
    }
}