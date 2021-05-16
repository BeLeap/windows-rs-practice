fn main() {
    windows::build!(
        Windows::Win32::UI::WindowsAndMessaging::{
            HWND,
            WPARAM,
            LPARAM,
            WM_DESTROY,
            PostQuitMessage,
            DefWindowProcA,
            WNDCLASSA,
            RegisterClassA,
            CreateWindowExA,
            CW_USEDEFAULT,
            ShowWindow,
            MSG,
            GetMessageA,
            TranslateMessage,
            DispatchMessageA,
            WM_PAINT,
            COLOR_WINDOW,
            WS_OVERLAPPEDWINDOW,
            SW_SHOW,
            WM_CLOSE,
            MessageBoxA,
            MB_OKCANCEL,
            DestroyWindow,
        },
        Windows::Win32::System::SystemServices::{
            LRESULT,
            HINSTANCE,
            PWSTR,
            PSTR,
            GetModuleHandleA,
            BOOL,
        },
        Windows::Win32::Graphics::Gdi::{
            HBRUSH,
            PAINTSTRUCT,
            HDC,
            BeginPaint,
            FillRect,
            EndPaint,
        },
    );
}