use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::Graphics::Gdi::ValidateRect,
    Win32::System::LibraryLoader::GetModuleHandleA,
    Win32::UI::WindowsAndMessaging::*
};

use std::mem::transmute;

pub mod ctypes {
    #[cfg(feature = "std")]
    pub use std::os::raw::c_void;
    #[cfg(not(feature = "std"))]
    pub enum c_void {}
    pub type c_char = i8;
    pub type c_schar = i8;
    pub type c_uchar = u8;
    pub type c_short = i16;
    pub type c_ushort = u16;
    pub type c_int = i32;
    pub type c_uint = u32;
    pub type c_long = i32;
    pub type c_ulong = u32;
    pub type c_longlong = i64;
    pub type c_ulonglong = u64;
    pub type c_float = f32;
    pub type c_double = f64;
    pub type __int8 = i8;
    pub type __uint8 = u8;
    pub type __int16 = i16;
    pub type __uint16 = u16;
    pub type __int32 = i32;
    pub type __uint32 = u32;
    pub type __int64 = i64;
    pub type __uint64 = u64;
    pub type wchar_t = u16;
}

//use ctypes::{c_int, c_short, c_ushort, c_ulong};

pub type DWORD = ctypes::c_ulong;
pub type WORD = ctypes::c_ushort;

#[inline]
pub fn HIWORD(l: DWORD) -> WORD {
    ((l >> 16) & 0xffff) as WORD
}

#[inline]
pub fn LOWORD(l: DWORD) -> WORD {
    (l & 0xffff) as WORD
}

#[inline]
pub fn GET_X_LPARAM(lp: LPARAM) -> ctypes::c_int {
    LOWORD(lp.0 as u32) as ctypes::c_short as ctypes::c_int
}
#[inline]
pub fn GET_Y_LPARAM(lp: LPARAM) -> ctypes::c_int {
    HIWORD(lp.0 as u32) as ctypes::c_short as ctypes::c_int
}

trait PolyDraw {
    fn new (command_line: &PolyDrawCommandLine) -> Result<Self>
    where
        Self: Sized;

    fn bind_to_window(&mut self, hwnd: &HWND) -> Result<()>;

    fn on_key_up(&mut self, _key: u8) {}
    fn on_key_down(&mut self, _key: u8) {}
    fn on_mouse_l_button_down(&mut self, x: i32, y: i32) {}
    fn on_mouse_l_button_up(&mut self, x: i32, y: i32) {}
    fn on_mouse_move(&mut self, x: i32, y: i32) {}
    
    fn window_size(&self) -> (i32, i32) {
        (640, 480)
    }
}

#[derive(Clone)]
struct PolyDrawCommandLine {
    // example_parameter: bool
}

fn build_command_line() -> PolyDrawCommandLine {
    for arg in std::env::args() {
        // if arg.eq_ignore_ascii_case("-test") {
            // example_parameter = true;
        //}
    }

    PolyDrawCommandLine{ /*example_parameter */ }
}

fn run_polydraw<P>() -> Result<()>
where
    P: PolyDraw,
{
    let instance = unsafe { GetModuleHandleA(None)? };

    debug_assert!(instance.0 != 0);

    let window_class = s!("window");

    let wc = WNDCLASSEXA {
        cbSize: std::mem::size_of::<WNDCLASSEXA>() as u32,
        hCursor: unsafe { LoadCursorW(None, IDC_ARROW)? },
        hInstance: instance,
        lpszClassName: window_class,
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(wndproc::<P>),
        ..Default::default()
    };

    let command_line = build_command_line();
    let mut poly = P::new(&command_line)?; // hahahaha =/

    let atom = unsafe { RegisterClassExA(&wc) };
    debug_assert_ne!(atom, 0);

    let size = poly.window_size();

    let mut window_rect = RECT {
        left: 0,
        top: 0,
        right: size.0,
        bottom: size.1
    };

    unsafe { AdjustWindowRect(&mut window_rect, WS_OVERLAPPEDWINDOW, false) };

    let hwnd = unsafe {
        CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            window_class,
            s!("PolyDraw by Christian ZeroBit"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            window_rect.right - window_rect.left,
            window_rect.bottom - window_rect.top,
            None,
            None,
            instance,
            Some(&mut poly as *mut _ as _)
        )
    };

    poly.bind_to_window(&hwnd)?;
    unsafe { ShowWindow(hwnd, SW_SHOW) };

    loop {
        let mut message = MSG::default();
        if unsafe { PeekMessageA(&mut message, None, 0, 0, PM_REMOVE) }.into() {
            unsafe {
                TranslateMessage(&message);
                DispatchMessageA(&message);
            }
            if message.message == WM_QUIT {
                break;
            }
        }
    }

    Ok(())
}

