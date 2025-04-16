use windows::Win32::{
    Foundation::{HWND, LPARAM, WPARAM},
    UI::{
        Input::Ime::{
            IMC_SETCONVERSIONMODE, IMC_SETOPENSTATUS, IME_CMODE_NATIVE, ImmGetDefaultIMEWnd,
        },
        WindowsAndMessaging::{GetForegroundWindow, SendMessageW, WM_IME_CONTROL},
    },
};
fn get_ime_status(hwnd: HWND) -> i32 {
    unsafe { SendMessageW(hwnd, WM_IME_CONTROL, Some(WPARAM(1)), Some(LPARAM(0))).0 as i32 }
}

fn set_ime(hwnd: HWND, ko: bool) {
    unsafe {
        SendMessageW(
            hwnd,
            WM_IME_CONTROL,
            Some(WPARAM(IMC_SETOPENSTATUS as usize)),
            Some(LPARAM(if ko { 1 } else { 0 })),
        );
        if ko {
            SendMessageW(
                hwnd,
                WM_IME_CONTROL,
                Some(WPARAM(IMC_SETCONVERSIONMODE as usize)),
                Some(LPARAM(IME_CMODE_NATIVE.0 as isize)), // 한글 모드로 설정
            );
        }
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!(
            "{}",
            get_ime_status(unsafe { ImmGetDefaultIMEWnd(GetForegroundWindow()) })
        );
        return;
    }

    let ko = args[1] == "1";
    unsafe {
        let hwnd = GetForegroundWindow();
        if !hwnd.is_invalid() {
            // toggle_ime(hwnd);
            let himc_default = ImmGetDefaultIMEWnd(hwnd);
            if !himc_default.is_invalid() {
                set_ime(himc_default, ko);
            }
        } else {
            println!("hwnd is invalid");
        }
    }
}
