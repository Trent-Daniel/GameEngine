pub mod utils;
pub mod os;
pub mod constants;
pub mod c_structs;
pub mod types;

use core::ptr::{null, null_mut};
use crate::graphics::windows::os::*;
use crate::graphics::windows::c_structs::*;
use crate::graphics::windows::types::*;
use crate::graphics::windows::constants::*;
use crate::graphics::windows::utils::*;

pub fn run()
{
    let hInstance = unsafe { GetModuleHandleW(core::ptr::null()) };
    let sample_window_class_wn = wide_null("Sample Window Class");

    let mut wc = WNDCLASSW::default();

    //wc.lpfnWndProc = Some(dummy_window_procedure);
    //wc.lpfnWndProc = Some(DefWindowProcW);
    wc.lpfnWndProc = Some(window_procedure);
    wc.hInstance = hInstance;
    wc.lpszClassName = sample_window_class_wn.as_ptr();
    wc.hCursor = unsafe { LoadCursorW(hInstance, IDC_ARROW) };
    let atom = unsafe { RegisterClassW(&wc) };
    if atom == 0
    {
        let last_error = unsafe { GetLastError() };
        panic!("Could not register the window class, error code: {}", last_error);
    }

    let sample_window_name_wn = wide_null("Sample Window Name");
    let hwnd = unsafe
    {
        CreateWindowExW(
            0,
            sample_window_class_wn.as_ptr(),
            sample_window_name_wn.as_ptr(),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            hInstance,
            core::ptr::null_mut(),
        )
    };

    if hwnd.is_null()
    {
        panic!("Failed to create a window.");
    }

    let _previously_visible = unsafe { ShowWindow(hwnd, SW_SHOW) };

    let mut msg = MSG::default();
    loop
    {
        let message_return = unsafe { GetMessageW(&mut msg, null_mut(), 0, 0) };
        if message_return == 0
        {
            break;
        }
        else if message_return == -1
        {
            let last_error = unsafe { GetLastError() };
            panic!("Error with `GetMessgeW`, error code: {}", last_error);
        }
        else
        {
            unsafe {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    }
}

pub unsafe extern "system"
fn dummy_window_procedure(hwnd: HWND,
                          uMsg: UINT,
                          wParam: WPARAM,
                          lParam: LPARAM,
                          ) -> LRESULT
{
    unimplemented!();
}

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
            let hdc = BeginPaint(hWnd, &mut ps);
            let success = FillRect(hdc, &ps.rcPaint, (COLOR_WINDOW + 1) as HBRUSH);
            println!("success is {:?}", success);
            EndPaint(hWnd, &ps);
        }
        _ => return DefWindowProcW(hWnd, Msg, wParam, lParam),
    }
    0
}