mod polydraw_lines {
    use super::*;

    pub struct PolyD {

    }

    impl PolyDraw for PolyD {
        fn new (command_line: &PolyDrawCommandLine) -> Result<Self> {
            Ok(PolyD{})
        }

        fn bind_to_window(&mut self, hwnd: &HWND) -> Result<()> {
            let (width, height) = self.window_size();
            Ok(())
        }

        fn window_size(&self) -> (i32, i32) {
            (640,480)
        }

        fn on_key_down(&mut self, _key: u8) {
            println!("Pressed key {_key}");
        }

        fn on_mouse_l_button_down(&mut self, x: i32, y: i32) {
            println!("MOUSE BUTTON LEFT DOWN X: {x}, Y: {y}");
        }

        fn on_mouse_l_button_up(&mut self, x: i32, y: i32) {
            println!("MOUSE BUTTON LEFT UP X: {x}, Y: {y}");
        }

        fn on_mouse_move(&mut self, x: i32, y: i32) {
            println!("MOUSE MOVE X: {x}, Y: {y}");
        }

    }
}

fn polyd_wndproc<P: PolyDraw>(poly: &mut P, message: u32, wparam: WPARAM, lparam: LPARAM) -> bool {
    match message {
        WM_KEYDOWN => {
            poly.on_key_down(wparam.0 as u8);
            true
        },
        WM_KEYUP => {
            poly.on_key_up(wparam.0 as u8);
            true
        },
        WM_LBUTTONDOWN => {
            let mut x = GET_X_LPARAM(lparam);
            let mut y = GET_Y_LPARAM(lparam);
            poly.on_mouse_l_button_down(x, y);
            true
        },
        WM_LBUTTONUP => {
            let mut x = GET_X_LPARAM(lparam);
            let mut y = GET_Y_LPARAM(lparam);
            poly.on_mouse_l_button_up(x, y);
            true
        },
        WM_MOUSEMOVE => {
            let mut x = GET_X_LPARAM(lparam);
            let mut y = GET_Y_LPARAM(lparam);
            poly.on_mouse_move(x, y);
            true
        }
        WM_PAINT => {
            // poly.update();
            // poly.render();
            true
        }
        _ => false,
    }
}

 /*no macros 
 // https://github.com/microsoft/windows-rs/issues/1445
 // https://github.com/microsoft/windows-rs/issues/921
extern "system" fn get_x_lparam(lp: LPARAM) -> u8 {
    lp.0 as u8
}

extern "system" fn get_y_lparam(lp: LPARAM) -> u8 {
    lp.0 as u8
}*/

extern "system" fn wndproc<P: PolyDraw>(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM
) -> LRESULT {
    match message {
        WM_CREATE => {
            unsafe {
                let create_struct: &CREATESTRUCTA = transmute(lparam);
                SetWindowLongPtrA(window, GWLP_USERDATA, create_struct.lpCreateParams as _);
            }
            LRESULT::default()
        }
        WM_DESTROY => {
            println!("WM_DESTROY");
            unsafe { PostQuitMessage(0) };
            LRESULT::default()
        }
        _ => {
            let user_data = unsafe { GetWindowLongPtrA(window, GWLP_USERDATA) };
            let poly = std::ptr::NonNull::<P>::new(user_data as _);
            let handled = poly.map_or(false, |mut p| {
                polyd_wndproc(unsafe { p.as_mut() }, message, wparam, lparam)
            });

            if handled {
                LRESULT::default()
            } else {
                unsafe { DefWindowProcA(window, message, wparam, lparam) }
            }
        },
    }
}

fn main() -> Result<()> {
    run_polydraw::<polydraw_lines::PolyD>()?;
    Ok(())
}