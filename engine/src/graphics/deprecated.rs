use core::ptr::{null, null_mut};
/*
// According to MicroSoft docs, signed and unsigned integers are 4 bytes
type c_int = i32;
type c_uint = u32;
type UINT = c_uint;

// According to Microsoft docs, HANDLE is:
// typedef PVOID HANDLE;
// And PVOID is a pointer to any type:
// typedef void * PVOID;
type PVOID = *mut core::ffi::c_void;
type HANDLE = PVOID;

// According to Microsoft docs, HINSTANCE is a handle to an instance; the base address of the
// module in memory.
type HINSTANCE = HANDLE;
type HMODULE = HINSTANCE;

type HICON = HANDLE;
type HCURSOR = HICON;
type HBRUSH = HANDLE;
type wchar_t = u16;
type WCHAR = wchar_t;
type LPARAM = LONG_PTR;
type LPCWSTR = *const WCHAR;
type HWND = HANDLE;

// LONG_PTR and UINT_PTR are, respectively, the signed and unsigned address sizes for whatever
// system we might be on. Rust determines those for us, so we don't need to specify them here, just
// refer to the built in data type.
type LONG_PTR = isize;
type UINT_PTR = usize;
type LRESULT = LONG_PTR;
type WPARAM = UINT_PTR;

type WNDPROC = Option<
    unsafe extern "system" fn (
        hwnd: HWND,
        uMsg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> LRESULT,
>;
*/
/*
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
*/
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

//unsafe_impl_default_zeroed!(WNDCLASSW);

pub unsafe extern "system"
fn dummy_window_procedure(hwnd: HWND,
                          uMsg: UINT,
                          wParam: WPARAM,
                          lParam: LPARAM,
                          ) -> LRESULT
{
    unimplemented!();
}
/*
#[link(name = "Kernel32")]
extern "system"
{
    /// [`GetModuleHandleW`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)
    pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;
}
*/
/*
pub fn wide_null(s: &str) -> Vec<u16>
{
    s.encode_utf16().chain(Some(0)).collect()
}
*/
/*
type c_ushort = u16;
type WORD = c_ushort;
type ATOM = WORD;
*/
/*
#[link(name = "User32")]
extern "system"
{
    /// [`RegisterClassW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassw)
    pub fn RegisterClassW(lpWndClass: *const WNDCLASSW) -> ATOM;
}

type c_ulong = u32;
type DWORD = c_ulong;
*/
/*
#[link(name = "Kernel32")]
extern "system"
{
    /// [`GetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
    pub fn GetLastError() -> DWORD;
}

type HMENU = HANDLE;
type LPVOID = *mut core::ffi::c_void;
#[link(name = "User32")]
extern "system"
{
    /// [`CreateWindowExW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw)
    pub fn CreateWindowExW(dwExStyle: DWORD,
                           lpClassName: LPCWSTR,
                           lpWindowName: LPCWSTR,
                           dwStyle: DWORD,
                           X: c_int,
                           Y: c_int,
                           nWidth: c_int,
                           nHeight: c_int,
                           hWndParent: HWND,
                           hMenu: HMENU,
                           hInstance: HINSTANCE,
                           lParam: LPVOID,
                           ) -> HWND;
}
*/
/*
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
type BOOL = c_int;
#[link(name = "User32")]
extern "system"
{
    /// [`ShowWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)
    pub fn ShowWindow(hWnd: HWND, nCmdShow: c_int) -> BOOL;
}

#[link(name = "User32")]
extern "system"
{
    /// [`DefWindowProcW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-defwindowprocw)
    pub fn DefWindowProcW(hWnd: HWND,
                          Msg: UINT,
                          wParam: WPARAM,
                          lParam: LPARAM,
                          ) -> LRESULT;
}
*/
/*
type LONG = c_long;
type c_long = i32;
#[repr(C)]
pub struct POINT
{
    x: LONG,
    y: LONG,
}
unsafe_impl_default_zeroed!(POINT);

#[repr(C)]
pub struct MSG
{
    hwnd: HWND,
    message: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
    time: DWORD,
    pt: POINT,
    lPrivate: DWORD,
}
unsafe_impl_default_zeroed!(MSG);

type LPMSG = *mut MSG;
type CPMSG = *const MSG;
*/
/*
#[link(name = "User32")]
extern "system"
{
    /// [`GetMessageW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagew)
    pub fn GetMessageW(lpMsg: LPMSG,
                       hWnd: HWND,
                       wMsgFilterMin: UINT,
                       wMsgFilterMax: UINT,
                       ) -> BOOL;
}

#[link(name = "User32")]
extern "system"
{
    /// [`TranslateMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translatemessage)
    pub fn TranslateMessage(lpMsg: CPMSG) -> BOOL;
    /// [`DispatchMessageW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dispatchmessagew)
    pub fn DispatchMessageW(lpMsg: CPMSG) -> LRESULT;
}
*/
/*
pub const WM_CLOSE: u32 = 0x0010;
pub const WM_DESTROY: u32 = 0x0002;
*/
/*
#[link(name = "User32")]
extern "system"
{
    /// [`DestroyWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroywindow)
    pub fn DestroyWindow(hWnd: HWND) -> BOOL;
    /// [`PostQuitMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postquitmessage)
    pub fn PostQuitMessage(nExitCode: c_int);
}

*/

