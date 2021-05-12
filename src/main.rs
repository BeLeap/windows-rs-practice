mod bindings {
    windows::include_bindings!();
}

use bindings::{
    Windows::Win32::Gdi::*, Windows::Win32::MenusAndResources::*,
    Windows::Win32::SystemServices::*, Windows::Win32::WindowsAndMessaging::*,
};
use libc;
use std::mem::size_of;
use std::ptr::null_mut;
use windows::HRESULT;

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    u_msg: u32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    match u_msg {
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, u_msg, w_param, l_param),
    }
}

fn main() -> windows::Result<()> {
    unsafe {
        let h_instance = HINSTANCE(GetModuleHandleW(PWSTR(&mut 0)));

        let class_name = PWSTR(b"Sample Window Class\0".as_ptr() as _);

        let wc = WNDCLASSW {
            style: WNDCLASS_STYLES::CS_OWNDC
                | WNDCLASS_STYLES::CS_HREDRAW
                | WNDCLASS_STYLES::CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            hInstance: h_instance,
            lpszClassName: class_name,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hIcon: HICON(0),
            hCursor: HCURSOR(0),
            hbrBackground: HBRUSH(0),
            lpszMenuName: PWSTR(&mut 0),
        };

        RegisterClassW(&wc as *const WNDCLASSW);

        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE::WS_EX_LEFT,
            class_name,
            "Learn to Program Windows",
            WINDOW_STYLE::WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            None,
            None,
            Some(h_instance),
            null_mut(),
        );

        if hwnd.is_null() {
            return Err(windows::Error::fast_error(HRESULT(0x1012001)));
        }

        ShowWindow(hwnd, SHOW_WINDOW_CMD::SW_SHOW);

        let msg = libc::malloc(size_of::<MSG>()) as *mut MSG;

        while GetMessageW(msg, None, 0, 0).as_bool() {
            TranslateMessage(msg);
            DispatchMessageW(msg);
        }
    }

    Ok(())
}
