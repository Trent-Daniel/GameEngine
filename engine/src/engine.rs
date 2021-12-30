pub mod application;
pub mod utils;
pub mod events;
pub mod status_codes;

//use application::Application;

pub
fn print() -> ()
{
    println!("Printing from engine");
}

// Look into changing dyn to impl trait for better performance
pub
trait CreateApplication
{
    fn create_application() -> Box<dyn application::Application>;
}

