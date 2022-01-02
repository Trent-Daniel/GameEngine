use crate::graphics::windows::c_structs::*;

pub type c_ushort = u16;
pub type WORD = c_ushort;
pub type ATOM = WORD;
pub type c_ulong = u32;
pub type DWORD = c_ulong;
pub type HMENU = HANDLE;
pub type LPVOID = *mut core::ffi::c_void;
pub type BOOL = c_int;
pub type LONG = c_long;
pub type c_long = i32;
pub type LPMSG = *mut MSG;
pub type CPMSG = *const MSG;
pub type LPWSTR = *mut WCHAR;
pub type ULONG_PTR = usize;
pub type HDC = HANDLE;
pub type BYTE = u8;
pub type LPPAINTSTRUCT = *mut PAINTSTRUCT;
// According to Microsoft docs, HANDLE is:
// typedef PVOID HANDLE;
// And PVOID is a pointer to any type:
// typedef void * PVOID;
pub type PVOID = *mut core::ffi::c_void;
pub type HANDLE = PVOID;

// According to Microsoft docs, HINSTANCE is a handle to an instance; the base address of the
// module in memory.
pub type HINSTANCE = HANDLE;
pub type HMODULE = HINSTANCE;

pub type HICON = HANDLE;
pub type HCURSOR = HICON;
pub type HBRUSH = HANDLE;
pub type wchar_t = u16;
pub type WCHAR = wchar_t;
pub type LPARAM = LONG_PTR;
pub type LPCWSTR = *const WCHAR;
pub type HWND = HANDLE;

// LONG_PTR and UINT_PTR are, respectively, the signed and unsigned address sizes for whatever
// system we might be on. Rust determines those for us, so we don't need to specify them here, just
// refer to the built in data type.
pub type LONG_PTR = isize;
pub type UINT_PTR = usize;
pub type LRESULT = LONG_PTR;
pub type WPARAM = UINT_PTR;

pub type WNDPROC = Option<
    unsafe extern "system" fn (
        hwnd: HWND,
        uMsg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> LRESULT,
>;
// According to MicroSoft docs, signed and unsigned integers are 4 bytes
pub type c_int = i32;
pub type c_uint = u32;
pub type UINT = c_uint;
pub type COLORREF = DWORD;
