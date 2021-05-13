mod bindings {
    windows::include_bindings!();
}

use bindings::Windows::Win32::{
    Graphics::Gdi::*, System::SystemServices::*, UI::WindowsAndMessaging::*,
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
                let mut ps = PAINTSTRUCT::default();
                let hdc = BeginPaint(hwnd, &mut ps);

                let color: u32 = COLOR_WINDOW.0 + 1;
                FillRect(hdc, &mut ps.rcPaint, HBRUSH(color as isize));

                EndPaint(hwnd, &mut ps);
                LRESULT(0)
            }
            WM_CLOSE => {
                if MessageBoxA(
                    hwnd,
                    PSTR(b"Really quit?".as_ptr() as _),
                    PSTR(b"My application".as_ptr() as _),
                    MB_OKCANCEL,
                )
                .0 == 1
                {
                    DestroyWindow(hwnd);
                }
                LRESULT(0)
            }
            _ => DefWindowProcA(hwnd, u_msg, w_param, l_param),
        }
    }

    fn run(&mut self) -> windows::Result<()> {
        unsafe {
            let h_instance = GetModuleHandleA(PSTR(&mut 0));

            let class_name = PSTR(b"Sample Window Class\0".as_ptr() as _);

            let wc = WNDCLASSA {
                lpfnWndProc: Some(Self::window_proc),
                hInstance: h_instance,
                lpszClassName: class_name,
                ..Default::default()
            };

            RegisterClassA(&wc as *const WNDCLASSA);

            let hwnd = CreateWindowExA(
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

            while GetMessageA(&mut msg, None, 0, 0).as_bool() {
                TranslateMessage(&mut msg);
                DispatchMessageA(&mut msg);
            }
        }

        Ok(())
    }
}

fn main() -> windows::Result<()> {
    let mut window = Window::new();
    window.run()
}
