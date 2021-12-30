//use std::fmt::{Display, Formatter, Error};

pub mod key_events;
pub mod mouse_events;
pub mod application_events;

/*
pub struct Event<'a>
{
    name: &'a str,
    //TODO: Implement the following properties
    //description: &'static str,
    //id: u8,
}

impl <'a> Display for Event<'a>
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>
    {
        write!(f, "{}", self.name)
    }
}

pub mod events
{
    pub const GENERIC_EVENT: super::Event = super::Event
                                             {
                                                 name: "GENERIC_EVENT",
                                                 //description: "This is a generic error",
                                                 //id: 0,
                                             };
}
*/

pub trait Event
{
    // Need to create a dispatcher

    // get_name, get_description, to_string, maybe others could be moved to debug only? Not sure if
    // this is possible

    fn get_event_type(&self) -> event_type::DataType;

    fn get_event_category_flags(&self) -> event_category::DataType;

    // We should try to improve the performance of this function if we can
    fn is_in_category(&self, category: event_category::DataType) -> bool;

    fn to_string(&self) -> String;

    fn get_name(&self) -> &str;

    fn get_description(&self) -> &str;
}

//The size of u8 was chosen only because it's the smallest size. Performance was not a factor in
//this decision. The size can be increased as needed later on.
pub mod event_type
{
    pub type DataType = u8;

    pub const NO:                  DataType = 0;

    pub const WINDOW_CLOSE:        DataType = 1;
    pub const WINDOW_RESIZE:       DataType = 2;
    pub const WINDOW_FOCUS:        DataType = 3;
    pub const WINDOW_LOST_FOCUS:   DataType = 4;
    pub const WINDOW_MOVED:        DataType = 5;
    
    pub const APP_TICK:            DataType = 6;
    pub const APP_UPDATE:          DataType = 7;
    pub const APP_RENDER:          DataType = 8;

    pub const KEY_PRESSED:         DataType = 9;
    pub const KEY_RELEASED:        DataType = 10;

    pub const MouseButtonPressed:  DataType = 11;
    pub const MouseButtonReleased: DataType = 12;
    pub const MouseMoved:          DataType = 13;
    pub const MouseScrolled:       DataType = 14;
}

// This is useful for filtering
// Because we use it for filtering, and we're going to use bitwise filtering, the following
// constants are listed in binary, to make it easier to compare their bits visually. As of
// 2021-12-28, there is no need for more than 8 bits. 8 bits was chosen because it's the smallest
// size in Rust, so it requires fewer characters below. Performance was not a factor in this
// decision. The size can be increased as needed later on.
pub mod event_category
{
    pub type DataType = u8;

    pub const NO:                          DataType = 0b0000_0000;
    pub const EVENT_CATEGORY_APPLICATION:  DataType = 0b0000_0001;
    pub const EVENT_CATEGORY_INPUT:        DataType = 0b0000_0010;
    pub const EVENT_CATEGORY_KEYBOARD:     DataType = 0b0000_0011;
    pub const EVENT_CATEGORY_MOUSE:        DataType = 0b0000_0100;
    pub const EVENT_CATEGORY_MOUSE_BUTTON: DataType = 0b0000_0101;
}
