use std::io::{self, Stdout};

use crossterm::{
  cursor, queue,
  style::{self, Color, Stylize},
  terminal::WindowSize,
};

use crate::canvas::{Canvas, PaintTool, Pixel};

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
  palette_pos: (u16, u16),
  bounds: Vec<Bound>,
}

impl DialogState {
  const TOOLBAR_SIZE: (u16, u16) = (4, 4);
  const PALETTE_SIZE: (u16, u16) = (24, 4);
  const COLORS: [Pixel; 20] = [
    Pixel::from_rgb(0, 0, 0),
    Pixel::from_rgb(120, 120, 120),
    Pixel::from_rgb(153, 0, 48),
    Pixel::from_rgb(237, 28, 36),
    Pixel::from_rgb(255, 126, 0),
    Pixel::from_rgb(255, 242, 0),
    Pixel::from_rgb(34, 177, 76),
    Pixel::from_rgb(0, 183, 239),
    Pixel::from_rgb(47, 54, 153),
    Pixel::from_rgb(111, 49, 152),
    Pixel::from_rgb(255, 255, 255),
    Pixel::from_rgb(180, 180, 180),
    Pixel::from_rgb(156, 90, 60),
    Pixel::from_rgb(255, 163, 177),
    Pixel::from_rgb(255, 194, 14),
    Pixel::from_rgb(245, 228, 156),
    Pixel::from_rgb(168, 230, 29),
    Pixel::from_rgb(153, 217, 234),
    Pixel::from_rgb(112, 154, 209),
    Pixel::from_rgb(181, 165, 213),
  ];

  pub fn new(terminal_size: &WindowSize) -> Self {
    let toolbar_pos = (2, terminal_size.rows - 5);
    let palette_pos = (toolbar_pos.0 + Self::TOOLBAR_SIZE.0, terminal_size.rows - 5);

    Self {
      hidden: true,
      toolbar_pos,
      palette_pos,
      bounds: Vec::new(),
    }
  }

  pub fn bounds(&self) -> &Vec<Bound> {
    &self.bounds
  }

  pub fn render_palette(&mut self, stdout: &mut Stdout) -> io::Result<()> {
    self
      .bounds
      .push(Bound::from_pos_size(self.palette_pos, Self::PALETTE_SIZE));
    draw_dialog(stdout, self.palette_pos, Self::PALETTE_SIZE)?;

    queue!(
      stdout,
      cursor::MoveTo(self.palette_pos.0 + 2, self.palette_pos.1 + 1),
    )?;

    for color in &Self::COLORS[0..10] {
      queue!(stdout, style::PrintStyledContent("  ".on((*color).into())))?;
    }

    queue!(
      stdout,
      cursor::MoveTo(self.palette_pos.0 + 2, self.palette_pos.1 + 2),
    )?;

    for color in &Self::COLORS[10..20] {
      queue!(stdout, style::PrintStyledContent("  ".on((*color).into())))?;
    }

    Ok(())
  }

  pub fn render(&mut self, stdout: &mut Stdout, canvas: &Canvas) -> io::Result<()> {
    self.bounds.clear();

    if self.hidden {
      return Ok(());
    }

    self
      .bounds
      .push(Bound::from_pos_size(self.toolbar_pos, Self::TOOLBAR_SIZE));
    draw_dialog(stdout, self.toolbar_pos, Self::TOOLBAR_SIZE)?;
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
      cursor::MoveTo(self.toolbar_pos.0 + 1, self.toolbar_pos.1 + 2),
      style::Print(" "),
      style::Print("🪣".on(bucket_color)),
    )?;

    self.render_palette(stdout)?;

    Ok(())
  }

  pub fn interact(&mut self, pos: (u16, u16), canvas: &mut Canvas) {
    if pos.0 >= self.toolbar_pos.0 + 2 && pos.0 < self.toolbar_pos.0 + Self::TOOLBAR_SIZE.0 {
      match pos.1 - self.toolbar_pos.1 {
        1 => canvas.set_tool(PaintTool::Paintbrush),
        2 => canvas.set_tool(PaintTool::Bucket),
        _ => (),
      }
    } else {
      let color_column_offset = (pos.0)
        .checked_sub(self.palette_pos.0 + 2)
        .map(|x| x / 2)
        .take_if(|x| *x < 10);

      let color_row_offset = (pos.1)
        .checked_sub(self.palette_pos.1 + 1)
        .take_if(|y| *y < 2)
        .map(|y| y * 10);

      let color_index = color_row_offset
        .and_then(|row_offset| color_column_offset.map(|column_offset| row_offset + column_offset));
      if let Some(color_index) = color_index {
        canvas.set_color(Self::COLORS[color_index as usize]);
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
