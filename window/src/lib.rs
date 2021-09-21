use core::mem::MaybeUninit;

use winapi::shared::minwindef::*;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::*;
use winapi::um::wingdi::*;
use winapi::shared::windef::*;

mod dib_section;
mod wide_strings;
mod font_loader;

use crate::dib_section::DIBSection;
use crate::wide_strings::WideStringManager;
use application::image::ImageViewMut;
use application::font::FontFactory;

#[derive(PartialEq, Eq)]
pub struct KeyCode(usize);

pub const KEY_SPACE: KeyCode = KeyCode(VK_SPACE as usize);
pub const KEY_NUMPAD1: KeyCode = KeyCode(VK_NUMPAD1 as usize);
pub const KEY_NUMPAD2: KeyCode = KeyCode(VK_NUMPAD2 as usize);
pub const KEY_NUMPAD3: KeyCode = KeyCode(VK_NUMPAD3 as usize);
pub const KEY_NUMPAD4: KeyCode = KeyCode(VK_NUMPAD4 as usize);
pub const KEY_NUMPAD5: KeyCode = KeyCode(VK_NUMPAD5 as usize);
pub const KEY_NUMPAD6: KeyCode = KeyCode(VK_NUMPAD6 as usize);
pub const KEY_NUMPAD7: KeyCode = KeyCode(VK_NUMPAD7 as usize);
pub const KEY_NUMPAD8: KeyCode = KeyCode(VK_NUMPAD8 as usize);
pub const KEY_NUMPAD9: KeyCode = KeyCode(VK_NUMPAD9 as usize);

pub const KEY_A: KeyCode = KeyCode('A' as usize);
pub const KEY_B: KeyCode = KeyCode('B' as usize);
pub const KEY_C: KeyCode = KeyCode('C' as usize);
pub const KEY_D: KeyCode = KeyCode('D' as usize);
pub const KEY_E: KeyCode = KeyCode('E' as usize);
pub const KEY_F: KeyCode = KeyCode('F' as usize);
pub const KEY_G: KeyCode = KeyCode('G' as usize);
pub const KEY_H: KeyCode = KeyCode('H' as usize);
pub const KEY_I: KeyCode = KeyCode('I' as usize);
pub const KEY_J: KeyCode = KeyCode('J' as usize);
pub const KEY_K: KeyCode = KeyCode('K' as usize);
pub const KEY_L: KeyCode = KeyCode('L' as usize);
pub const KEY_M: KeyCode = KeyCode('M' as usize);
pub const KEY_N: KeyCode = KeyCode('N' as usize);
pub const KEY_O: KeyCode = KeyCode('O' as usize);
pub const KEY_P: KeyCode = KeyCode('P' as usize);
pub const KEY_Q: KeyCode = KeyCode('Q' as usize);
pub const KEY_R: KeyCode = KeyCode('R' as usize);
pub const KEY_S: KeyCode = KeyCode('S' as usize);
pub const KEY_T: KeyCode = KeyCode('T' as usize);
pub const KEY_U: KeyCode = KeyCode('U' as usize);
pub const KEY_V: KeyCode = KeyCode('V' as usize);
pub const KEY_W: KeyCode = KeyCode('W' as usize);
pub const KEY_X: KeyCode = KeyCode('X' as usize);
pub const KEY_Y: KeyCode = KeyCode('Y' as usize);
pub const KEY_Z: KeyCode = KeyCode('Z' as usize);

pub type AppFontFactory = FontFactory<font_loader::GDIFontLoader>;

pub trait Application {
  fn on_key_down(
    &mut self,
    code: KeyCode,
    must_repaint: &mut bool,
    must_close: &mut bool
  );

  fn on_paint(&mut self, destination: &mut ImageViewMut<u32>, font_factory: &mut AppFontFactory);
}

struct Context<'i, AppImpl: Application> {
  application: &'i mut AppImpl,
  buffer: Option<DIBSection>,
  font_factory: AppFontFactory,
}