pub unsafe extern "system"
fn window_procedure(hWnd: HWND,
                    Msg: UINT,
                    wParam: WPARAM,
                    lParam: LPARAM
                    ) -> LRESULT
{
    match Msg
    {
        WM_CLOSE => drop(DestroyWindow(hWnd)),
        WM_DESTROY => PostQuitMessage(0),
        WM_PAINT =>
        {
            let mut ps = PAINTSTRUCT::default();
//            ps.rcPaint.left = 0;
//            ps.rcPaint.right = 100;
//            ps.rcPaint.top - 0;
//            ps.rcPaint.bottom = 100;
            let hdc = BeginPaint(hWnd, &mut ps);
            let success = FillRect(hdc, &ps.rcPaint, (COLOR_WINDOW + 1) as HBRUSH);
            println!("success is {:?}", success);
            EndPaint(hWnd, &ps);
        }
        _ => return DefWindowProcW(hWnd, Msg, wParam, lParam),
    }
    0
}
/*
#[link(name = "User32")]
extern "system"
{
    /// [`LoadCursorW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw)
    pub fn LoadCursorW(hInstance: HINSTANCE,
                       lpCursorName: LPCWSTR,
                       ) -> HCURSOR;
}

type LPWSTR = *mut WCHAR;
type ULONG_PTR = usize;
/// [`MAKEINTRESOURCEW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-makeintresourcew)
pub const fn MAKEINTRESOURCEW(i: WORD) -> LPWSTR
{
    i as ULONG_PTR as LPWSTR
}
pub const IDC_ARROW: LPCWSTR = MAKEINTRESOURCEW(32512);

pub const WM_PAINT: u32 = 0x000F;

type HDC = HANDLE;
type BYTE = u8;
*/

/*
#[repr(C)]
pub struct PAINTSTRUCT
{
    hdc: HDC,
    fErase: BOOL,
    rcPaint: RECT,
    fRestore: BOOL,
    fIncUpdate: BOOL,
    rgbReserved: [BYTE; 32],
}
unsafe_impl_default_zeroed!(PAINTSTRUCT);

#[repr(C)]
pub struct RECT
{
    left: LONG,
    top: LONG,
    right: LONG,
    bottom: LONG,
}
unsafe_impl_default_zeroed!(RECT);
*/
/*
type LPPAINTSTRUCT = *mut PAINTSTRUCT;

#[link(name = "User32")]
extern "system"
{
    /// [`BeginPaint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-beginpaint)
    pub fn BeginPaint(hWnd: HWND,
                      lpPaint: LPPAINTSTRUCT
                      ) -> HDC;
    /// [`FillRect`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-fillrect)
    pub fn FillRect(hDC: HDC,
                    lprc: *const RECT,
                    hbr: HBRUSH
                    ) -> c_int;
    /// [`EndPaint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-endpaint)
    pub fn EndPaint(hWnd: HWND,
                    lpPaint: *const PAINTSTRUCT
                    ) -> BOOL;
}

pub const COLOR_WINDOW: u32 = 5;
//pub const COLOR_WINDOW: u32 = 0x00FFFFFF;
*/
