use diesel::PgConnection;

use crate::{
    commands::{self, create_task},
    db::models::{NewTask, Task, User},
};

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
    user: Option<User>,
    conn: PgConnection,
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
    pub fn new(conn: PgConnection) -> App {
        App {
            user: None,
            conn,
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

    pub fn toggle_registering(&mut self) {
        if let Some(register_mode) = &self.currently_registering {
            match register_mode {
                CurrentlyRegistering::Email => {
                    self.currently_registering = Some(CurrentlyRegistering::Username)
                }
                CurrentlyRegistering::Username => {
                    self.currently_registering = Some(CurrentlyRegistering::Password)
                }
                CurrentlyRegistering::Password => {
                    todo!()
                }
            }
        }
    }

    pub fn toggle_logging_in(&mut self) {
        if let Some(login_mode) = &self.currently_logging_in {
            match login_mode {
                CurrentlyLoggingIn::Username => {
                    self.currently_registering = Some(CurrentlyRegistering::Password)
                }
                CurrentlyLoggingIn::Password => {
                    todo!()
                }
            }
        }
    }

    pub fn save_task(&mut self) {
        if let Some(current_user) = &self.user {
            if !self.title_input.is_empty() {
                create_task(
                    &self.conn,
                    &self.title_input,
                    Some(&self.description_input),
                    self.finished_status,
                    current_user.user_id,
                );
            } else {
                todo!()
            }
        } else {
            todo!()
        }
    }
}
