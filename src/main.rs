use std::collections::LinkedList;
use color_eyre::Result;
use ratatui::DefaultTerminal;
use ratatui::widgets::{Widget, StatefulWidget, Block, Borders, 
    Paragraph, List, ListItem, ListState};
use ratatui::layout::{Layout, Rect, Constraint};
use ratatui::text::Line;
use ratatui::buffer::Buffer;
use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use std::io;
#[derive(Debug)]
struct App {
    list: TodoList,
    running: bool
}

#[derive(Default, Debug)]
struct TodoList {
    items: LinkedList<TodoItem>,
    state: ListState
}

#[derive(Debug)]
struct TodoItem {
    name: String,
    info: String,
    status: Status
}

#[derive(Debug)]
enum Status {
    Todo,
    Complete
}

fn main() -> Result<()> {

    color_eyre::install()?;
    ratatui::run(|terminal| App::default().run(terminal))
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
            items: LinkedList::new(),
            state: ListState::default()
        }
    }

    fn remove_task(&mut self) {

    }

    fn add_task(&mut self) {
    }

    fn next_item(&mut self) {

    }

    fn prev_item(&mut self) {

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
}

impl Default for App {
    fn default() -> Self {
        Self {
            list: TodoList::from_iter([
                ("Task 1", "info 1", Status::Todo),
                ("Task 2", "info 2", Status::Todo),
            ]),
            running: false    
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {}
}

impl App {
    fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            self.handle_events();
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
            //Quit
            //Move through list
            //Adding items
            //Removing items
            _ => {}
        }
    }


}
