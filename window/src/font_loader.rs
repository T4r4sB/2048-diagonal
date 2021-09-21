use application::image::*;
use application::font::*;
use std::collections::HashMap;
use std::mem::MaybeUninit;

use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::um::wingdi::*;
use winapi::um::winuser::*;

use crate::DIBSection;
use crate::WideStringManager;

pub struct GDIFontLoader {
}

impl FontLoader for GDIFontLoader {
  fn load_glyphs(font_name: &str, font_size: usize, code_from: u32, code_to: u32) -> FontImages {
    let mut dst = DIBSection::new((1,1));
    let mut max_size = dst.get_size();
    let mut wide_strings = WideStringManager::new();
    let mut result = HashMap::new();

    unsafe {
      let font = CreateFontW(
        font_size as i32, 0, 0, 0,
        FW_NORMAL,
        FALSE as u32, FALSE as u32, FALSE as u32,
        OEM_CHARSET, OUT_RASTER_PRECIS,
        CLIP_DEFAULT_PRECIS, NONANTIALIASED_QUALITY, DEFAULT_PITCH | FF_DONTCARE, wide_strings.from(font_name));

      let mut dc = dst.get_dc();
      SelectObject(dc, font as HGDIOBJ);

      for code in code_from .. code_to {
        if let Some(c) = char::from_u32(code) {
          let s = c.to_string();
          let ws = wide_strings.from(&s);
          let mut rect = MaybeUninit::uninit();
          GetTextExtentPoint32W(dc, ws, 1, rect.as_mut_ptr());
          let rect = rect.assume_init();
          let rect = (rect.cx as usize, rect.cy as usize);

          max_size = (std::cmp::max(rect.0, max_size.0), std::cmp::max(rect.1, max_size.1));

          if dst.get_size() != max_size {
            dst = DIBSection::new(max_size);
            dc = dst.get_dc();
            SelectObject(dc, font as HGDIOBJ);
          }

          let mut text_rect = RECT{left: 0, top: 0, right: max_size.0 as i32, bottom: max_size.1 as i32};
          dst.as_view_mut().fill(|p| *p = 0xFFFFFF);
          DrawTextW(dc, ws, 1, &mut text_rect, DT_SINGLELINE | DT_LEFT | DT_TOP);
          let mut new_image = Image::new(rect);
          new_image.as_view_mut().draw(dst.as_view(), (0, 0), |d, s| *d = *s == 0);
          result.insert(c, Glyph::NoAA(new_image));
        }
      }

      DeleteObject(font as HGDIOBJ);
    }

    result
  }
}