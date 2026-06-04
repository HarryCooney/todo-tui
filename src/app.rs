use ratatui::DefaultTerminal;
use serde::{Deserialize, Serialize};
use color_eyre::Result;
use serde_json::{Value, Error};
use std::io;
use crossterm::event::{self, Event, KeyEvent, KeyEventKind, KeyCode};
use ratatui::widgets::{ListItem, ListState};
use std::fs::File;
use std::fs;

#[derive(Debug)]
pub struct App {
    pub list: TodoList,
    pub task_buffer: TaskBuffer,
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

//TODO
//It seems possible to improve this, maybe just add a reference to a single task in main App struct
//with a lifetime or something like that... but that's for another day
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
    fn default() -> Self {
        TodoList {
            items: Vec::new(),
            state: ListState::default()
        }
    }
    //TODO
    //This seems to work but I should probably test it better
    //If first item of list is removed and list has more than one item, it might select last item
    //in list. I might do something about that
    fn remove_task(&mut self, i: usize) {
        if self.items.len() == 1 {
            self.state.select(None);
        } else {
            self.state.select_previous();
        }
        self.items.remove(i);
    }

    fn add_task(&mut self) {
    }

    fn get_item(&mut self, i: usize) -> Option<&mut TodoItem> {
        //Returns the ith item as a mutable reference
        self.items.get_mut(i)
    }
    //TODO
    //Test this
    pub fn serialize(&self) -> Result<()> {
        let t = serde_json::to_string_pretty(&self.items)?;
        println!("{}", t);
        Ok(())
    }
    
    //TODO
    //Test that this works
    fn write_list_to_file(data: &str) -> Result<()> {
         let file = "test.json";
         fs::write(file, data)?;
         Ok(())
    }
    
    //TODO
    //Read into String format
    fn read_list_from_file() {}
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
    pub fn serialize(&self) -> Result<()> {
        let todo = serde_json::to_string(self)?;
        println!("{}", todo);
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
            running: false
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
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_events(key_event);
            }
            _ => {}
        }
        Ok(())
    }
    
    fn handle_key_events(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.quit(),
            KeyCode::Char('j') => self.select_next(),
            KeyCode::Char('k') => self.select_previous(),
            KeyCode::Char('g') => self.select_first(),
            KeyCode::Char('G') => self.select_last(),
            KeyCode::Char('h') => self.select_none(),
            KeyCode::Enter => self.handle_enter(),
            KeyCode::Char('x') => self.remove_task(),
            //Adding items
            _ => {}
        }
    }
    
    fn quit(&mut self) {
        self.running = false;
    }
    
    fn remove_task(&mut self) {
        if let Some(i) = self.list.state.selected() {
                self.list.remove_task(i);
                self.update_task_buffer();
        }
    }

    fn update_task_buffer(&mut self) {
        if let Some(i) = self.list.state.selected() {
            self.task_buffer.task_name = self.list.items[i].name.to_owned();
            self.task_buffer.task_info = self.list.items[i].info.to_owned();
            self.task_buffer.current_task = Some(i);
        } else {
            self.task_buffer.task_name = String::from("");
            self.task_buffer.task_info = String::from("");
            self.task_buffer.current_task = None;
        }
    }

    fn handle_enter(&mut self) {
        //If the task being selected is the task in task_buffer, It will change status
        if self.task_buffer.current_task == self.list.state.selected() {
            self.change_status();
        } else {
            self.update_task_buffer();
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
