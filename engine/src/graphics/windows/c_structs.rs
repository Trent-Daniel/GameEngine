use crate::graphics::windows::utils::unsafe_impl_default_zeroed;
use crate::graphics::windows::types::*;

#[repr(C)]
pub struct POINT
{
    pub x: LONG,
    pub y: LONG,
}
unsafe_impl_default_zeroed!(POINT);

#[repr(C)]
pub struct MSG
{
    pub hwnd: HWND,
    pub message: UINT,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: DWORD,
    pub pt: POINT,
    pub lPrivate: DWORD,
}
unsafe_impl_default_zeroed!(MSG);

#[repr(C)]
pub struct PAINTSTRUCT
{
    pub hdc: HDC,
    pub fErase: BOOL,
    pub rcPaint: RECT,
    pub fRestore: BOOL,
    pub fIncUpdate: BOOL,
    pub rgbReserved: [BYTE; 32],
}
unsafe_impl_default_zeroed!(PAINTSTRUCT);

#[repr(C)]
pub struct RECT
{
    pub left: LONG,
    pub top: LONG,
    pub right: LONG,
    pub bottom: LONG,
}
unsafe_impl_default_zeroed!(RECT);
#[repr(C)]
pub struct WNDCLASSW
{
  pub style: UINT,
  pub lpfnWndProc: WNDPROC,
  pub cbClsExtra: c_int,
  pub cbWndExtra: c_int,
  pub hInstance: HINSTANCE,
  pub hIcon: HICON,
  pub hCursor: HCURSOR,
  pub hbrBackground: HBRUSH,
  pub lpszMenuName: LPCWSTR,
  pub lpszClassName: LPCWSTR,
}
unsafe_impl_default_zeroed!(WNDCLASSW);

