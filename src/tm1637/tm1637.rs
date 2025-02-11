use crate::tm1637::{
    Address, AddressMode, CommandByte, DataCommand, DisplayMode, DisplaySwitch, PulseWidth,
    Tm1637Error,
};
use esp_hal::delay::Delay;
use esp_hal::gpio::OutputOpenDrain;
use heapless::Vec;

pub struct Tm1637<'a> {
    dio: OutputOpenDrain<'a>,
    scl: OutputOpenDrain<'a>,
    delay: Delay,
}

impl<'a> Tm1637<'a> {
    pub fn new(sda: OutputOpenDrain<'a>, scl: OutputOpenDrain<'a>) -> Self {
        Self {
            dio: sda,
            scl,
            delay: Delay::new(),
        }
    }

    fn start_input(&mut self) {
        self.dio.set_high();
        self.scl.set_high();
        self.delay.delay_micros(3);

        self.dio.set_low();
        self.delay.delay_micros(3);
        self.scl.set_low();
    }

    fn end_input(&mut self) {
        self.scl.set_low();
        self.dio.set_low();
        self.delay.delay_micros(3);

        self.scl.set_high();
        self.delay.delay_micros(3);
        self.dio.set_high();
    }

    pub fn write_byte(&mut self, byte: u8) -> bool {
        for i in 0..8 {
            // Asetetaan DIO bittiarvon mukaan
            if (byte >> i) & 1 == 1 {
                self.dio.set_high();
            } else {
                self.dio.set_low();
            }

            // Kello-pulssi
            self.scl.set_high();
            self.delay.delay_micros(3);
            self.scl.set_low();
            self.delay.delay_micros(3);
        }

        self.scl.set_high();
        self.delay.delay_micros(3);

        let ack = self.dio.is_low(); // TM1637 vetää linjan alas, jos ACK annetaan

        self.scl.set_low();
        self.delay.delay_micros(3);

        ack
    }

    pub fn write_value_to_register(&mut self, bit_vec: &[u8]) {
        self.start_input();

        self.write_command_to_register(DisplaySwitch::On);
        self.delay.delay_micros(3);

        self.write_command_to_register(AddressMode::Automatic);
        self.delay.delay_micros(3);

        self.write_command_to_register(DataCommand::WriteDataToDisplayRegister);
        self.delay.delay_micros(3);

        for bit in bit_vec {
            self.write_byte(*bit);
        }

        self.write_command_to_register(DisplaySwitch::Off);

        self.end_input();
    }

    fn write_command_to_register<T>(&mut self, value: T)
    where
        T: CommandByte,
    {
        let bit = self.command_to_u8(value);
        self.write_byte(bit);
    }

    fn match_segment(&mut self, num: u8) -> u8 {
        match num {
            0 => 0x3F,
            1 => 0x06,
            2 => 0x5B,
            3 => 0x4F,
            4 => 0x66,
            5 => 0x6D,
            6 => 0x7D,
            7 => 0x07,
            8 => 0x7F,
            9 => 0x6F,
            _ => 0x00,
        }
    }

    /// write to the display. Max 4 digits
    pub fn write(&mut self, message: u16) {
        let digits = [
            (message / 1000) % 10,
            (message / 100) % 10,
            (message / 10) % 10,
            message % 10,
        ];

        let mut bit_vec: Vec<u8, 4> = Vec::new();

        for &digit in &digits {
            let bit = self.match_segment(digit as u8);
            bit_vec.push(bit).unwrap();
        }

        self.write_value_to_register(&bit_vec);
    }

    fn command_to_u8<T>(&mut self, command: T) -> u8
    where
        T: CommandByte,
    {
        command.to_u8()
    }
}

// impl<E: fmt::Debug> Error for Tm1637Error<E> {}
