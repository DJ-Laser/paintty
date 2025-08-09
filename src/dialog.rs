use std::io::{self, Stdout};

use crossterm::{
  cursor, queue,
  style::{self, Color, Stylize},
  terminal::WindowSize,
};

use crate::canvas::{Canvas, PaintTool};

pub struct Bound {
  top: u16,
  left: u16,

  bottom: u16,
  right: u16,
}

impl Bound {
  pub fn from_pos_size(pos: (u16, u16), size: (u16, u16)) -> Self {
    Self {
      top: pos.1,
      left: pos.0,

      bottom: pos.1 + size.1,
      right: pos.0 + size.0,
    }
  }

  pub fn contains(&self, pos: (u16, u16)) -> bool {
    return (self.left..self.right).contains(&pos.0) && (self.top..self.bottom).contains(&pos.1);
  }
}

pub struct DialogState {
  pub hidden: bool,
  toolbar_pos: (u16, u16),
  bounds: Vec<Bound>,
}

impl DialogState {
  pub fn new(terminal_size: &WindowSize) -> Self {
    let toolbar_pos = (2, terminal_size.rows - 4);

    Self {
      hidden: true,
      toolbar_pos,
      bounds: Vec::new(),
    }
  }

  pub fn bounds(&self) -> &Vec<Bound> {
    &self.bounds
  }

  pub fn render(&mut self, stdout: &mut Stdout, canvas: &Canvas) -> io::Result<()> {
    self.bounds.clear();

    if self.hidden {
      return Ok(());
    }

    let toolbar_size = (14, 3);
    self
      .bounds
      .push(Bound::from_pos_size(self.toolbar_pos, toolbar_size));
    draw_dialog(stdout, self.toolbar_pos, toolbar_size)?;
    let mut brush_color = Color::Reset;
    let mut bucket_color = Color::Reset;
    match canvas.current_tool() {
      PaintTool::Paintbrush => brush_color = Color::White,
      PaintTool::Bucket => bucket_color = Color::White,
    }

    queue!(
      stdout,
      cursor::MoveTo(self.toolbar_pos.0 + 1, self.toolbar_pos.1 + 1),
      style::Print(" "),
      style::Print("🖌️ ".on(brush_color)),
      style::Print("  "),
      style::Print("🪣".on(bucket_color)),
      style::Print("  "),
      style::Print("🎨")
    )?;

    Ok(())
  }

  pub fn interact(&mut self, pos: (u16, u16), canvas: &mut Canvas) {
    if pos.1 == self.toolbar_pos.1 + 1 {
      match pos.0 - self.toolbar_pos.0 {
        (2..=3) => canvas.set_tool(PaintTool::Paintbrush),
        (6..=7) => canvas.set_tool(PaintTool::Bucket),
        (10..=11) => todo!("palette"),
        _ => (),
      }
    }
  }
}

fn draw_dialog(stdout: &mut Stdout, position: (u16, u16), size: (u16, u16)) -> io::Result<()> {
  for y in position.1..(position.1 + size.1) {
    queue!(
      stdout,
      cursor::MoveTo(position.0, y as u16),
      style::PrintStyledContent(" ".repeat(size.0 as usize).reset())
    )?;
  }

  Ok(())
}
