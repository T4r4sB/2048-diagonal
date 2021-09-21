use crate::image::*;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::cmp::max;
use std::cell::RefCell;

pub enum Glyph {
  NoAA (Image<bool>),
  AA (Image<u8>),
  TT (Image<u32>),
}

pub type FontImages =  HashMap<char, Glyph>;

pub trait FontLoader {
  fn load_glyphs(font_name: &str, font_size: usize, code_from: u32, code_to: u32) -> FontImages;
}

pub type FontLibrary = Vec<FontImages>;

pub struct Font<'i, L: FontLoader> {
  name: String,
  size: usize,
  color: u32,
  layout_horizontal: TextLayoutHorizontal,
  layout_vertical: TextLayoutVertical,
  id: usize,
  library: &'i RefCell<FontLibrary>,
  loader: PhantomData<L>
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum TextLayoutVertical {
  TOP, MIDDLE, BOTTOM
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum TextLayoutHorizontal {
  LEFT, MIDDLE, RIGHT
}

impl<'i, L: FontLoader> Font<'i, L> {
  fn new(
    name: String,
    size: usize,
    color: u32,
    layout_horizontal: TextLayoutHorizontal,
    layout_vertical: TextLayoutVertical,
    id: usize,
    library: &'i RefCell<FontLibrary>,
  ) -> Self {
    Self {
      name,
      size,
      color,
      layout_horizontal,
      layout_vertical,
      id,
      library,
      loader: PhantomData,
    }
  }

  fn get_char(&self, c: char, images: &'i mut FontImages) -> Option<&'i Glyph> {
    if !images.contains_key(&c) {
      let code = c as u32;
      let additional_glyphs = L::load_glyphs(&self.name, self.size, code & !0xFF, (code & !0xFF) + 0x100);
      images.extend(additional_glyphs);
    }

    images.get(&c)
  }

  fn get_size_with(&self, text: &str, images: &mut FontImages) -> ImageSize {
    let mut result = (0, 0);
    for c in text.chars() {
      match self.get_char(c, images) {
        Some(Glyph::NoAA(img)) => {
          let sz = img.get_size();
          result.0 += sz.0;
          result.1 = max(result.1, sz.1);
        },
        _ => {}
      }
    }

    result
  }

  pub fn get_size(&self, text: &str) -> ImageSize {
    let images = &mut self.library.borrow_mut()[self.id];
    self.get_size_with(text, images)
  }

  pub fn draw(&self, text: &str, position: ImageSize, dst: &mut ImageViewMut<u32>) {
    let images = &mut self.library.borrow_mut()[self.id];
    let size = self.get_size_with(text, images);
    let mut position = (
      match self.layout_horizontal {
        TextLayoutHorizontal::LEFT => position.0 as isize,
        TextLayoutHorizontal::MIDDLE => position.0 as isize - size.0 as isize / 2,
        TextLayoutHorizontal::RIGHT => position.0 as isize - size.0 as isize,
      },
      match self.layout_vertical {
        TextLayoutVertical::TOP => position.1 as isize,
        TextLayoutVertical::MIDDLE => position.1 as isize - size.1 as isize / 2,
        TextLayoutVertical::BOTTOM => position.1 as isize - size.1 as isize,
      }
    );

    let color = self.color;

    for c in text.chars() {
      match self.get_char(c, images) {
        Some(Glyph::NoAA(img)) => {
          dst.draw(img.as_view(), position, |dst, src| if *src {*dst = color;});
          position.0 += img.get_size().0 as isize;
        },
        _ => {}
      }
    }
  }
}

pub struct FontFactory<L: FontLoader> {
  mapping: RefCell<HashMap<(String, usize), usize>>,
  library: RefCell<FontLibrary>,
  loader: PhantomData<L>,
}

impl<L: FontLoader> FontFactory<L> {
  pub fn new() -> Self {
    Self {
      mapping: Default::default(),
      library: Default::default(),
      loader: PhantomData,
    }
  }

  pub fn new_font(
    &self,
    name: impl Into<String> + Clone,
    size: usize,
    color: u32,
    layout_horizontal: TextLayoutHorizontal,
    layout_vertical: TextLayoutVertical,
  ) -> Font<L> {
    let mapping = &mut self.mapping.borrow_mut();
    let id = if let Some(id) = mapping.get(&(name.clone().into(), size)) {
      *id
    } else {
      let mut library = self.library.borrow_mut();
      library.push(Default::default());
      mapping.insert((name.clone().into(), size), library.len() - 1);
      library.len() - 1
    };

    Font::new(name.into(), size, color, layout_horizontal, layout_vertical, id, &self.library)
  }
}
