use ratatui::widgets::{Widget, StatefulWidget, Block,
    Paragraph, List, ListItem, Wrap, Padding};
use ratatui::layout::{Position, Layout, Rect, Constraint, Direction};
use ratatui::buffer::Buffer;
use ratatui::Frame;
use ratatui::style::{Style, Color, Stylize, Modifier};
use crate::app;
use crate::editor::{Editor, InputMode, CurrentlyEditing};

impl Widget for &mut app::App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let frame_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(1)
            ]).split(area);
        let main_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(40),
                Constraint::Fill(1)
            ]).split(frame_area[0]);
        self.render_list(main_area[0], buf);
        self.render_tab(main_area[1], buf);
        self.render_command(frame_area[1], buf);
    }
}


impl app::App {
    pub fn render(&mut self, frame: &mut Frame) {
        frame.render_widget(&mut *self, frame.area());
        //self.render_cursor(frame.area(), frame);
    }
    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered();
        let list_items: Vec<ListItem> = self.list
            .items
            .iter()
            .map(|item| ListItem::from(item))
            .collect();
        let list = List::new(list_items)
            .block(block)
            .highlight_symbol("> ");
        StatefulWidget::render(list, area, buf, &mut self.list.state);
    }

    fn render_command(&self, area: Rect, buf: &mut Buffer) {
        let padding = Block::new().padding(Padding::horizontal(1));
        //TODO change this back once testing is done
        let message = Paragraph::new(self.command_message.to_owned())
            .block(padding);
        message.render(area, buf);
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

    //TODO: Implement later once I figure out a better system.
    /*
    fn render_cursor(&mut self, input_area: Rect, frame: &mut Frame) {
        let frame_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(1)
            ]).split(frame.area());
        let tab_areas = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(40),
                Constraint::Fill(1)
            ]).split(frame_area[0]);

        //This is a terrible thing.
        match self.mode {
            #[expect(clippy::cast_possible_truncation)]
            app::Mode::Editing => {
                match self.editor.currently_editing {
                    CurrentlyEditing::Title => {
                        frame.set_cursor_position(Position::new(
                            tab_areas[1].x + self.editor.char_index as u16 + 2,
                            tab_areas[1].y + 1))
                    },
                    CurrentlyEditing::Info => {
                        //TODO Figure out how to get cursor working when editing a paragraph
                        frame.set_cursor_position(Position::new(
                                tab_areas[1].x + self.editor.char_index as u16 + 1,
                                tab_areas[1].y + self.calculate_y_pos_for_info()))
                    }
                }
            }
            _ => {}
        }
    }
    */

    fn render_tab_title(&self, area: Rect, buf: &mut Buffer) {
        if let app::Mode::Editing = self.mode {
            match self.editor.currently_editing {
                CurrentlyEditing::Title => {
                    let block = Block::bordered().padding(Padding::horizontal(1)).on_dark_gray();
                    Paragraph::new(self.editor.title_input.to_owned())
                        .wrap(Wrap {trim: true})
                        .block(block).render(area, buf);
                },
                CurrentlyEditing::Info => {
                    let block = Block::bordered().padding(Padding::horizontal(1));
                    Paragraph::new(self.editor.title_input.to_owned())
                        .wrap(Wrap {trim: true})
                        .block(block).render(area, buf)

                }
            }
        }
        else {
            let block = Block::bordered().padding(Padding::horizontal(1));
            match self.task_buffer.current_task {
                Some(_) => Paragraph::new(self.task_buffer.task_name.to_owned())
                    .block(block)
                    .render(area, buf),
                None => Paragraph::new("")
                    .block(block)
                    .render(area, buf)
            }
        }
    }

    fn render_tab_info(&self, area: Rect, buf: &mut Buffer) {
        if let app::Mode::Editing = self.mode {
            match self.editor.currently_editing {
                CurrentlyEditing::Info => {
                    let block = Block::bordered().padding(Padding::horizontal(1)).on_dark_gray();
                    Paragraph::new(self.editor.info_input.to_owned())
                        .wrap(Wrap {trim: true})
                        .block(block).render(area, buf)
                },
                CurrentlyEditing::Title => {
                    let block = Block::bordered().padding(Padding::horizontal(1));
                    Paragraph::new(self.editor.info_input.to_owned())
                        .wrap(Wrap {trim: true})
                        .block(block).render(area, buf)

                }
            }
        }
        else {
            let block = Block::bordered().padding(Padding::horizontal(1));
            match self.task_buffer.current_task {
                Some(_) => Paragraph::new(self.task_buffer.task_info.to_owned())
                    .wrap(Wrap {trim: true})
                    .block(block).render(area, buf),
                None => Paragraph::new("")
                    .block(block)
                    .render(area,buf)
            }
        }
    }
}
