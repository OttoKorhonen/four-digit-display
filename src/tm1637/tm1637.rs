use crate::tm1637::{AddressMode, CommandByte, DataCommand, DisplaySwitch};
use embedded_hal::{
    delay::DelayNs,
    digital::{OutputPin, StatefulOutputPin},
};
use heapless::Vec;

pub struct Tm1637<'a, E, D>
where
    E: embedded_hal::digital::Error,
    D: DelayNs,
{
    dio: &'a mut dyn StatefulOutputPin<Error = E>,
    scl: &'a mut dyn OutputPin<Error = E>,
    delay: D,
}

impl<'a, E, D> Tm1637<'a, E, D>
where
    E: embedded_hal::digital::Error,
    D: DelayNs,
{
    pub fn new(
        dio: &'a mut dyn StatefulOutputPin<Error = E>,
        scl: &'a mut dyn OutputPin<Error = E>,
        delay: D,
    ) -> Self {
        Self { dio, scl, delay }
    }

    fn start_input(&mut self) {
        self.dio.set_high().unwrap();
        self.scl.set_high().unwrap();
        self.delay.delay_ms(5);

        self.dio.set_low().unwrap();
        self.delay.delay_ms(5);
        self.scl.set_low().unwrap();
    }

    fn end_input(&mut self) {
        self.scl.set_low().unwrap();
        self.dio.set_low().unwrap();
        self.delay.delay_ms(5);

        self.scl.set_high().unwrap();
        self.delay.delay_ms(5);
        self.dio.set_high().unwrap();
    }

    fn write_byte(&mut self, byte: u8) -> bool {
        for i in 0..8 {
            // Asetetaan DIO bittiarvon mukaan
            if (byte >> i) & 1 == 1 {
                self.dio.set_high().unwrap();
            } else {
                self.dio.set_low().unwrap();
            }

            // Kello-pulssi
            self.scl.set_high().unwrap();
            self.delay.delay_ms(5);
            self.scl.set_low().unwrap();
            self.delay.delay_ms(5);
        }

        self.scl.set_high().unwrap();
        self.delay.delay_ms(5);

        let ack = self.dio.is_set_low().unwrap(); // TM1637 vetää linjan alas, jos ACK annetaan

        self.scl.set_low().unwrap();
        self.delay.delay_ms(5);

        ack
    }

    fn write_value_to_register(&mut self, bit_vec: &[u8]) {
        self.start_input();

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

    fn digit_to_segment(&mut self, num: u8) -> u8 {
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

    pub fn init(&mut self) {
        self.write_command_to_register(DisplaySwitch::On);
        self.delay.delay_ms(5);
        self.write_command_to_register(AddressMode::Automatic);
        self.delay.delay_ms(5);
        self.write_command_to_register(DataCommand::WriteDataToDisplayRegister);
        self.delay.delay_ms(5);
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
            let bit = self.digit_to_segment(digit as u8);
            bit_vec.push(bit).unwrap();
        }

        self.write_value_to_register(&bit_vec);
        self.write_command_to_register(DisplaySwitch::Off);
    }

    fn command_to_u8<T>(&mut self, command: T) -> u8
    where
        T: CommandByte,
    {
        command.to_u8()
    }
}

// impl<E: fmt::Debug> Error for Tm1637Error<E> {}
