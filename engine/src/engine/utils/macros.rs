// To be used for getting the current function's name, only for debugging or logging purposes, not
// programmatic ones.
macro_rules! get_this_function_name
{
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str
        {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
    }}
}

// We should try t figure out a way to make the two get_name macros a single one, as long as it
// doesn't affect performance

// To be used for getting the current struct's name, only for debugging or logging purposes, not
// programmatic ones.
macro_rules! get_this_struct_name
{
    ($cls: expr) =>
    {{
        fn type_name_of<T>(_: T) -> &'static str
        {
            std::any::type_name::<T>()
        }
        let name = type_name_of($cls);
        &name[..name.len() - 3]
    }}
}

macro_rules! impl_event
{
    () =>
    {

        fn get_event_type(&self) -> event_type::DataType
        {
            self.event_type
        }

        fn get_event_category_flags(&self) -> event_category::DataType
        {
            self.event_category_flags
        }

        // We should try to improve the performance of this function if we can
        fn is_in_category(&self, category: event_category::DataType) -> bool
        {
            (self.get_event_category_flags() & category) != 0
        }

        fn to_string(&self) -> String
        {
            ["Event Type: ", self.get_name(), "\n",
             "Description: ", self.get_description(), "\n"]
                 .join("").clone()
        }

        fn get_name(&self) -> &str
        {
            self.name
        }

        fn get_description(&self) -> &str
        {
            &(self.description)
        }

    }
}

pub(crate) use impl_event;
pub(crate) use get_this_function_name;
pub(crate) use get_this_struct_name;
