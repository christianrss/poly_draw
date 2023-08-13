use windows::{
    core::*,
    Win32::Foundation::*,
    //Win32::Graphics::Gdi::ValidateRect,
    Win32::System::LibraryLoader::GetModuleHandleA,
    Win32::UI::WindowsAndMessaging::*,
    Win32::Graphics::Gdi::*
};

use std::mem::transmute;
use std::mem::MaybeUninit;

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

    fn on_paint(&mut self, hdc: HDC) {}
    
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

    unsafe{ SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2) };
    let _gdiiplus = GdiPlus::startup(None, None).unwrap();

    let instance = unsafe { GetModuleHandleA(None)? };


    debug_assert!(instance.0 != 0);

    let window_class = s!("window");

    let wc = WNDCLASSEXA {
        cbSize: std::mem::size_of::<WNDCLASSEXA>() as u32,
        hCursor: unsafe { LoadCursorW(None, IDC_ARROW)? },
        hInstance: instance,
        lpszClassName: window_class,
        style: CS_HREDRAW | CS_VREDRAW | CS_OWNDC,
        lpfnWndProc: Some(wndproc::<P>),
        cbClsExtra: 0,
        cbWndExtra: 0,
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

        fn on_paint(&mut self, hdc: HDC) {
            (|| -> Result<()>  {
                let mut graphics:Graphics = Graphics::from_hdc(hdc)?;
                let mut pen = Pen::new(&Color::from(RED), 1.0)?;
                let last_pos = 
                    graphics
                        .with_pen(&mut pen)
                        .draw_line((20.0, 50.0), (200.0, 50.0))?
                        .current_pos();
                Ok(())
            })()
            .unwrap();


        }

    }
}

