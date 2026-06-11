use ratatui::DefaultTerminal;
use std::process;
use serde::{Deserialize, Serialize};
use color_eyre::Result;
use serde_json::Error;
use std::io::{self, BufReader};
use crossterm::event::{self, Event, KeyEvent, KeyEventKind, KeyCode, KeyModifiers};
use ratatui::widgets::{ListItem, ListState};
use std::fs::{self, File};
use crate::editor::{Editor, InputMode};

#[derive(Debug)]
pub struct App {
    pub list: TodoList,
    pub task_buffer: TaskBuffer,
    //Might have to change this later if I chage how it works
    //Keep simple for now, allow for scalability
    pub command_message: String,
    pub editor: Editor,
    pub mode: Mode,
    pub running: bool
}

#[derive(Default, Debug)]
pub struct TodoList {
    pub items: Vec<TodoItem>,
    pub state: ListState
}

#[derive(Debug, Serialize,Deserialize)]
pub struct TodoItem {
    pub name: String,
    pub info: String,
    pub status: Status
}

#[derive(Default, Debug)]
pub struct TaskBuffer {
    pub task_name: String, 
    pub task_info: String,
    pub current_task: Option<usize>
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    Todo,
    Complete
}

#[derive(Debug)]
pub enum Mode {
    Editing,
    Viewing,
    SelectingFile
}

impl FromIterator<(&'static str, &'static str, Status)> for TodoList {
    fn from_iter<I>(iter: I) -> Self
        where I: IntoIterator<Item = (&'static str, &'static str, Status)>
    {
        let items = iter
            .into_iter()
            .map(|(name, info, status)| TodoItem::new(name, info, status))
            .collect();
        let state = ListState::default();
        Self {items, state}
    }
}

impl TodoList {
    #[allow(dead_code)]
    fn default() -> Self {
        TodoList {
            items: Vec::new(),
            state: ListState::default()
        }
    }
    
    ///Removes a task
    ///
    ///The edge case which has to be accounted for is if there are no
    ///items left after a deletion. In this case, ListState is set to none.
    fn remove_task(&mut self, i: Option<usize>) {
        if let Some(i) = i {
            if self.items.len() == 1 {
                self.state.select(None);
            }
            self.items.remove(i);
        }
    }
    /*
     * If the item being displayed is deleted, display the next item that is selected
     * If there is only one item in the list and it is deleted, set list.selected to None
     * Then, flush task buffer
     * Should all of the logic be in this?
    */

    #[allow(dead_code)]
    fn add_task(&mut self) {

    }
    ///Returns the TodoItem at a given index
    fn get_item(&mut self, i: usize) -> Option<&mut TodoItem> {
        self.items.get_mut(i)
   }

    //TODO
    //Test this
    pub fn serialize(&self) -> Result<String, Error> {
        serde_json::to_string_pretty(&self.items)
    }
    
    pub fn read_list_from_file(&mut self, file_name: &str) -> Result<()> {
        match File::open(file_name) {
            Ok(file) => {
                let reader = BufReader::new(file);
                self.items = serde_json::from_reader(reader)?;
            },
            Err(e) => {
                println!("{:?}", e);
                process::exit(1);
            }
        };
        Ok(())
    }
}

impl TodoItem {
    fn new(name: &str, info: &str, status: Status) -> Self {
        TodoItem {
            name: name.to_string(),
            info: info.to_string(),
            status
        }
    }
    
    fn change_status(&mut self) {
        match self.status {
            Status::Todo => self.status = Status::Complete,
            Status::Complete => self.status = Status::Todo
        }
    }

