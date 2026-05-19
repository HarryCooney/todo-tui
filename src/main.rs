use std::collections::LinkedList;
use ratatui::widgets::{List, ListItem, ListState};

#[derive(Default, Debug)]
struct App {
    list: TodoList,
    running: bool
}

#[derive(Default, Debug)]
struct TodoList {
    items: LinkedList<Item>,
    state: ListState
}

#[derive(Debug)]
struct Item {
    name: String,
    info: String,
    status: Status
}

#[derive(Debug)]
enum Status {
    Todo,
    Complete
}

fn main() {
    
}

impl TodoList {
    fn default() -> Self {
        TodoList {
            items: LinkedList::new(),
            state: ListState::default()
        }
    }
}

impl Item {
    fn new(name: String, info: String) -> Self {
        Item {
            name,
            info,
            status: Status::Todo
        }
    }
}