fn polyd_wndproc<P: PolyDraw>(poly: &mut P, window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> bool {

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
            let hdc: HDC;
            let mut ps: PAINTSTRUCT = PAINTSTRUCT::default();
            hdc = unsafe{ BeginPaint(window,   &mut ps) };
            poly.on_paint(hdc);
            unsafe{ EndPaint(window, &ps) };
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

pub type Status = ctypes::c_int;
pub type Unit = ctypes::c_int;
pub type REAL = f32;
pub type ULONG_PTR = usize;
pub type UINT32 = ctypes::c_uint;
pub type PVOID = *mut ctypes::c_void;
pub const NULL: PVOID = 0 as PVOID;
pub type CHAR = ctypes::c_char;
// x,y
pub type Point = (REAL, REAL);
pub type ARGB = DWORD;
pub const Status_Ok: Status = 0;
pub use self::{ Status as GpStatus, Unit as GpUnit };
pub const BLACK: u32 = 0xff000000;
pub const RED: u32 = 0xffff0000;
pub type Result<T> = core::result::Result<T, Error>;

macro_rules! DECLARE_HANDLE {
    ($name:ident, $inner:ident) => {
        pub enum $inner {}
        pub type $name = *mut $inner;
    };
}

DECLARE_HANDLE!{DPI_AWARENESS_CONTEXT, DPI_AWARENESS_CONTEXT__}
pub const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2: DPI_AWARENESS_CONTEXT
    = -2isize as DPI_AWARENESS_CONTEXT;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GpGraphics {
    pub _address: u8
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GpPen {
    pub _address: u8
}
extern "C" {
    #[link_name = "\u{1}GdipCreateFromHDC"]
    pub fn GdipCreateFromHDC(hdc: HDC, graphics: *mut *mut GpGraphics) -> GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawLine"]
    pub fn GdipDrawLine(
        graphics: *mut GpGraphics,
        pen: *mut GpPen,
        x1: REAL,
        y1: REAL,
        x2: REAL,
        y2: REAL
        
    ) -> GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreatePen1"]
    pub fn GdipCreatePen1(color: ARGB, width: REAL, unit: GpUnit, pen: *mut *mut GpPen)
        -> GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPenColor"]
    pub fn GdipGetPenColor(pen: *mut GpPen, argb: *mut ARGB) -> GpStatus;
}
pub type DebugEventLevel = ctypes::c_int;
pub type DebugEventProc = Option<unsafe extern "C" fn(level: DebugEventLevel, message: *mut CHAR)>;
pub type NotificationHookProc = Option<unsafe extern "C" fn(token: *mut ULONG_PTR) -> Status>;
pub type NotificationUnhookProc = Option<unsafe extern "C" fn(token: ULONG_PTR)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct  GdiplusStartupInput {
    pub GdiplusVersion: UINT32,
    pub DebugEventCallback: DebugEventProc,
    pub SupressBackgroundThread: BOOL,
    pub SupressExternalCodecs: BOOL
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GdiplusStartupOutput {
    pub NotificationHook: NotificationHookProc,
    pub NotificationUnhook: NotificationUnhookProc,
}
extern "C" {
    #[link_name = "\u{1}GdiplusStartup"]
    pub fn GdiplusStartup(
        token: *mut ULONG_PTR,
        input: *const GdiplusStartupInput,
        output: *mut GdiplusStartupOutput,
    ) -> Status;
}
extern "C" {
    #[link_name = "\u{1}GdiplusShutdown"]
    pub fn GdiplusShutdown(token: ULONG_PTR);
}

pub struct Graphics {
    graphics: *mut GpGraphics,
}

#[macro_export]
macro_rules! return_iferror{
    ($code:expr) => {{
        let res = unsafe{$code};
        if res!= Status_Ok {
            //return Err(crate::Error::from(res));
        }
    }};
}

impl Graphics {
    pub(crate) fn graphics(&self) -> *mut GpGraphics {
        self.graphics
    }

    pub fn from_hdc(hdc: HDC) -> Result<Self> {
        let mut graphics = MaybeUninit::uninit();
        
        return_iferror!(GdipCreateFromHDC(hdc, graphics.as_mut_ptr()));

        let graphics = unsafe { graphics.assume_init() };

        Ok(Self{ graphics })
    }

    pub fn with_pen<'a>(&'a mut self, pen: &'a mut Pen) -> WithPen<'a> {
        WithPen::new(self, pen)
    }
}

pub struct WithPen<'a> {
    graphics: &'a mut Graphics,
    pen: &'a mut Pen,
    current_pos: Point,
}
impl<'a> WithPen<'a> {
    pub fn new(graphics: &'a mut Graphics, pen: &'a mut Pen) -> Self {
        Self {
            graphics,
            pen,
            current_pos: (0.0, 0.0)
        }
    }

    pub fn current_pos(&self) -> Point {
        self.current_pos
    }

    pub fn draw_line(&mut self, from: Point, to: Point) -> Result<&mut Self> {
        return_iferror!(GdipDrawLine(
            self.graphics.graphics(),
            self.pen.pen(),
            from.0,
            from.1,
            to.0,
            to.1
        ));
        Ok(self)
    }
}

pub struct Pen {
    pen: *mut GpPen,
}
impl Pen {
    pub(crate) fn pen(&self) -> *mut GpPen {
        self.pen
    }

    pub fn new(color: &Color, width: REAL) -> Result<Self> {
        let mut pen = MaybeUninit::uninit();

        return_iferror!(GdipCreatePen1(color.argb(), width, 0, pen.as_mut_ptr()));

        Ok(Self {
            pen: unsafe { pen.assume_init() }
        })
    }

    pub fn color(&self) -> Result<Color> {
        let mut argb = 0;
        return_iferror!(GdipGetPenColor(self.pen, &mut argb));
        Ok(Color::from(argb))
    }
}

type AlphaColorTuple = (u8, u8, u8, u8);
type ColorTuple = (u8, u8, u8, u8);

pub struct Color {
    color: u32,
}
impl Color {
    pub fn argb(&self) -> u32 {
        self.color
    }
}
impl From<u32> for Color {
    fn from(color: u32) -> Self {
        Self { color }
    }
}

pub struct GdiPlus {
    token: usize,
    input: Option<Box<GdiplusStartupInput>>,
    output: Option<Box<GdiplusStartupOutput>>
}
impl GdiPlus {
    pub fn startup(
        input: Option<Box<GdiplusStartupInput>>,
        mut output: Option<Box<GdiplusStartupOutput>>
    ) -> Result<GdiPlus> {
        let mut token = MaybeUninit::uninit();
        let input = input.unwrap_or_else(|| {
            Box::new(GdiplusStartupInput {
                DebugEventCallback: None,
                GdiplusVersion: 1,
                SupressBackgroundThread: FALSE,
                SupressExternalCodecs: FALSE,
            })
        });

        if let Some(ref mut output) = output {
            return_iferror!(GdiplusStartup(
                token.as_mut_ptr(),
                input.as_ref(),
                output.as_mut()
            ));
        } else {
            return_iferror!(GdiplusStartup(
                token.as_mut_ptr(),
                input.as_ref(),
                NULL as  _
            ));
        }

        let token = unsafe { token.assume_init() };

        Ok(GdiPlus {
            token,
            input: Some(input),
            output
        })
    }
    pub fn shutdown(&self) {
        unsafe {
            GdiplusShutdown(self.token);
        }
    }
}
impl Drop for GdiPlus {
    fn drop(&mut self) {
        self.shutdown();
    }
}

extern "system" {
    pub fn SetProcessDpiAwarenessContext(value: DPI_AWARENESS_CONTEXT) -> BOOL;
}

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
                polyd_wndproc(unsafe { p.as_mut() }, window, message, wparam, lparam)
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