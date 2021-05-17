mod bindings {
    windows::include_bindings!();
}

use bindings::Windows::Win32::{
    System::Com::*, System::SystemServices::*, UI::Shell::*, UI::WindowsAndMessaging::*,
};
use libc::c_void;
use std::ptr::null_mut;

struct Window {}

impl Window {
    fn new() -> Self {
        Self {}
    }

    fn win_main(&mut self) -> windows::Result<()> {
        unsafe {
            let hr = CoInitializeEx(
                null_mut(),
                COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE,
            );

            if hr.is_ok() {
                let file_open: IFileOpenDialog =
                    CoCreateInstance(&FileOpenDialog, None, CLSCTX_ALL).unwrap();

                let hr = file_open.Show(None);

                if hr.is_ok() {
                    let mut item: Option<IShellItem> = None;
                    let hr = file_open.GetResult(&mut item);

                    if hr.is_ok() {
                        let mut file_path = PWSTR(b"\0".as_ptr() as _);
                        let hr = item
                            .unwrap()
                            .GetDisplayName(SIGDN_FILESYSPATH, &mut file_path);

                        if hr.is_ok() {
                            MessageBoxW(
                                None,
                                file_path,
                                PWSTR(b"File Path\0".as_ptr() as _),
                                MB_OK,
                            );
                            CoTaskMemFree(file_path.0 as *mut c_void);
                        }
                    }
                }
                CoUninitialize();
            }
        }

        Ok(())
    }
}

fn main() -> windows::Result<()> {
    let mut window = Window::new();
    window.win_main()
}
