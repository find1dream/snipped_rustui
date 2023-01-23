#[allow(unused_imports)]
use tui::{
    backend::{Backend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, List, ListItem, ListState, Paragraph, BorderType, Borders, Wrap},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

use crate::note::Note;
use crate::key::InputMode;

pub struct StatefulList<T> {
    selected_num: Option<usize>,
    state: ListState,
    pub items: Vec<T>
}

impl<T> StatefulList<T> {
    pub fn get_selected_num(&self) -> Option<usize> {
        self.selected_num
    }

    pub fn set_selected_num(&mut self, num: usize) {
        self.selected_num = Some(num as usize);
        self.state.select(Some(num as usize));
    }

    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            selected_num: None,
            state: ListState::default(),
            items
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                match i >= self.items.len() - 1 {
                    true  => 0,
                    false => i + 1
                }
            }
            None => 0
        };
        self.set_selected_num(i);
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                match i == 0 {
                    true  => self.items.len() - 1,
                    false => i - 1
                }
            }
            None => 0
        };
        self.set_selected_num(i);
    }

    pub fn delete(&mut self, index: usize) {
        self.items.remove(index);
    }

    pub fn unselect(&mut self) {
        self.state.select(None)
    }
}

pub struct App {
    pub input: String,
    pub input_mode: InputMode,
    pub list: StatefulList<Note>,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            list: StatefulList::with_items(vec![])
        }
    }
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App, note: &Note, search_text: &str) {
    let size = f.size();

    // Surrounding block
    let block = Block::default();
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(40),Constraint::Length(2), Constraint::Percentage(58)].as_ref())
        .split(f.size());

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        .split(chunks[0]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Length(1), Constraint::Min(1)].as_ref())
        .split(chunks[2]);

    let search_input = Paragraph::new(search_text.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::EditingSearch => Style::default().fg(Color::Yellow),
            _ => Style::default()
        })
        .block(Block::default().borders(Borders::ALL).title("Search").title_alignment(Alignment::Center));
    f.render_widget(search_input, left_chunks[0]);
    // Iterate through all elements in the `items` app and append some debug text to it.
    let items: Vec<ListItem> = app.list.items
        .iter()
        .enumerate()
        .map(|(_i, m)| {
            let content = vec![Spans::from(Span::raw(format!("  {}", m.title)))];
            ListItem::new(content).style(Style::default())//.fg(Color::Black).bg(Color::White))
        })
        .collect();

    // Create a List from all list items and highlight the currently selected one
    let items = List::new(items)
        .block(Block::default()
        .borders(Borders::ALL)
        .title("List")
        .title_alignment(Alignment::Center))
        .highlight_style(
            Style::default()
            .bg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
        );

    f.render_stateful_widget(items, left_chunks[1], &mut app.list.state);

    let title = Paragraph::new(note.title.as_ref())
        .block(Block::default()
        .borders(Borders::NONE))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    f.render_widget(title, right_chunks[0]);

    let language = Paragraph::new(note.language.as_ref())
        .block(Block::default()
            .borders(Borders::NONE));
    f.render_widget(language, right_chunks[1]);

    let contents = Paragraph::new(note.contents.as_ref())
        .block(Block::default()
            .borders(Borders::ALL)
            .title(app.input_mode.to_string())
            .title_alignment(Alignment::Center))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    f.render_widget(contents, right_chunks[2]);

    match app.input_mode {
        InputMode::Normal => {}
        InputMode::EditingSearch => {
            f.set_cursor(
                left_chunks[0].x + search_text.width() as u16 + 1,
                left_chunks[0].y + 1
            )
        }
        InputMode::EditingTitle => {
            f.set_cursor(
                right_chunks[0].x + note.title.width() as u16,
                right_chunks[0].y
            )
        }
        InputMode::EditingLanguage => {
            f.set_cursor(
                right_chunks[1].x + note.language.width() as u16,
                right_chunks[1].y 
            )
        }
        _ => {}
    }
}
