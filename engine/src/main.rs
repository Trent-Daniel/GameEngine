// AKA entrypoint
mod engine;
mod sandbox_app;
mod graphics;

use crate::engine::CreateApplication;
use crate::engine::utils::logging::Logger;
use crate::engine::utils::logging::log_levels;

use crate::graphics::windows;

pub static LOGGER: Logger = Logger::new("application", log_levels::FATAL, "info_log.txt");

fn main()
{
    println!("Starting program");
    engine::print();
    

    windows::run();



/*

    
    let hInstance = unsafe { GetModuleHandleW(core::ptr::null()) };
    let sample_window_class_wn = wide_null("Sample Window Class");

    let mut wc = windows::WNDCLASSW::default();

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



*/






    let sb = sandbox_app::Sandbox::create_application();
    sb.run();
    println!("Ending program");
}