    ///Serializes TodoItem to json form
    pub fn serialize(&self) -> Result<()> {
        serde_json::to_string(self)?;
        Ok(())
    }
    pub fn deserialize(todo_item: &str) -> Result<TodoItem, serde_json::Error> {
        serde_json::from_str(todo_item)
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            list: TodoList::from_iter([
                ("Task 1",
                "This will contain info about the first task, whatever that might be. Basically this text is just here to fill up space and ensure that everything is formatting correctly and that the text is being contained within the border. Also, when I switch to another task I want to make sure that the corresponding text for that task also displays properly so I will write something for the other task",
                 Status::Todo),
                ("Task 2", "Lorem ipsum dolor sit amet consectetur adipiscing elit. Quisque faucibus ex sapien vitae pellentesque sem placerat. In id cursus mi pretium tellus duis convallis. Tempus leo eu aenean sed diam urna tempor. Pulvinar vivamus fringilla lacus nec metus bibendum egestas. Iaculis massa nisl malesuada lacinia integer nunc posuere. Ut hendrerit semper vel class aptent taciti sociosqu. Ad litora torquent per conubia nostra inceptos himenaeos.",
                 Status::Complete),
                ("Task 3", "Text here",
                 Status::Todo)
            ]),
            task_buffer: TaskBuffer::default(),
            command_message: String::from(""),
            editor: Editor::default(),
            mode: Mode::Viewing,
            running: false,
        }
    }
}

