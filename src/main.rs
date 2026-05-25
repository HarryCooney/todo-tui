use std::collections::LinkedList;
use color_eyre::Result;
use ratatui::DefaultTerminal;
use ratatui::widgets::{Widget, StatefulWidget, Block, Borders, 
    Paragraph, List, ListItem, ListState};
use ratatui::layout::{Layout, Rect, Constraint, Direction};
use ratatui::text::Line;
use ratatui::buffer::Buffer;
use crossterm::event::{self, Event, KeyEvent, KeyEventKind, KeyCode};
use std::io;
#[derive(Debug)]
struct App {
    list: TodoList,
    running: bool
}

#[derive(Default, Debug)]
struct TodoList {
    items: Vec<TodoItem>,
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
            items: Vec::new(),
            state: ListState::default()
        }
    }

    fn remove_task(&mut self, i: usize) {
        self.items.remove(i);
    }

    fn add_task(&mut self) {
    }

    fn next_item(&mut self) {

    }

    fn prev_item(&mut self) {

    }


    fn get_item(&mut self, i: usize) -> Option<&mut TodoItem> {
        //Returns the ith item as a mutable reference
        self.items.get_mut(i)
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
}

impl Default for App {
    fn default() -> Self {
        Self {
            list: TodoList::from_iter([
                ("Task 1", "info 1", Status::Todo),
                ("Task 2", "info 2", Status::Complete),
            ]),
            running: false    
        }
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let main_layout = Layout::horizontal([
            Constraint::Length(40),
            Constraint::Fill(1)
        ]);
        let [list_area, tab_area] = area.layout(&main_layout);
        self.render_list(list_area, buf);
        App::render_tab(tab_area, buf);
    }
}

impl App {
    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered();
        let list_items: Vec<ListItem> = self.list
            .items
            .iter()
            .map(|item| ListItem::from(item))
            .collect();
        let list = List::new(list_items)
            .block(block)
            .highlight_symbol(">");
        StatefulWidget::render(list, area, buf, &mut self.list.state);

    }


    fn render_tab(area: Rect, buf: &mut Buffer) {
        let tab_layout = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1)
        ]);
        let [tab_title_layout, tab_info_layout] = area.layout(&tab_layout);
        App::render_tab_title(tab_title_layout, buf);
        App::render_tab_info(tab_info_layout, buf);
    }
    fn render_tab_title(area: Rect, buf: &mut Buffer) {
        let block = Block::bordered();
        Paragraph::new("Title here").block(block).render(area, buf);
    }
    fn render_tab_info(area: Rect, buf: &mut Buffer) {
        let block = Block::bordered();
        Paragraph::new("A lot of text will go here just to describe what is happening")
            .block(block).render(area, buf);

    }

    fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
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
            KeyCode::Enter => self.change_status(),
            KeyCode::Char('x') => self.remove_task(),
            //Adding items
            //Removing items
            _ => {}
        }
    }
    
    fn quit(&mut self) {
        self.running = false;
    }
    
    fn remove_task(&mut self) {
        if let Some(i) = self.list.state.selected() {
                self.list.remove_task(i)
        }
    }

    fn view_info(&mut self) {

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
            Status::Todo => format!("- {}", value.name),
            Status::Complete => format!("+ {}", value.name)
        };
        ListItem::new(line)

    }
}
