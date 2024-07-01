use crate::{config::AppState, db::models::Task};

pub enum CurrentScreen {
    Registering,
    LoggingIn,
    Main,
    Editing,
    Exiting,
}

pub enum CurrentlyEditing {
    Title,
    Description,
    Status,
}

pub enum CurrentlyRegistering {
    Email,
    Username,
    Password,
}

pub enum CurrentlyLoggingIn {
    Username,
    Password,
}

pub struct TaskUi {
    pub title: String,
    pub description: String,
    pub finished: bool,
}

pub struct App {
    pub title_input: String,
    pub description_input: String,
    pub finished_status: bool,
    pub tasks: Vec<Task>,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
    pub currently_registering: Option<CurrentlyRegistering>,
    pub currently_logging_in: Option<CurrentlyLoggingIn>,
}

impl App {
    pub fn new() -> App {
        App {
            title_input: String::new(),
            description_input: String::new(),
            finished_status: false,
            tasks: Vec::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            currently_registering: None,
            currently_logging_in: None,
        }
    }

    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::Title => {
                    self.currently_editing = Some(CurrentlyEditing::Description)
                }
                CurrentlyEditing::Description => {
                    self.currently_editing = Some(CurrentlyEditing::Status)
                }
                CurrentlyEditing::Status => todo!(),
            };
        } else {
            self.currently_editing = Some(CurrentlyEditing::Title);
        }
    }
}
