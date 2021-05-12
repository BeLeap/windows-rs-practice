mod bindings {
    windows::include_bindings!();
}

extern crate libc;

use std::mem::size_of;

use bindings::{Windows::Win32::SystemServices::*, Windows::Win32::WindowsAndMessaging::*};
use libc::c_void;
use std::ptr::null_mut;
use windows::HRESULT;

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    uMsg: u32,
    wParam: WPARAM,
    lParam: LPARAM,
) -> LRESULT {
    match uMsg {
        WM_DESTROY => {
            unsafe {
                PostQuitMessage(0);
            }
            LRESULT(0)
        }
        _ => unsafe { DefWindowProcA(hwnd, uMsg, wParam, lParam) },
    }
}

fn main() -> windows::Result<()> {
    unsafe {
        let hInstance = HINSTANCE(GetModuleHandleA(PSTR(&mut 0)));

        let CLASS_NAME = PSTR(b"Sample Window Class\0".as_ptr() as _);

        let wc = libc::malloc(size_of::<WNDCLASSA>()) as *mut WNDCLASSA;
        (*wc).lpfnWndProc = Some(window_proc);
        (*wc).hInstance = hInstance;
        (*wc).lpszClassName = CLASS_NAME;

        RegisterClassA(wc as *const WNDCLASSA);

        let hwnd = CreateWindowExA(
            WINDOW_EX_STYLE::WS_EX_LEFT,
            CLASS_NAME,
            "Learn to Program Windows",
            WINDOW_STYLE::WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            None,
            None,
            Some(hInstance),
            null_mut(),
        );

        println!("{:?}", hwnd.is_null());

        if hwnd.is_null() {
            return Err(windows::Error::fast_error(HRESULT(0x1012001)));
        }

        ShowWindow(hwnd, SHOW_WINDOW_CMD::SW_SHOW);

        let msg = libc::malloc(size_of::<MSG>()) as *mut MSG;

        while GetMessageA(msg, None, 0, 0).as_bool() {
            TranslateMessage(msg);
            DispatchMessageA(msg);
        }
    }

    Ok(())
}
