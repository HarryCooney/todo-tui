use std::env::args;
use color_eyre::Result;
use ratatui::DefaultTerminal;
use ratatui::widgets::{Widget, StatefulWidget, Block,
    Paragraph, List, ListItem, ListState, Wrap, Padding};
use ratatui::layout::{Layout, Rect, Constraint, Direction};
use ratatui::text::Line;
use ratatui::buffer::Buffer;
use crossterm::event::{self, Event, KeyEvent, KeyEventKind, KeyCode};
use std::io;
#[derive(Debug)]
struct App {
    list: TodoList,
    task_buffer: TaskBuffer,
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

//TODO
//It seems possible to improve this, maybe just add a reference to a single task in main App struct
//with a lifetime or something like that... but that's for another day
#[derive(Default, Debug)]
struct TaskBuffer {
    task_title: String,
    task_info: String,
    current_task: Option<usize>
}

#[derive(Debug)]
enum Status {
    Todo,
    Complete
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args:Vec<String> = args().collect();
    if args.len() > 1 {
        return exec_args(&args);
    }
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
                ("Task 1",
                "This will contain info about the first task, whatever that might be. Basically this text is just here to fill up space and ensure that everything is formatting correctly and that the text is being contained within the border. Also, when I switch to another task I want to make sure that the corresponding text for that task also displays properly so I will write something for the other task",
                 Status::Todo),
                ("Task 2", "Lorem ipsum dolor sit amet consectetur adipiscing elit. Quisque faucibus ex sapien vitae pellentesque sem placerat. In id cursus mi pretium tellus duis convallis. Tempus leo eu aenean sed diam urna tempor. Pulvinar vivamus fringilla lacus nec metus bibendum egestas. Iaculis massa nisl malesuada lacinia integer nunc posuere. Ut hendrerit semper vel class aptent taciti sociosqu. Ad litora torquent per conubia nostra inceptos himenaeos.", Status::Complete),
            ]),
            task_buffer: TaskBuffer::default(),
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
        self.render_tab(tab_area, buf);
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


    fn render_tab(&self, area: Rect, buf: &mut Buffer) {
        let tab_layout = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1)
        ]);
        let [tab_title_layout, tab_info_layout] = area.layout(&tab_layout);
        self.render_tab_title(tab_title_layout, buf);
        self.render_tab_info(tab_info_layout, buf);
    }
    fn render_tab_title(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().padding(Padding::horizontal(1));
        Paragraph::new(self.task_buffer.task_title.to_owned()).block(block).render(area, buf)
    }

    fn render_tab_info(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().padding(Padding::horizontal(1));
        Paragraph::new(self.task_buffer.task_info.to_owned())
            .wrap(Wrap {trim: true})
            .block(block).render(area, buf)
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
        match self.list.state.selected() {
            Some(i) => {
                self.task_buffer.task_title = self.list.items[i].name.to_owned();
                self.task_buffer.task_info = self.list.items[i].info.to_owned();
                self.task_buffer.current_task = self.list.state.selected()
            }
            None => {
                self.task_buffer.task_title = String::from("");
                self.task_buffer.task_info = String::from("");
                self.task_buffer.current_task = None
            }
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
            Status::Todo => format!("- {}", value.name),
            Status::Complete => format!("+ {}", value.name)
        };
        ListItem::new(line)

    }
}

fn exec_args(args: &Vec<String>) -> Result<()> {
    for arg in args.iter().skip(1) {
        if arg == "help" {
            help_info();
        } 
        else {
            panic!("Invalid argument: {arg}");
        }
    }
    Ok(())
}

fn help_info() {
    println!("
    Hello! Welcome to my Tuido app (The i is silent)
    These are the controls, I hope they work for you!\n
    q - Quit app
    k - Select previous task in list
    j - Select next task list
    x - Delete task
    g - Select first task in list
    G - Select last task in list
    h - Deselect current task
    ENTER - Toggle item as complete or todo");
}
