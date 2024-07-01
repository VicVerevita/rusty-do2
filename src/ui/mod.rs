use app::{App, CurrentScreen, CurrentlyEditing, CurrentlyLoggingIn, CurrentlyRegistering};
use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event, KeyCode},
    Terminal,
};

pub mod app;

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal
            .draw(|f| ui(f, app))
            .expect("Could not draw terminal!");

        if let Event::Key(key) = event::read().expect("Could not read key") {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match app.current_screen {
                CurrentScreen::Registering => match key.code {
                    KeyCode::Char('e') => {
                        app.currently_registering = Some(CurrentlyRegistering::Email);
                    }
                    KeyCode::Char('u') => {
                        app.currently_registering = Some(CurrentlyRegistering::Username);
                    }
                    KeyCode::Char('p') => {
                        app.currently_registering = Some(CurrentlyRegistering::Password);
                    }
                    KeyCode::Char('l') => {
                        app.current_screen = CurrentScreen::LoggingIn;
                        app.currently_logging_in = Some(CurrentlyLoggingIn::Username);
                    }
                    KeyCode::Char('q') | KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    KeyCode::Enter => {
                        todo!();
                        app.current_screen = CurrentScreen::Main;
                    }
                    _ => {}
                },
                CurrentScreen::LoggingIn => match key.code {
                    KeyCode::Char('u') => {
                        app.currently_logging_in = Some(CurrentlyLoggingIn::Username);
                    }
                    KeyCode::Char('p') => {
                        app.currently_logging_in = Some(CurrentlyLoggingIn::Password);
                    }
                    KeyCode::Char('q') | KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    KeyCode::Enter => {
                        todo!();
                        app.current_screen = CurrentScreen::Main;
                    }
                    _ => {}
                },
                CurrentScreen::Main => match key.code {
                    KeyCode::Char(' ') => {
                        app.current_screen = CurrentScreen::Editing;
                        app.currently_editing = Some(CurrentlyEditing::Title);
                    }
                    _ => {}
                },
                CurrentScreen::Editing => todo!(),
                CurrentScreen::Exiting => todo!(),
            }
        }
    }
}
