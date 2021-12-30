use crate::graphics::windows::types::*;

pub const WS_OVERLAPPED: u32 = 0x00000000;
pub const WS_CAPTION: u32 = 0x00C00000;
pub const WS_SYSMENU: u32 = 0x00080000;
pub const WS_THICKFRAME: u32 = 0x00040000;
pub const WS_MINIMIZEBOX: u32 = 0x00020000;
pub const WS_MAXIMIZEBOX: u32 = 0x00010000;
pub const WS_OVERLAPPEDWINDOW: u32 = WS_OVERLAPPED
    | WS_CAPTION
    | WS_SYSMENU
    | WS_THICKFRAME
    | WS_MINIMIZEBOX
    | WS_MAXIMIZEBOX;
pub const CW_USEDEFAULT: c_int = 0x80000000_u32 as c_int;

pub const SW_SHOW: c_int = 5;
pub const WM_CLOSE: u32 = 0x0010;
pub const WM_DESTROY: u32 = 0x0002;

/// [`MAKEINTRESOURCEW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-makeintresourcew)
pub const fn MAKEINTRESOURCEW(i: WORD) -> LPWSTR
{
    i as ULONG_PTR as LPWSTR
}

pub const IDC_ARROW: LPCWSTR = MAKEINTRESOURCEW(32512);

pub const WM_PAINT: u32 = 0x000F;
pub const COLOR_WINDOW: u32 = 5;
//pub const COLOR_WINDOW: u32 = 0x00FFFFFF;

