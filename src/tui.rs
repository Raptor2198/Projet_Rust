// TUI = Terminal User Interface

use std::io::{Stdout, Write};
use std::time::Duration;

use crossterm::event::Event;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{cursor, execute, ExecutableCommand};

use crate::map::{Map, MapTile};

const CORNER_UPPER_RIGHT: &str = "┐";
const CORNER_UPPER_LEFT: &str = "┌";
const CORNER_LOWER_RIGHT: &str = "┘";
const CORNER_LOWER_LEFT: &str = "└";
const HORIZ_UPPER_LINE: &str = "─";
const HORIZ_LOWER_LINE: &str = "─";
const VERT_LINE_LEFT: &str = "│";
const VERT_LINE_RIGHT: &str = "│";

pub struct Terminal {
    pub stdout: Stdout,
}

impl Terminal {
    pub fn init() -> Result<Terminal, std::io::Error> {
        let mut stdout = std::io::stdout();
        enable_raw_mode()?;
        execute!(stdout,
            EnterAlternateScreen,
            cursor::Hide,
        )?;

        // Function to execute when we enter a panic
        // See https://doc.rust-lang.org/std/panic/fn.set_hook.html
        std::panic::set_hook(Box::new(move |info| {
            let _ = disable_raw_mode();
            let _ = execute!(std::io::stdout(),
                LeaveAlternateScreen,
                cursor::Show,
            );
            println!("{}", info.to_string());
        }));

        Ok(Terminal { stdout })
    }

    // Attempt to get the next event
    // Optionally, can use a timeout to return a None after a while
    pub fn poll_event(&self, timeout: Option<Duration>) -> Result<Option<Event>, std::io::Error> {
        let timeout = timeout.or(Some(Duration::from_secs(0))).unwrap();
        if crossterm::event::poll(timeout)? {
            crossterm::event::read().map(|v| Some(v))
        } else {
            Ok(None)
        }
    }

    pub fn wait_for_event(&self) -> Result<Event, std::io::Error> {
        crossterm::event::read()
    }

    pub fn display_map<T: MapTile>(&mut self, map: &Map<T>) -> Result<(), std::io::Error> {
        self.stdout.execute(cursor::MoveTo(0, 0))?;
        write!(
            self.stdout,
            "{CORNER_UPPER_LEFT}{}{CORNER_UPPER_RIGHT}",
            HORIZ_UPPER_LINE.repeat(map.width)
        )?;

        for y in 0..map.height {
            self.stdout.execute(cursor::MoveTo(
                0,
                (y + 1).try_into().expect("map y overflow u16"),
            ))?;
            write!(self.stdout, "{VERT_LINE_LEFT}")?;
            map.write_line(y, &mut self.stdout)?;
            write!(self.stdout, "{VERT_LINE_RIGHT}")?;
        }

        self.stdout.execute(cursor::MoveTo(
            0,
            (map.height + 1)
                .try_into()
                .expect("map height overflow u16"),
        ))?;

        write!(
            self.stdout,
            "{CORNER_LOWER_LEFT}{}{CORNER_LOWER_RIGHT}",
            HORIZ_LOWER_LINE.repeat(map.width)
        )?;
        self.stdout.flush()?;
        Ok(())
    }
}

// What happens when the variable gets destroyed
// Gracefully disable the raw mode, and clean the terminal
impl Drop for Terminal {
    fn drop(&mut self) {
        if let Err(e) = execute!(self.stdout,
            LeaveAlternateScreen,
            cursor::Show,
        ) {
            println!("Error while leaving alternate screen: {e:?}");
        }

        if let Err(e) = disable_raw_mode() {
            println!("Error while disabling raw mode: {e:?}");
        }
    }
}
