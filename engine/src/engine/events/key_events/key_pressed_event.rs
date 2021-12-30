use crate::engine::utils::macros::impl_event;
use crate::engine::events::Event;
use crate::engine::events::event_category;
use crate::engine::events::event_type;
use crate::engine::events::key_events::KeyCodeType;
use crate::engine::events::key_events::KeyEvent;

pub struct KeyPressedEvent
{
    event_type: event_type::DataType,
    event_category_flags: event_category::DataType,
    name: &'static str,
    description: String,
    key_code: KeyCodeType,
    // Might want to add a property and getter/setter for continuous presses to the same key, i.e.
    // the continuous press that would result in 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa' if the 'a' key
    // was pressed. Could be a boolean
}

impl KeyPressedEvent
{
    pub fn new(key_code: KeyCodeType) -> Self
    {
        KeyPressedEvent
        {
            event_type: event_type::KEY_PRESSED,
            event_category_flags: (event_category::EVENT_CATEGORY_KEYBOARD | event_category::EVENT_CATEGORY_INPUT),
            name: "KeyPressedEvent",
            key_code: key_code,
            description: ["Key ",
                          &(key_code.to_string()),
                          " was pressed."]
                              .join("").clone(),
        }
    }
}

impl KeyEvent for KeyPressedEvent
{
    fn get_key_code(&self) -> KeyCodeType
    {
        self.key_code
    }
}

impl Event for KeyPressedEvent
{
    impl_event!();
}
