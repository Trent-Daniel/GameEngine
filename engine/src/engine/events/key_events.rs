pub mod key_pressed_event;
pub mod key_released_event;

type KeyCodeType = i32;

pub trait KeyEvent
{
    fn get_key_code(&self) -> KeyCodeType;
}
