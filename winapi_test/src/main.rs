#![windows_subsystem = "windows"]

extern crate window;
extern crate application;

use rand::Rng;
use rand::rngs::ThreadRng;
use rand::thread_rng;
use smallvec::SmallVec;
use std::cmp::{min, max};

const SIZE : usize = 4;

#[derive(Debug, Default)]
struct Field {
  numbers: [[i32; SIZE]; SIZE],
  game_over: bool,
}

impl Field {
  fn new() -> Self {
    return Field::default();
  }

  fn new_game(&mut self, rng: &mut impl Rng) {
    *self = Self::new();
    self.add_item(rng);
    self.add_item(rng);
  }

  fn fail(&self) -> bool {
    for i in 0 .. self.numbers.len() {
      for j in 0 .. self.numbers[i].len() {
        if self.numbers[i][j] == 0 {
          return false;
        }

        for i2 in max(i, 1) - 1 .. min(i + 2, self.numbers.len()) {
          for j2 in max(j, 1) - 1 .. min(j + 2, self.numbers[i2].len()) {
            if (i != i2 || j != j2) && self.numbers[i][j] == self.numbers[i2][j2] {
              return false;
            }
          }
        }
      }
    }

    return true;
  }

  fn add_item(&mut self, rng: &mut impl Rng) {
    let mut empty_fields = SmallVec::<[(usize, usize); SIZE * SIZE]>::new();
    for i in 0 .. self.numbers.len() {
      for j in 0 .. self.numbers[i].len() {
        if self.numbers[i][j] == 0 {
          empty_fields.push((i, j));
        }
      }
    }

    if empty_fields.len() == 0 {
      self.game_over = true;
    }

    let f = rng.gen_range(0 .. empty_fields.len());
    let y = empty_fields[f].0;
    let x = empty_fields[f].1;
    let start_numbers = [2, 4];
    let new_number = start_numbers[rng.gen_range(0 .. start_numbers.len())];
    self.numbers[y][x] = new_number;

    if self.fail() {
      self.game_over = true;
    }
  }

  fn push_dir(&mut self, dx: i32, dy: i32) -> bool {
    let mut lines = SmallVec::<
      [SmallVec::<[(usize, usize); SIZE]>; SIZE * 2]
    >::new();

    fn valid(x: i32, y: i32) -> bool {
      x >= 0 && x < SIZE as i32 && y >= 0 && y < SIZE as i32
    }

    fn good_number(i: i32) -> bool {
      i & (i - 1) == 0
    }

    for y in 0 .. self.numbers.len() as i32 {
      for x in 0 .. self.numbers[y as usize].len() as i32 {
        if !valid(x + dx, y + dy) {
          lines.push(Default::default());
          let last_line = lines.last_mut().unwrap();
          let mut cur_x = x;
          let mut cur_y = y;
          while valid(cur_x, cur_y) {
            last_line.push((cur_y as usize, cur_x as usize));
            cur_x -= dx;
            cur_y -= dy;
          }
        }
      }
    }

    let mut result = false;

    for l in &lines {
      let mut i = 0;
      let mut j = 1;
      while i < l.len() {
        while j == i || (j < l.len() && self.numbers[l[j].0][l[j].1] == 0) {
          j += 1;
        }

        if j == l.len() {
          break;
        }

        if self.numbers[l[i].0][l[i].1] == 0 {
          result = true;
          self.numbers[l[i].0][l[i].1] = self.numbers[l[j].0][l[j].1];
          self.numbers[l[j].0][l[j].1] = 0;
          continue;
        }

        let possible_number = 
          self.numbers[l[j].0][l[j].1] + self.numbers[l[i].0][l[i].1];

        if good_number(possible_number) {
          result = true;
          self.numbers[l[i].0][l[i].1] = possible_number;
          self.numbers[l[j].0][l[j].1] = 0;
        }

        i += 1;
      }
    }

    result
  }
}

#[derive(Default)]
struct Application2048 {
  field: Field,
  rng: ThreadRng,
}

impl Application2048 {
  fn new() -> Self {
    let mut rng =  thread_rng();
    let mut field = Field::new();
    field.new_game(&mut rng);
    Application2048 {rng, field,}
  }
}

impl window::Application for Application2048 {
  fn on_key_down(
    &mut self,
    key_code: window::KeyCode,
    must_repaint: &mut bool,
    _must_close: &mut bool
  ) {
    match key_code {
      window::KEY_SPACE => {
        if self.field.game_over {
          self.field.new_game(&mut self.rng);
          *must_repaint = true;
        }
      },
      window::KEY_NUMPAD1 | window::KEY_Z  => {
        if self.field.push_dir(-1, 1) {
          self.field.add_item(&mut self.rng); 
          *must_repaint = true;
        }
      }
      window::KEY_NUMPAD2 | window::KEY_X => {
        if self.field.push_dir(0, 1) {
          self.field.add_item(&mut self.rng); 
          *must_repaint = true;
        }
      }
      window::KEY_NUMPAD3 | window::KEY_C => {
        if self.field.push_dir(1, 1) {
          self.field.add_item(&mut self.rng); 
          *must_repaint = true;
        }
      }
      window::KEY_NUMPAD4 | window::KEY_A => {
        if self.field.push_dir(-1, 0) {
          self.field.add_item(&mut self.rng); 
          *must_repaint = true;
        }
      }
      window::KEY_NUMPAD6 | window::KEY_D => {
        if self.field.push_dir(1, 0) {
          self.field.add_item(&mut self.rng); 
          *must_repaint = true;
        }
      }
      window::KEY_NUMPAD7 | window::KEY_Q => {
        if self.field.push_dir(-1, -1) {
          self.field.add_item(&mut self.rng); 
          *must_repaint = true;
        }
      }
      window::KEY_NUMPAD8 | window::KEY_W => {
        if self.field.push_dir(0, -1) {
          self.field.add_item(&mut self.rng);
          *must_repaint = true;
        }
      }
      window::KEY_NUMPAD9 | window::KEY_E => {
        if self.field.push_dir(1, -1) {
          self.field.add_item(&mut self.rng);
          *must_repaint = true;
        }
      }
      _ => {}
    }
  }

