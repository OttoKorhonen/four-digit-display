use core::fmt;

#[derive(Debug)]
pub enum Tm1637Error<E: fmt::Debug>{
    CreationError,
    ReadError,
    WriteError,
    NoAck(E)
}

impl<E: fmt::Debug> fmt::Display for Tm1637Error<E> {
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tm1637Error::CreationError => write!(f, "Failed to create new instance of TM1637"),
            Tm1637Error::NoAck(e) => write!(f, "No acknowledgement {:?}", e),
            Tm1637Error::ReadError => write!(f,"Failed to read source"),
            Tm1637Error::WriteError => write!(f,"Failed to write to source")
        }
    }
    
}