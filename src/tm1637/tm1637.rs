use crate::tm1637::{AddressMode, CommandByte, DisplaySwitch, IntoMessage};
use embedded_hal::{
    delay::DelayNs,
    digital::{InputPin, OutputPin},
};
use heapless::{String, Vec};

pub struct Tm1637<Dio, Scl, D> {
    dio: Dio,
    scl: Scl,
    delay: D,
}

impl<Dio, Scl, D> Tm1637<Dio, Scl, D>
where
    Dio: OutputPin + InputPin,
    Scl: OutputPin,
    D: DelayNs,
{
    pub fn new(dio: Dio, scl: Scl, delay: D) -> Self {
        Self { dio, scl, delay }
    }

    fn start_input(&mut self) {
        self.dio.set_high().unwrap();
        self.scl.set_high().unwrap();
        self.delay.delay_ns(2);

        self.dio.set_low().unwrap();
        self.delay.delay_ns(2);
        self.scl.set_low().unwrap();
    }

    fn end_input(&mut self) {
        self.scl.set_low().unwrap();
        self.dio.set_low().unwrap();
        self.delay.delay_ns(2);

        self.scl.set_high().unwrap();
        self.delay.delay_ns(2);
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
            self.delay.delay_ns(2);
            self.scl.set_low().unwrap();
            self.delay.delay_ns(2);
        }

        self.scl.set_high().unwrap();
        self.delay.delay_ns(2);

        let ack = self.dio.is_low().unwrap_or(false); // TM1637 vetää linjan alas, jos ACK annetaan

        self.scl.set_low().unwrap();
        self.delay.delay_ns(2);

        ack
    }

    fn write_value_to_register(&mut self, bit_vec: &[u8]) {
        self.start_input();

        for bit in bit_vec {
            self.write_byte(*bit);
        }

        self.write_command_to_register(DisplaySwitch::Off);
        self.write_byte(0xC0);
        self.end_input();
    }

    fn write_command_to_register<T>(&mut self, value: T)
    where
        T: CommandByte,
    {
        let bit = self.command_to_u8(value);
        self.write_byte(bit);
    }

    fn set_brightness(&mut self, brightness: u8) {
        let brightness = brightness.min(7); // Maksimi kirkkaus on 7
        let command = 0x88 | brightness; // Display control command

        self.start_input();
        self.write_byte(command);
        self.end_input();
    }

    pub fn init(&mut self) {
        self.delay.delay_ms(10);
        
        self.start_input();
        self.write_byte(AddressMode::Automatic as u8);
        self.end_input();
        self.delay.delay_ms(1);
        
        self.set_brightness(3);
        self.delay.delay_ms(1);
    }

    fn char_to_segment(&self, ch: char) -> u8 {
        match ch {
            '0' => 0x3F,
            '1' => 0x06,
            '2' => 0x5B,
            '3' => 0x4F,
            '4' => 0x66,
            '5' => 0x6D,
            '6' => 0x7D,
            '7' => 0x07,
            '8' => 0x7F,
            '9' => 0x6F,
            'A' | 'a' => 0x77,
            'B' | 'b' => 0x7C,
            'C' | 'c' => 0x39,
            'D' | 'd' => 0x5E,
            'E' | 'e' => 0x79,
            'F' | 'f' => 0x71,
            'N' | 'n' => 0x4E,
            'O' | 'o' => 0x4F,
            'R' | 'r' => 0x72,
            _ => 0x00,
        }
    }

    /// write to the display. Max 4 digits
    pub fn write<T>(&mut self, message: T)
    where
        T: IntoMessage,
    {
        let mut buffer = String::<4>::new();
        message.write_to(&mut buffer);

        // let mut bit_vec: Vec<u8, 4> = Vec::new();
        let mut bit_vec: [u8; 4] = [0x00; 4];
        let chars: Vec<char, 4> = buffer.chars().collect();

        for (i, &ch) in chars.iter().take(4).enumerate() {
            bit_vec[i] = self.char_to_segment(ch);
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
