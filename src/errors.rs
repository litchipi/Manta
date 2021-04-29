use std::fmt;

#[derive(Debug)]
pub enum Errcode{
    NotImplemented,
}


impl fmt::Display for Errcode{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(
            match &self{
                Errcode::NotImplemented => format_args!("This code is not yet implemented"),
            }
        )
    }
}
