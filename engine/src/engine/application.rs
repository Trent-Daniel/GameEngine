use crate::LOGGER;
use crate::engine::utils::macros::get_this_function_name;
use crate::engine::status_codes::status_codes;
use crate::engine::events::key_events::key_pressed_event::KeyPressedEvent;
use crate::engine::events::Event;
use crate::engine::events::key_events::KeyEvent;
use crate::engine::events::event_category;

pub trait Application
{
    fn run(&self) -> ()
    {
        println!("Starting loop");
        LOGGER.fatal("Static Fatal message from Application::run()", get_this_function_name!(), status_codes::OK);
        
        // The following is a demo of the event system.
        let e = KeyPressedEvent::new(5 as i32);
        if e.is_in_category(event_category::EVENT_CATEGORY_INPUT)
        {
            println!("Event {} is of category input", e.get_name());
        }
        else
        {
            println!("Event {} is not of category input", e.get_name());
        }
        if e.is_in_category(event_category::EVENT_CATEGORY_MOUSE)
        {
            println!("Event {} is of category mouse", e.get_name());
        }
        else
        {
            println!("Event {} is not of category mouse", e.get_name());
        }
        //loop {}
    }
}
