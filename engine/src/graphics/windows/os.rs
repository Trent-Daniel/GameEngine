use crate::graphics::windows::types::*;
use crate::graphics::windows::c_structs::*;

#[link(name = "Kernel32")]
extern "system"
{
    /// [`GetModuleHandleW`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)
    pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;
    /// [`GetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
    pub fn GetLastError() -> DWORD;
}

#[link(name = "User32")]
extern "system"
{
    /// [`RegisterClassW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassw)
    pub fn RegisterClassW(lpWndClass: *const WNDCLASSW) -> ATOM;
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
    /// [`ShowWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)
    pub fn ShowWindow(hWnd: HWND, nCmdShow: c_int) -> BOOL;
    /// [`DefWindowProcW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-defwindowprocw)
    pub fn DefWindowProcW(hWnd: HWND,
                          Msg: UINT,
                          wParam: WPARAM,
                          lParam: LPARAM,
                          ) -> LRESULT;
    /// [`GetMessageW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagew)
    pub fn GetMessageW(lpMsg: LPMSG,
                       hWnd: HWND,
                       wMsgFilterMin: UINT,
                       wMsgFilterMax: UINT,
                       ) -> BOOL;
    /// [`TranslateMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translatemessage)
    pub fn TranslateMessage(lpMsg: CPMSG) -> BOOL;
    /// [`DispatchMessageW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dispatchmessagew)
    pub fn DispatchMessageW(lpMsg: CPMSG) -> LRESULT;
    /// [`DestroyWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroywindow)
    pub fn DestroyWindow(hWnd: HWND) -> BOOL;
    /// [`PostQuitMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postquitmessage)
    pub fn PostQuitMessage(nExitCode: c_int);
    /// [`LoadCursorW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw)
    pub fn LoadCursorW(hInstance: HINSTANCE,
                       lpCursorName: LPCWSTR,
                       ) -> HCURSOR;
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

#[link(name = "Gdi32")]
extern "system"
{
    ///https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createsolidbrush
    pub fn CreateSolidBrush(color: COLORREF) -> HBRUSH;

}
