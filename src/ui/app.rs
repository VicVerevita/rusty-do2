use crate::config::AppState;

pub enum CurrentScreen {
    Register,
    Login,
    Main,
    Editing,
    Exiting,
}

pub enum CurrentlyEditing {
    Title,
    Description,
    Status,
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
    pub currently_editing: CurrentlyEditing,
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
        }
    }

    pub fn login(&mut self) {
        if AppState::
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
