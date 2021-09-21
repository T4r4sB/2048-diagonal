use winapi::shared:: windef::*;
use winapi::um::wingdi::*;
use winapi::um::winuser::*;
use winapi::um::winnt::LONG;
use winapi::um::winnt::HANDLE;
use winapi::shared::minwindef::DWORD;
use winapi::ctypes::c_void;

use application::image::*;

pub struct DIBSection {
  handle: HBITMAP,
  dc: HDC,
  memory: *mut c_void,
  size: ImageSize,
}

impl DIBSection {
  pub fn new(size: ImageSize) -> Self {
    let bitmap_info = BITMAPINFO {
      bmiHeader: BITMAPINFOHEADER {
        biSize: std::mem::size_of::<BITMAPINFOHEADER>() as DWORD,
        biWidth: size.0 as LONG,
        biHeight: -(size.1 as LONG),
        biPlanes: 1,
        biBitCount: 32,
        biCompression: BI_RGB,
        biSizeImage: 0,
        biXPelsPerMeter: 0,
        biYPelsPerMeter: 0,
        biClrUsed: 0,
        biClrImportant: 0,
      },
      bmiColors: [
        RGBQUAD {
          rgbBlue: 0,
          rgbGreen: 0,
          rgbRed: 0,
          rgbReserved: 0,
        }
      ]
    };

    unsafe {
      let screen_dc = GetDC(0 as HWND);
      let dc = CreateCompatibleDC(screen_dc);
      let mut memory = 0 as *mut c_void;
      let handle = CreateDIBSection(
        dc, 
        &bitmap_info,
        DIB_RGB_COLORS,
        &mut memory,
        0 as HANDLE,
        0
      );

      SelectObject(dc, handle as HGDIOBJ);
      ReleaseDC(0 as HWND, screen_dc);
      Self {handle, dc,  memory,  size,}
    }
  }
}

impl<'i> DIBSection {
  pub fn get_size(&self) -> ImageSize {
    self.size
  }

  pub unsafe fn get_dc(&self) -> HDC {
    self.dc
  }

  pub fn as_view(&self) -> ImageView<u32> {
    unsafe { ImageView::from_raw(std::mem::transmute(self.memory), self.size, self.size.0) }
  }

  pub fn as_view_mut(&mut self) -> ImageViewMut<u32> {
    unsafe { ImageViewMut::from_raw(std::mem::transmute(self.memory), self.size, self.size.0) }
  }
}

impl Drop for DIBSection {
  fn drop(&mut self) {
    unsafe {
      DeleteObject(self.handle as HGDIOBJ);
      DeleteDC(self.dc);
    }
  }
}
