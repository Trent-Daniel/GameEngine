use std::fmt::{Display, Formatter, Error};

pub struct ErrorCode
{
    // The static lifetime specifier means that we can only pass string literals or other slices
    // with similar lifetimes to the name and description properties
    name: &'static str,
    //TODO: Implement the following properties
    //description: &'static str,
    //id: u8,
}

impl Display for ErrorCode
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
    {
        write!(f, "{}", self.name)
    }
}

pub mod error_codes
{
    pub const GENERIC_ERROR: super::ErrorCode = super::ErrorCode
                                             {
                                                 name: "GENERIC_ERROR",
                                                 //description: "This is a generic error",
                                                 //id: 0,
                                             };
}
