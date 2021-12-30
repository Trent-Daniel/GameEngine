use crate::engine::application::Application;
use crate::engine::CreateApplication;

pub
struct Sandbox {}

impl Application for Sandbox {}

impl CreateApplication for Sandbox
{
    fn create_application() -> Box<dyn Application>
    {
        return Box::new(Sandbox{});
    }
}
