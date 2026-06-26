use std::io::{self};
use crate::app::{Mode, App};
use crate::editor::{InputMode, CurrentlyEditing};
use crossterm::event::{self, Event, KeyEvent, KeyEventKind, KeyCode, KeyModifiers};
use std::path::Path;

impl App {
    /// Handles all keyboard inputs
    pub fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_events(&key_event);
            },
            _ => {}
        }
        Ok(())
    }

    ///Handles all keyboard inputs.
    ///Inputs are seperated based on the app mode for ease of readability
    pub fn handle_key_events(&mut self, key_event: &KeyEvent) {
        if key_event.modifiers == KeyModifiers::CONTROL {
            match self.mode {
                Mode::Viewing => self.handle_commands_viewer_events(key_event),
                Mode::Editing => self.handle_commands_editor_events(key_event),
                Mode::SelectingFile => self.handle_commands_selecting_file_events(key_event),
                Mode::SavingFile => self.handle_commands_saving_file_events(key_event)
            }
        }
        else {
            match self.mode {
                Mode::Viewing => self.handle_viewing_events(key_event),
                Mode::Editing => self.handle_editor_events(key_event),
                Mode::SelectingFile => self.handle_selecting_file_events(key_event),
                Mode::SavingFile => self.handle_saving_file_events(key_event)
            }
        }
    }

    pub fn handle_commands_saving_file_events(&mut self, key_event: &KeyEvent) {
        match self.editor.input_mode {
            InputMode::Normal => {
                match key_event.code {
                    _ => {}
                }
            },
            InputMode::Editing => {
                match key_event.code {
                    KeyCode::Char('c') => self.editor.input_mode = InputMode::Normal,
                    _ => {}
                }
            }
        }
    }
    pub fn handle_saving_file_events(&mut self, key_event: &KeyEvent) {
        match self.editor.input_mode {
            InputMode::Editing => {
                match key_event.code {
                    KeyCode::Char(to_insert) => self.editor.insert_char(to_insert),
                    KeyCode::Backspace => self.editor.delete_char(),
                    KeyCode::Enter => self.handle_enter_saving_file(),
                    KeyCode::Tab => self.editor.insert_char('\t'),
                    _ => {}
                }
            },
            InputMode::Normal => {
                match key_event.code {
                    KeyCode::Char('i') => self.editor.input_mode = InputMode::Editing,
                    KeyCode::Char('q') => self.quit(),
                    KeyCode::Enter => self.handle_enter_saving_file(),
                    _ => {}
                }
            }
        }
    }

    pub fn handle_enter_saving_file(&mut self) {
        self.save_list(self.editor.file_name_input.clone());
    }

    pub fn handle_enter_viewing(&mut self) {
        //If the task being selected is the task in task_buffer, It will change status
        if self.task_buffer.current_task == self.list.state.selected() {
            self.change_status();
        } else {
            self.update_task_buffer(self.list.state.selected());
        }
    }

    fn handle_enter_selecting_file(&mut self, selected: Option<usize>) {
        if let Some(file) = selected {
            let p = format!("./lists/{}", self.file_viewer.files[file]);
            let path = Path::new(p.as_str());
            self.list.read_list_from_file(path);
            self.switch_mode(Mode::Viewing);
        }
    }

    pub fn handle_commands_editor_events(&mut self, key_event: &KeyEvent) {
        match self.editor.input_mode {
            InputMode::Normal => {
                //When mode is switched to viewing, changes are saved from editor to the task which
                //was being edited (if it exists)
                if let KeyCode::Char('v') = key_event.code {
                    self.switch_mode(Mode::Viewing);
                    self.task_buffer.task_name = self.editor.title_input.clone();
                    self.task_buffer.task_info = self.editor.info_input.clone();
                    if let Some(i) = self.task_buffer.current_task {
                        self.list.items[i].name = self.editor.title_input.clone();
                        self.list.items[i].info = self.editor.info_input.clone();
                    };
                }
            },
            InputMode::Editing => {
                if let KeyCode::Char('c') = key_event.code {
                    self.editor.input_mode = InputMode::Normal;
                }
            }
        }
    }

    pub fn handle_editor_events(&mut self, key_event: &KeyEvent) {
        match self.editor.input_mode {
            InputMode::Normal => {
                match key_event.code {
                    KeyCode::Char('q') => self.quit(),
                    KeyCode::Char('h') => self.editor.move_cursor_left(),
                    KeyCode::Char('l') => self.editor.move_cursor_right(),
                    KeyCode::Tab => self.editor.switch_editing(),
                    KeyCode::Char('i') => {
                        self.editor.input_mode = InputMode::Editing;
                    },
                    _ => {}
                }
            },
            InputMode::Editing => {
                match key_event.code {
                    KeyCode::Char(to_insert) => self.editor.insert_char(to_insert),
                    KeyCode::Backspace => self.editor.delete_char(),
                    KeyCode::Enter => self.editor.insert_char('\n'),
                    KeyCode::Tab => self.editor.insert_char('\t'),
                    _ => {}
                }
            }
        }
    }

    pub fn handle_viewing_events(&mut self, key_event: &KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.quit(),
            KeyCode::Char('j') => self.select_next(Mode::Viewing),
            KeyCode::Char('k') => self.select_previous(Mode::Viewing),
            KeyCode::Char('g') => self.select_first(Mode::Viewing),
            KeyCode::Char('G') => self.select_last(Mode::Viewing),
            KeyCode::Char('h') => self.select_none(Mode::Viewing),
            KeyCode::Enter => self.handle_enter_viewing(),
            KeyCode::Char('x') => self.remove_task(),
            _ => {}
        }
    }

    pub fn handle_commands_viewer_events(&mut self, key_event: &KeyEvent) {
        match key_event.code {
            KeyCode::Char('s') => {
                self.switch_mode(Mode::SavingFile);
                self.editor.currently_editing = CurrentlyEditing::FileName;
                self.editor.input_mode = InputMode::Editing
            },
            //When you start editing, the task buffer is loaded into the editor as the info to be
            //edited.
            KeyCode::Char('e') => {
                self.switch_mode(Mode::Editing);
                self.editor.title_input = self.task_buffer.task_name.clone();
                self.editor.info_input = self.task_buffer.task_info.clone();
            },
            KeyCode::Char('a') => {
                self.create_new_task();
                self.editor.title_input = self.task_buffer.task_name.clone();
                self.editor.info_input = self.task_buffer.task_info.clone();
                self.select_last(Mode::Viewing);
                self.switch_mode(Mode::Editing);
            },
            KeyCode::Char('o') => {
                self.switch_mode(Mode::SelectingFile);
                self.file_viewer.read_storage();
            },
            _ => {}
        }
    }

    pub fn handle_commands_selecting_file_events(&mut self, key_event: &KeyEvent) {
        match key_event.code {
            KeyCode::Char('v') => self.switch_mode(Mode::Viewing),
            //Delete list?
            _ => {}
        }
    }

    pub fn handle_selecting_file_events(&mut self, key_event: &KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.quit(),
            KeyCode::Char('j') => self.select_next(Mode::SelectingFile),
            KeyCode::Char('k') => self.select_previous(Mode::SelectingFile),
            KeyCode::Char('g') => self.select_first(Mode::SelectingFile),
            KeyCode::Char('G') => self.select_last(Mode::SelectingFile),
            KeyCode::Char('h') => self.select_none(Mode::SelectingFile),
            KeyCode::Enter => self.handle_enter_selecting_file(self.file_viewer.state.selected()),
            KeyCode::Char('x') => self.remove_task(),
            _ => {}
        }
    }
}
