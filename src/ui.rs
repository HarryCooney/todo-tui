use ratatui::widgets::{Widget, StatefulWidget, Block,
    Paragraph, List, ListItem, Wrap, Padding};
use ratatui::layout::{Layout, Rect, Constraint};
use ratatui::buffer::Buffer;
use crate::app;

impl Widget for &mut app::App {
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


impl app::App {
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
        match self.task_buffer.current_task {
            Some(_) => Paragraph::new(self.task_buffer.task_name.to_owned())
                .block(block)
                .render(area, buf),
            None => Paragraph::new("")
                .block(block)
                .render(area, buf)
        }
    }

    fn render_tab_info(&self, area: Rect, buf: &mut Buffer) {
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
