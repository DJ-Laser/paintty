use std::io::{self};

use app::App;

mod app;
mod canvas;

fn main() -> io::Result<()> {
  let mut app = App::new()?;

  app.run()?;

  Ok(())
}