pub fn get_client_rect(hwnd: HWND) -> RECT {
  unsafe {
    let mut rect = MaybeUninit::uninit();
    GetClientRect(hwnd, rect.as_mut_ptr());
    rect.assume_init()
  }
}

pub unsafe extern "system" fn window_proc<AppImpl: Application> (
  hwnd: HWND,
  msg: UINT,
  wparam: WPARAM,
  lparam: LPARAM,
) -> LRESULT {
  let get_context = || -> &mut Context<AppImpl> {
      std::mem::transmute(GetWindowLongPtrW(hwnd, GWL_USERDATA))
  };

  match msg {
    WM_KEYDOWN => {
      let mut must_repaint = false;
      let mut must_close = false;
      get_context().application.on_key_down(
        KeyCode(wparam as usize), &mut must_repaint, &mut must_close
      );
      if must_repaint {
        InvalidateRect(hwnd, 0 as *const RECT, FALSE);
      }
    }

    WM_PAINT => {
      let mut paint_struct = MaybeUninit::uninit();
      let rect = get_client_rect(hwnd);
      let rect_size = ((rect.right - rect.left) as usize, (rect.bottom - rect.top) as usize);
      let context = get_context();
      let buffer = &mut context.buffer;
      if buffer.is_none() || buffer.as_ref().unwrap().get_size() != rect_size {
        *buffer = Some(DIBSection::new(rect_size));
      }

      let buffer = buffer.as_mut().unwrap();

      context.application.on_paint(&mut buffer.as_view_mut(), &mut context.font_factory);
      let hdc = BeginPaint(hwnd, paint_struct.as_mut_ptr());
      BitBlt(hdc, 0, 0, rect_size.0 as i32, rect_size.1 as i32, buffer.get_dc(), 0, 0, SRCCOPY);
      EndPaint(hwnd, paint_struct.as_mut_ptr());
    }

    WM_DESTROY => {
        PostQuitMessage(0);
    }
    _ => { return DefWindowProcW(hwnd, msg, wparam, lparam); }
  }
  return 0;
}

fn create_window<AppImpl: Application>(context: *mut Context<AppImpl>) -> HWND {
  let mut wide_strings = WideStringManager::new();

  unsafe {
    let hinstance = GetModuleHandleW( 0 as *const u16 );
    let wnd_class = WNDCLASSW {
      style : CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
      lpfnWndProc : Some( window_proc::<AppImpl> ),
      hInstance : hinstance,
      lpszClassName : wide_strings.from("MyClass"),
      cbClsExtra : 0,
      cbWndExtra : 0,
      hIcon: 0 as HICON,
      hCursor: LoadCursorW(0 as HINSTANCE, IDC_ARROW),
      hbrBackground: 0 as HBRUSH,
      lpszMenuName: 0 as *const u16,
    };
    RegisterClassW(&wnd_class);

    let hwnd = CreateWindowExW(
      0,                                  // dwExStyle
      wide_strings.from("MyClass"),       // class we registered
      wide_strings.from("Заголовок"),     // title
      WS_OVERLAPPEDWINDOW | WS_VISIBLE,   // dwStyle
      CW_USEDEFAULT,
      CW_USEDEFAULT,
      CW_USEDEFAULT,
      CW_USEDEFAULT,  // size and position
      0 as HWND,      // hWndParent
      0 as HMENU,     // hMenu
      hinstance,      // hInstance
      0 as LPVOID );  // lpParam

    SetWindowLongPtrW(hwnd, GWL_USERDATA, std::mem::transmute(context));
    hwnd
  }
}

fn handle_message( window : HWND ) -> bool {
  unsafe {
    let mut msg = MaybeUninit::<MSG>::uninit();
    if GetMessageW( msg.as_mut_ptr(), window, 0, 0 ) > 0 {
      TranslateMessage( msg.as_ptr() );
      DispatchMessageW( msg.as_ptr() );
      true
    } else {
      false
    }
  }
}

pub fn run_application(application: &mut impl Application) {
  let mut context = Context {application, buffer: None, font_factory: AppFontFactory::new()};
  let window = create_window(&mut context);
  loop {
    if !handle_message(window) {
      break;
    }
  }
}
