use crate::image::*;

pub struct SizeConstraint {
  pub absolute: usize,
  pub relative: usize,
}

pub struct SizeConstraints(SizeConstraint, SizeConstraint);

pub trait GuiControl {
  fn get_size(&self)-> SizeConstraints;
  fn set_position(&mut self, left_top: ImageSize, right_bottom: ImageSize);
  fn on_draw(&self, dst: &mut ImageViewMut<u32>);
}

pub struct Container {
  size_constraints: SizeConstraints,
  size: ImageSize,
  children: Vec<usize>,
}