impl App {
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    /// Handles all keyboard inputs
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_events(&key_event);
            },
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press &&
                key_event.modifiers == KeyModifiers::CONTROL => {
                self.handle_command_events(&key_event);   
            },
            _ => {}
        }
        Ok(())
    }
    
    ///Handles all keyboard inputs.
    ///Inputs are seperated based on the app mode for ease of readability
    fn handle_key_events(&mut self, key_event: &KeyEvent) {
        if key_event.modifiers == KeyModifiers::CONTROL {
            match self.mode {
                Mode::Viewing => self.handle_commands_viewer_events(key_event),
                Mode::Editing => self.handle_commands_editor_events(key_event),
                Mode::SelectingFile => self.handle_commands_file_view_events(key_event)
            }
        }
        else {
            match self.mode {
                Mode::Viewing => self.handle_viewing_events(key_event),
                Mode::Editing => self.handle_editor_events(key_event),
                Mode::SelectingFile => {}
            }
        }
    }

    //TODO (Things todo might not necessarily be in this method, could also be in
    //command_editing method)
    //Add binds for setting normal mode
    //add binds for inserting and deleting chars
    fn handle_editor_events(&mut self, key_event: &KeyEvent) {
        match self.editor.input_mode {
            InputMode::Normal => {
                match key_event.code {
                    KeyCode::Char('h') => self.editor.move_cursor_left(),
                    KeyCode::Char('l') => self.editor.move_cursor_right(),
                    KeyCode::Tab => self.editor.switch_editing(),
                    KeyCode::Char('i') => self.editor.input_mode = InputMode::Editing,
                    _ => {}
                }
            },
            InputMode::Editing => {
            }
        }
    }

    //TODO This will be used when the file viewer is added
    fn handle_file_view_events(&mut self, key_event: &KeyEvent) {

    }

    fn handle_viewing_events(&mut self, key_event: &KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.quit(),
            KeyCode::Char('j') => self.select_next(),
            KeyCode::Char('k') => self.select_previous(),
            KeyCode::Char('g') => self.select_first(),
            KeyCode::Char('G') => self.select_last(),
            KeyCode::Char('h') => self.select_none(),
            KeyCode::Enter => self.handle_enter(),
            KeyCode::Char('x') => self.remove_task(),
            _ => {}
        }
    }

    fn handle_commands_editor_events(&mut self, key_event: &KeyEvent) {
        //TODO Add way to get back to viewing panela (maybe using CTRL-V)
        match key_event.code {
            KeyCode::Char('c') => self.editor.input_mode = InputMode::Normal,
            _ => {}
        }
    }

    fn handle_commands_viewer_events(&mut self, key_event: &KeyEvent) {
        match key_event.code {
            KeyCode::Char('s') => self.save_list(),
            KeyCode::Char('e') => self.switch_mode(Mode::Editing),
            _ => {}
        }
    }

    fn handle_commands_file_view_events(&mut self, key_event: &KeyEvent) {

    }

    fn switch_mode(&mut self, mode: Mode) {
        match mode {
            Mode::Viewing => self.mode = Mode::Viewing,
            Mode::Editing => self.mode = Mode::Editing,
            Mode::SelectingFile => self.mode = Mode::SelectingFile
        }
    }
    
    fn quit(&mut self) {
        self.running = false;
    }

    ///method called when remove keybind is pressed
    ///
    ///Updates TodoList and TaskBuffer accordingly:
    ///If there is only one item left in TodoList, wipe TB and TL
    ///If the TB = currently selected and is the last item, decrement both
    ///If the task in TB is a larger index than what is being deleted,
    ///decrement current_task in TB.
    ///Otherwise, just remove task and don't update TB at all.
    fn remove_task(&mut self) {
        if self.list.state.selected() == Some(0) {
            self.list.remove_task(self.list.state.selected());
            self.update_task_buffer(self.list.state.selected());
        }
        else if self.list.state.selected() == self.task_buffer.current_task 
        && self.list.state.selected() == Some(self.list.items.len() - 1) {

            self.list.remove_task(self.list.state.selected());
            self.select_previous();
            self.update_task_buffer(self.list.state.selected());
        }
        else if self.list.state.selected() < self.task_buffer.current_task {
            self.list.remove_task(self.list.state.selected());
            self.task_buffer.decrement_current_task();
        }
        else {
            self.list.remove_task(self.list.state.selected());
        }
    }

    ///Updates information in task buffer.
    ///
    ///This information is used to render the info tab.
    ///The buffer is only updated if the index of the current TodoItem in TodoList
    ///Doesn't match the item in the buffer. This saves having to clone on each frame.
    fn update_task_buffer(&mut self, i: Option<usize>) {
        match i {
            Some(i) => {
                self.task_buffer.task_name = self.list.items[i].name.to_owned();
                self.task_buffer.task_info = self.list.items[i].info.to_owned();
                self.task_buffer.current_task = Some(i);
            },
            None => {
                self.task_buffer.task_name = String::from("");
                self.task_buffer.task_info = String::from("");
                self.task_buffer.current_task = None;
            }
        }
    }

    fn handle_enter(&mut self) {
        //If the task being selected is the task in task_buffer, It will change status
        if self.task_buffer.current_task == self.list.state.selected() {
            self.change_status();
        } else {
            self.update_task_buffer(self.list.state.selected());
        }
    }

    fn select_first(&mut self) {
        self.list.state.select_first();
    }
    fn select_last(&mut self) {
        self.list.state.select_last();
    }
    fn select_next(&mut self) {
        self.list.state.select_next();
    }
    fn select_previous(&mut self) {
        self.list.state.select_previous();
    }
    fn select_none(&mut self) {
        self.list.state.select(None);
    }
    fn change_status(&mut self) {
        if let Some(i) = self.list.state.selected() {
            match self.list.get_item(i) {
                Some(item) => {
                    item.change_status();
                },
                None => {} 
            }
        }
    }

    fn write_list_to_file(data: &str) -> Result<()> {
         let file = "test.json";
         fs::write(file, data)?;
         Ok(())
    }
    fn save_list(&mut self) {
        match self.list.serialize() {
            Ok(json) => match App::write_list_to_file(&json) {
                Ok(()) => self.command_message = String::from("List saved"),
                Err(_) => self.command_message = String::from("Error: Save file failed")
            },
            Err(_) => self.command_message = String::from("Error: Save file failed")
        }
    }
}

impl From<&TodoItem> for ListItem<'_> {
    fn from(value: &TodoItem) -> Self {
        let line = match value.status {
            Status::Todo => format!("☓ {}", value.name),
            Status::Complete => format!("✓ {}", value.name)
        };
        ListItem::new(line)
    }
}

impl TaskBuffer {
    fn decrement_current_task(&mut self) {
        if let Some(i) = self.current_task {
            if i == 0 {
                self.current_task = None;
            } else {
                self.current_task = Some(i - 1);
            }
        }
    }
}
