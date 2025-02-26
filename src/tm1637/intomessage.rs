use heapless::String;
use core::fmt::Write;


pub trait IntoMessage{
    fn write_to(&self, buffer: &mut String<4>);
}

impl IntoMessage for &str {
    fn write_to(&self, buffer: &mut String<4>) {
        buffer.push_str(self).ok();
    }
}

impl IntoMessage for u32 {
    fn write_to(&self, buffer: &mut String<4>) {
        write!(buffer, "{}", self).ok();
    }
}