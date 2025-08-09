pub enum PaintTool {
  Paintbrush,
  Bucket,
}

pub struct Canvas {
  pixels: Vec<Vec<Pixel>>,
  current_color: Pixel,
  current_tool: PaintTool,
}

impl Canvas {
  pub fn new(width: usize, height: usize) -> Self {
    let mut pixels = Vec::with_capacity(height);
    pixels.resize_with(height, || {
      let mut row = Vec::with_capacity(width);
      row.resize(width, Default::default());
      row
    });

    Self {
      pixels,
      current_color: Pixel::BLACK,
      current_tool: PaintTool::Paintbrush,
    }
  }

  pub fn pixels(&self) -> &Vec<Vec<Pixel>> {
    &self.pixels
  }

  pub fn current_tool(&self) -> &PaintTool {
    &self.current_tool
  }

  pub fn set_tool(&mut self, tool: PaintTool) {
    self.current_tool = tool;
  }

  fn paint_pixel(&mut self, x: usize, y: usize) {
    let Some(pixel) = self.pixels.get_mut(y).and_then(|row| row.get_mut(x)) else {
      return;
    };

    *pixel = self.current_color;
  }

  pub fn interact_with_pixel(&mut self, x: usize, y: usize) {
    match self.current_tool {
      PaintTool::Paintbrush => self.paint_pixel(x, y),
      PaintTool::Bucket => todo!(),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pixel {
  r: u8,
  g: u8,
  b: u8,
  a: u8,
}

impl Pixel {
  pub const WHITE: Self = Self::from_rgb(255, 255, 255);
  pub const BLACK: Self = Self::from_rgb(0, 0, 0);

  pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
    Self { r, g, b, a }
  }

  pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
    Self::from_rgba(r, g, b, 255)
  }
}

impl Default for Pixel {
  fn default() -> Self {
    Self::WHITE
  }
}

impl From<Pixel> for crossterm::style::Color {
  fn from(value: Pixel) -> Self {
    use crossterm::style::Color;
    if value.a < 255 {
      return Color::Reset;
    }

    Color::Rgb {
      r: value.r,
      g: value.g,
      b: value.b,
    }
  }
}
