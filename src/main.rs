mod bindings {
    windows::include_bindings!();
}

use bindings::Windows::Win32::{
    Graphics::Gdi::*,
    System::SystemServices::*,
    UI::{DisplayDevices::*, WindowsAndMessaging::*},
};
use windows::HRESULT;

struct Window {}

impl Window {
    fn new() -> Window {
        Self {}
    }

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
            WM_PAINT => {
                let mut ps = PAINTSTRUCT {
                    hdc: HDC(0),
                    fErase: BOOL(0),
                    rcPaint: RECT {
                        left: 0,
                        top: 0,
                        right: 400,
                        bottom: 400,
                    },
                    fRestore: BOOL(0),
                    fIncUpdate: BOOL(0),
                    rgbReserved: [0; 32],
                };
                let hdc = BeginPaint(hwnd, &mut ps);

                let color: u32 = COLOR_WINDOW.0 + 1;
                print!("{}", color);
                FillRect(hdc, &mut ps.rcPaint, HBRUSH(color as isize));

                EndPaint(hwnd, &mut ps);
                LRESULT(0)
            }
            _ => DefWindowProcW(hwnd, u_msg, w_param, l_param),
        }
    }

    fn run(&mut self) -> windows::Result<()> {
        unsafe {
            let h_instance = GetModuleHandleW(PWSTR(&mut 0));

            let class_name = PWSTR(b"Sample Window Class\0".as_ptr() as _);

            let wc = WNDCLASSW {
                lpfnWndProc: Some(Self::window_proc),
                hInstance: h_instance,
                lpszClassName: class_name,
                ..Default::default()
            };

            RegisterClassW(&wc as *const WNDCLASSW);

            let hwnd = CreateWindowExW(
                Default::default(),
                class_name,
                "Learn to Program Windows",
                WS_OVERLAPPEDWINDOW,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                None,
                None,
                Some(h_instance),
                self as *mut _ as _,
            );

            if hwnd.is_null() {
                return Err(windows::Error::fast_error(HRESULT(0x1012002)));
            }

            ShowWindow(hwnd, SW_SHOW);

            let mut msg = MSG::default();

            while GetMessageW(&mut msg, None, 0, 0).as_bool() {
                TranslateMessage(&mut msg);
                DispatchMessageW(&mut msg);
            }
        }

        Ok(())
    }
}

fn main() -> windows::Result<()> {
    let mut window = Window::new();
    window.run()
}
