pub fn wide_null(s: &str) -> Vec<u16>
{
    s.encode_utf16().chain(Some(0)).collect()
}

macro_rules! unsafe_impl_default_zeroed
{
        ($t:ty) =>
        {
        impl Default for $t
        {
            #[inline]
            #[must_use]
            fn default() -> Self
            {
                unsafe { core::mem::zeroed() }
            }
        }
    };
}

pub(crate) use unsafe_impl_default_zeroed;
