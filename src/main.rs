use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::fmt;
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, List, ListItem, ListState, Paragraph, BorderType, Borders},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

#[derive(Debug)]
enum InputMode {
    Normal,
    EditingSearch,
    EditingCode,
    EditingTitle,
    EditingLanguage,
}

impl fmt::Display for InputMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

struct Note {
    title: String,
    contents: String
}

struct StatefulList<T> {
    selected_num: Option<usize>,
    state: ListState,
    items: Vec<T>
}

impl<T> StatefulList<T> {
    fn get_selected_num(&self) -> Option<usize> {
        self.selected_num
    }

    fn set_selected_num(&mut self, num: usize) {
        self.selected_num = Some(num);
    }
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            selected_num: None,
            state: ListState::default(),
            items
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                match i >= self.items.len() - 1 {
                    true  => 0,
                    false => i + 1
                }
            }
            None => 0
        };
        self.state.select(Some(i));
        self.set_selected_num(i);
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                match i == 0 {
                    true  => self.items.len() - 1,
                    false => i - 1
                }
            }
            None => 0
        };
        self.state.select(Some(i));
        self.set_selected_num(i);
    }

    fn unselect(&mut self) {
        self.state.select(None)
    }
}

struct App {
    search_input: String,
    code_input: String,
    input_mode: InputMode,
    list: StatefulList<Note>,
}

impl Default for App {
    fn default() -> App {
        App {
            search_input: String::new(),
            code_input: String::new(),
            input_mode: InputMode::Normal,
            list: StatefulList::with_items(vec![Note{title:"Item0".to_string(), contents: "Item0 content".to_string()}, 
                                                Note{title:"Item1".to_string(), contents: "Item1 content".to_string()}])
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::default();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn process_key_input(mut app: App) {
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key {
                    KeyEvent {code: KeyCode::Char('e'), modifiers: KeyModifiers::CONTROL, kind: pressed, state: none} => {app.input_mode = InputMode::EditingSearch;}
                    KeyEvent {code: KeyCode::Char('g'), modifiers: KeyModifiers::CONTROL, kind: pressed, state: none} => {app.input_mode = InputMode::EditingCode;}
                    KeyEvent {code: KeyCode::Char('q'), modifiers: KeyModifiers::ALT, kind: pressed, state: none} => return Ok(()),
                    KeyEvent {code: KeyCode::Left, modifiers: KeyModifiers::NONE, kind: pressed, state: none } => {
                        app.list.unselect();
                        app.code_input = app.list.items[app.list.get_selected_num().unwrap()].contents.clone();
                    }
                    KeyEvent {code: KeyCode::Down, modifiers: KeyModifiers::NONE, kind: pressed, state: none} => {
                        app.list.next();
                        app.code_input = app.list.items[app.list.get_selected_num().unwrap()].contents.clone();
                    }
                    KeyEvent {code: KeyCode::Up, modifiers: KeyModifiers::NONE, kind: pressed, state: none} => {
                        app.list.previous()
                    }
                    _ => {}

                }
                InputMode::EditingSearch => match key {
                    KeyEvent {code: KeyCode::Enter, modifiers: KeyModifiers::NONE, kind: pressed, state: none} => {
                        app.list.items.push(Note{title: app.search_input.drain(..).collect(), contents:app.search_input.drain(..).collect()} );
                    }
                    KeyEvent {code: KeyCode::Char(c), modifiers: KeyModifiers::NONE, kind: pressed, state: none} => {
                       app.search_input.push(c);
                    }
                    KeyEvent {code: KeyCode::Backspace, modifiers: KeyModifiers::NONE, kind: pressed, state: none} => {
                        app.search_input.pop();
                    }
                    KeyEvent {code: KeyCode::Esc, modifiers: KeyModifiers::NONE, kind: pressed, state: none} => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                }

                InputMode::EditingCode => match key {
                    KeyEvent {code: KeyCode::Enter, modifiers: KeyModifiers::NONE, kind: pressed, state: none} => {
                    }
                    KeyEvent {code: KeyCode::Char(c), modifiers: KeyModifiers::NONE, kind: pressed, state: none} => {
                       app.code_input.push(c);
                    }
                    KeyEvent {code: KeyCode::Backspace, modifiers: KeyModifiers::NONE, kind: pressed, state: none} => {
                        app.code_input.pop();
                    }
                    KeyEvent {code: KeyCode::Esc, modifiers: KeyModifiers::NONE, kind: pressed, state: none} => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                }
                _  => {}
            };

        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // Wrapping block for a group
    // Just draw the block and the group on the same area and build the group
    // with at least a margin of 1
    let size = f.size();

    // Surrounding block
    let block = Block::default();
    //    .title("Code Snippet Sync App")
    //    .title_alignment(Alignment::Center);
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

    let search_input = Paragraph::new(app.search_input.as_ref())
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
        .map(|(i, m)| {
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

    let code_input = Paragraph::new(app.code_input.as_ref())
        .block(Block::default()
            .borders(Borders::ALL)
            .title(app.input_mode.to_string())
            .title_alignment(Alignment::Center));
    f.render_widget(code_input, chunks[2]);

    match app.input_mode {
        InputMode::Normal => {}
        InputMode::EditingSearch => {
            f.set_cursor(
                left_chunks[0].x + app.search_input.width() as u16 + 1,
                left_chunks[0].y + 1
            )
        }
        InputMode::EditingCode => {
            f.set_cursor(
                chunks[2].x + app.code_input.width() as u16 + 1,
                chunks[2].y + 1
            )
        }
        _ => {}
    }

}
