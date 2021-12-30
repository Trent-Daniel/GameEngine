use std::fmt::{Display, Formatter, Error};

pub struct StatusCode<'a>
{
    name: &'a str,
    //TODO: Implement the following properties
    //description: &'static str,
    //id: u8,
}

impl <'a> Display for StatusCode<'a>
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>
    {
        write!(f, "{}", self.name)
    }
}

pub mod status_codes
{
    pub const OK: super::StatusCode = super::StatusCode
                                             {
                                                 name: "OK",
                                                 //description: "Operation was successful.",
                                                 //id: 0,
                                             };
}
