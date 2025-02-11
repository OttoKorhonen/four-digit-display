use core::ops::Add;
use crate::tm1637::CommandByte;


#[derive(Clone, Copy, Debug)]
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

impl Add<i32> for Address {
    type Output = Option<Address>;

    fn add(self, other: i32) -> Option<Address> {
        let result = self.to_u8().wrapping_add(other as u8); // Turvallinen ylivuoto
        match result {
            0b1100_0000 => Some(Address::C0H),
            0b1100_0001 => Some(Address::C1H),
            0b1100_0010 => Some(Address::C2H),
            0b1100_0011 => Some(Address::C3H),
            0b1100_0100 => Some(Address::C4H),
            0b1100_0101 => Some(Address::C5H),
            _ => None, // Palautetaan None virhetilanteessa
        }
    }
}