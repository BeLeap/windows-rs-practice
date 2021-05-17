mod bindings {
    windows::include_bindings!();
}

use bindings::Windows::Win32::{
    System::Com::*, System::SystemServices::*, UI::Shell::*, UI::WindowsAndMessaging::*,
};
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
                    let mut some_item: Option<IShellItem> = None;
                    let hr = file_open.GetResult(&mut some_item);

                    if hr.is_ok() {
                        let mut file_path = PWSTR(b"\0".as_ptr() as _);
                        let hr = some_item
                            .unwrap()
                            .GetDisplayName(SIGDN_FILESYSPATH, &mut file_path);

                        if hr.is_ok() {
                            MessageBoxW(
                                None,
                                file_path,
                                PWSTR(b"File Path\0".as_ptr() as _),
                                MB_OK,
                            );
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
