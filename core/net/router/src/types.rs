use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Methods{
    GET,
    POST
}

impl fmt::Display for Methods {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Methods::GET => write!(f, "GET"),
            Methods::POST => write!(f, "POST")
        }
    }
}