  fn on_paint(
    &mut self,
    dst: &mut application::image::ImageViewMut<u32>,
    font_factory: &mut window::AppFontFactory,
  ) {
    dst.fill(|p| *p = 0);
    let size = dst.get_size();
    let font_size = size.1 / 16;
    let font_black = font_factory.new_font(
      "Arial", font_size, 0,
      application::font::TextLayoutHorizontal::MIDDLE,
      application::font::TextLayoutVertical::MIDDLE
    );
    let font_white = font_factory.new_font(
      "Arial", font_size, 0x00FFFFFF,
      application::font::TextLayoutHorizontal::MIDDLE,
      application::font::TextLayoutVertical::MIDDLE
    );

    let colors = vec![
      0x000060, 0x006060, 0x006000, 0x606000, 0x603000, 0x600000, 0x600060, 0x6000C0,
      0x0000C0, 0x0060C0, 0x00C0C0, 0x00C060, 0x00C000, 0x60C000, 0xC0C000, 0xC06000, 0xC00000,
    ];

    for y in 0 .. SIZE {
      for x in 0 .. SIZE {
        let n = self.field.numbers[y][x];
        let color = if n == 0 {
          0
        } else {
          let mut n = n;
          let mut log2: usize = 0;
          while n > 2 {
            log2 += 1;
            n /= 2;
          } colors[log2 % colors.len()]
        };
        let mut w = dst.window_mut(
          (size.0 * (x * 32 + 33) / ((SIZE + 2) * 32), size.1 * (y * 32 + 33) / ((SIZE + 2) * 32)),
          (size.0 * (x * 32 + 63) / ((SIZE + 2) * 32), size.1 * (y * 32 + 63) / ((SIZE + 2) * 32))
        );
        w.fill(|p| *p = color);

        let center = (w.get_size().0 / 2, w.get_size().1 / 2);
        let shift = size.1 / 256 + 1;

        if n > 0 {
          font_black.draw(&format!("{}", n), (center.0 + shift, center.1 + shift), &mut w);
          font_white.draw(&format!("{}", n), center, &mut w);
        }
      }
    }

    if self.field.game_over {
      let font_size = size.1 / 8;
      let font_black = font_factory.new_font(
        "Arial", font_size, 0,
        application::font::TextLayoutHorizontal::MIDDLE,
        application::font::TextLayoutVertical::MIDDLE
      );
      let font_white = font_factory.new_font(
        "Arial", font_size, 0x00FFFFFF,
        application::font::TextLayoutHorizontal::MIDDLE,
        application::font::TextLayoutVertical::MIDDLE
      );

      dst.fill(|p| *p = (*p & 0xFCFCFCFC) >> 2);
      let center = (dst.get_size().0 / 2, dst.get_size().1 * 2 / 5);
      let shift = size.1 / 128 + 1;
      font_black.draw("Game over", (center.0 + shift, center.1 + shift), dst);
      font_white.draw("Game over", center, dst);

      let center = (dst.get_size().0 / 2, dst.get_size().1 * 3 / 5);
      let shift = size.1 / 128 + 1;
      font_black.draw("Press SPACE", (center.0 + shift, center.1 + shift), dst);
      font_white.draw("Press SPACE", center, dst);
    } else {
      let center = (dst.get_size().0 / 12, dst.get_size().1 / 12);
      font_white.draw("Q", center, dst);
      let center = (dst.get_size().0 / 2, dst.get_size().1 / 12);
      font_white.draw("W", center, dst);
      let center = (dst.get_size().0 * 11 / 12, dst.get_size().1 / 12);
      font_white.draw("E", center, dst);
      let center = (dst.get_size().0 / 12, dst.get_size().1 / 2);
      font_white.draw("A", center, dst);
      let center = (dst.get_size().0 * 11 / 12, dst.get_size().1 / 2);
      font_white.draw("D", center, dst);
      let center = (dst.get_size().0 / 12, dst.get_size().1 * 11  / 12);
      font_white.draw("Z", center, dst);
      let center = (dst.get_size().0 / 2, dst.get_size().1 * 11  / 12);
      font_white.draw("X", center, dst);
      let center = (dst.get_size().0 * 11 / 12, dst.get_size().1 * 11  / 12);
      font_white.draw("C", center, dst);
    }
  }
}

fn main() {
  window::run_application(&mut Application2048::new())
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn fiend_is_fail() {
    let mut field = Field::new();
    field.numbers = [[4, 32, 2, 4], [2, 32, 16, 4], [8, 4, 16, 32], [2, 4, 2, 8]];
    assert!(!field.fail());
  }
}