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
}
