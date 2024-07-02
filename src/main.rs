use std::io;

use db::establish_connection;
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::event::DisableMouseCapture;
use ratatui::crossterm::event::EnableMouseCapture;
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::disable_raw_mode;
use ratatui::crossterm::terminal::enable_raw_mode;
use ratatui::crossterm::terminal::EnterAlternateScreen;
use ratatui::crossterm::terminal::LeaveAlternateScreen;
use ratatui::Terminal;
use ui::app::App;

#[macro_use]
extern crate diesel;

pub mod auth;
pub mod commands;
pub mod config;
pub mod db;
pub mod ui;

fn main() {
    let conn = establish_connection();

    enable_raw_mode().expect("Could not initiate terminal instance!");
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)
        .expect("Could not enable terminal instance!");

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend).expect("Failed starting terminal instance!");

    let mut app = App::new(conn);
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode().expect("Failed closing terminal instance!");
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )
    .expect("Failed leaving terminal instance!");

    if let Ok(do_print) = res {
        if do_print {
            todo!();
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }
}
