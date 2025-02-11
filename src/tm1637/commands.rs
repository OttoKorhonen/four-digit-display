
use crate::tm1637::command_byte::CommandByte;

pub enum DataCommand {
    WriteDataToDisplayRegister = 0b0100_0000,
    ReadKeyScan = 0b0100_0010,
}

impl CommandByte for DataCommand {
    fn to_u8(self) -> u8 {
        self as u8
    }
}

pub enum AddressMode {
    Automatic = 0b0100_0000,
    Fixed = 0b0100_0100,
}

impl CommandByte for AddressMode{
    fn to_u8(self) -> u8 {
        self as u8
    }
}

pub enum DisplayMode {
    Normal = 0b0100_0000,
    Test = 0b0100_1000,
}

impl CommandByte for DisplayMode {
    fn to_u8(self) -> u8 {
        self as u8
    }
}