#[derive(Debug)]
pub struct Editor {
    pub title_input: String,
    pub info_input: String,
    pub char_index: usize,
    pub input_mode: InputMode,
    pub currently_editing: CurrentlyEditing
}

#[derive(Debug)]
pub enum InputMode {
    Normal,
    Editing
}

#[derive(Debug)]
pub enum CurrentlyEditing {
    Title,
    Info
}

impl Editor {
    pub fn move_cursor_right(&mut self) {
            self.char_index = self.clamp_cursor(self.char_index.saturating_add(1));
    }
    pub fn move_cursor_left(&mut self) {
            self.char_index = self.clamp_cursor(self.char_index.saturating_sub(1));
    }

    fn reset_cursor(&mut self) {
        self.char_index = 0;     
    }

    pub fn delete_char(&mut self) {
        let index = self.char_index;
        if index != 0 {
            let new_index = self.char_index - 1;
            match self.currently_editing {
                CurrentlyEditing::Title => {
                    let before = self.title_input.chars().take(new_index);
                    let after = self.title_input.chars().skip(new_index + 1);
                    self.title_input = before.chain(after).collect()

                },
                CurrentlyEditing::Info => {
                    let before = self.info_input.chars().take(new_index);
                    let after = self.info_input.chars().skip(new_index + 1);
                    self.info_input = before.chain(after).collect()
                }
            }
            self.move_cursor_left();
        }
    }

    pub fn insert_char(&mut self, input_char: char) {
        let index = self.char_index;
        match self.currently_editing {
            CurrentlyEditing::Title  => self.title_input.insert(index, input_char),
            CurrentlyEditing::Info =>  self.info_input.insert(index, input_char)
        }
        self.move_cursor_right();
    }
    
    pub fn switch_editing(&mut self) {
        match self.currently_editing {
            CurrentlyEditing::Title => self.currently_editing = CurrentlyEditing::Info,
            CurrentlyEditing::Info => self.currently_editing = CurrentlyEditing::Title,
        }
        self.reset_cursor();
    }

    pub fn cursor_to_end(&mut self) {
        match self.currently_editing {
            CurrentlyEditing::Title => {
                let cursor_pos = self.title_input.chars().count();
                self.char_index = self.clamp_cursor(cursor_pos);
            },
            CurrentlyEditing::Info => {
                let cursor_pos = self.info_input.chars().count();
                self.char_index = self.clamp_cursor(cursor_pos);
            }
        }
    }

    fn clamp_cursor(&mut self, cursor_pos: usize) -> usize {
        match self.currently_editing {
            CurrentlyEditing::Title => {
                cursor_pos.clamp(0, self.title_input.chars().count())
            },
            CurrentlyEditing::Info => {
                cursor_pos.clamp(0, self.info_input.chars().count())
            }
        }
    }

    pub fn default() -> Self {
        Editor {
            title_input: String::from(""),
            info_input: String:: from(""),
            char_index: 0,
            input_mode: InputMode::Normal,
            currently_editing: CurrentlyEditing::Title
        }
    }
}
