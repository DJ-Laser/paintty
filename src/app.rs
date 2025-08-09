use std::io::{self, Stdout};

use crossterm::{
  cursor,
  event::{
    self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseButton, MouseEvent,
    MouseEventKind,
  },
  execute, queue,
  style::{self, Color, Stylize},
  terminal::{self, WindowSize, window_size},
};

pub struct App {
  stdout: Stdout,
  image: Vec<Vec<Color>>,
  should_quit: bool,
}

impl App {
  pub fn new() -> io::Result<Self> {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    let WindowSize { rows, columns, .. } = window_size()?;
    execute!(stdout, event::EnableMouseCapture)?;

    let mut image = Vec::with_capacity(rows as usize);
    for _ in 0..rows {
      let mut row = Vec::with_capacity(columns as usize);
      row.resize(columns as usize, Color::White);
      image.push(row);
    }

    Ok(Self {
      stdout,
      image,
      should_quit: false,
    })
  }

  fn redraw_screen(&mut self) -> io::Result<()> {
    queue!(
      self.stdout,
      terminal::BeginSynchronizedUpdate,
      terminal::Clear(terminal::ClearType::All)
    )?;

    for (y, row) in self.image.iter().enumerate() {
      queue!(self.stdout, cursor::MoveTo(0, y as u16))?;
      for pixel in row {
        queue!(self.stdout, style::PrintStyledContent(" ".on(*pixel)))?;
      }
    }

    execute!(self.stdout, terminal::EndSynchronizedUpdate)?;

    Ok(())
  }

  pub fn handle_key_event(&mut self, event: KeyEvent) -> io::Result<()> {
    if !matches!(event.kind, KeyEventKind::Press) {
      return Ok(());
    }

    match event.code {
      KeyCode::Char('q') => self.should_quit = true,
      KeyCode::Char('c') if event.modifiers.contains(KeyModifiers::CONTROL) => {
        self.should_quit = true
      }

      _ => (),
    }

    Ok(())
  }

  pub fn handle_mouse_event(&mut self, event: MouseEvent) -> io::Result<()> {
    if !matches!(
      event.kind,
      MouseEventKind::Down(MouseButton::Left) | MouseEventKind::Drag(MouseButton::Left)
    ) {
      return Ok(());
    }

    self.image[event.row as usize][event.column as usize] = Color::Black;

    Ok(())
  }

  pub fn handle_event(&mut self, event: Event) -> io::Result<()> {
    match event {
      Event::Key(event) => self.handle_key_event(event)?,
      Event::Mouse(event) => self.handle_mouse_event(event)?,
      _ => (),
    }

    Ok(())
  }

  pub fn run(&mut self) -> io::Result<()> {
    loop {
      self.redraw_screen()?;
      if self.should_quit {
        break;
      }

      self.handle_event(event::read()?)?;
      if self.should_quit {
        break;
      }
    }

    Ok(())
  }
}

impl Drop for App {
  fn drop(&mut self) {
    let _ = execute!(self.stdout, event::DisableMouseCapture);
    let _ = terminal::disable_raw_mode();
  }
}
